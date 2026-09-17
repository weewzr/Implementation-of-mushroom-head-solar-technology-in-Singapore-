"""Triangular-facet geometry and direct-beam visibility.

The original implementation used nested Python loops over receiving facets and
blocking triangles. That reference path is retained for verification. The
production direct-beam path vectorises Moller--Trumbore intersections across
all potential blockers for each receiving facet, substantially reducing Python
overhead while preserving the same geometric model.
"""
from __future__ import annotations
from dataclasses import dataclass
import numpy as np

# Numerical tolerance (class N). This is not a physical length scale. A future
# refinement should make tolerance scale-aware and include it in convergence.
EPS = 1e-9
RAY_OFFSET_MULTIPLIER = 100.0

@dataclass
class Triangle:
    vertices: np.ndarray

    def __post_init__(self):
        self.vertices = np.asarray(self.vertices, dtype=float).reshape(3, 3)

    @property
    def centroid(self):
        return self.vertices.mean(axis=0)

    @property
    def normal(self):
        e1 = self.vertices[1]-self.vertices[0]
        e2 = self.vertices[2]-self.vertices[0]
        n = np.cross(e1, e2)
        return n/np.linalg.norm(n)

    @property
    def area(self):
        return 0.5*np.linalg.norm(np.cross(self.vertices[1]-self.vertices[0],
                                           self.vertices[2]-self.vertices[0]))


def ray_triangle_distance(origin, direction, triangle: Triangle):
    """Reference scalar Moller--Trumbore intersection distance."""
    o = np.asarray(origin, dtype=float)
    d = np.asarray(direction, dtype=float)
    d /= np.linalg.norm(d)
    v0, v1, v2 = triangle.vertices
    e1, e2 = v1-v0, v2-v0
    h = np.cross(d, e2)
    a = np.dot(e1, h)
    if abs(a) < EPS:
        return None
    f = 1.0/a
    q0 = o-v0
    u = f*np.dot(q0, h)
    if u < 0.0 or u > 1.0:
        return None
    q = np.cross(q0, e1)
    v = f*np.dot(d, q)
    if v < 0.0 or u+v > 1.0:
        return None
    t = f*np.dot(e2, q)
    return float(t) if t > EPS else None


def direct_visibility(index: int, triangles, sun_vector) -> float:
    """Reference scalar visibility: 1 if centroid sees Sun, otherwise 0."""
    tri = triangles[index]
    s = np.asarray(sun_vector, dtype=float)
    s /= np.linalg.norm(s)
    origin = tri.centroid + EPS*RAY_OFFSET_MULTIPLIER*s
    for j, blocker in enumerate(triangles):
        if j == index:
            continue
        if ray_triangle_distance(origin, s, blocker) is not None:
            return 0.0
    return 1.0


def mesh_arrays(triangles):
    """Convert Triangle objects to contiguous arrays used repeatedly by rays."""
    vertices = np.asarray([t.vertices for t in triangles], dtype=float)
    v0 = vertices[:, 0]
    e1 = vertices[:, 1]-v0
    e2 = vertices[:, 2]-v0
    cross = np.cross(e1, e2)
    twice_area = np.linalg.norm(cross, axis=1)
    normals = cross/twice_area[:, None]
    areas = 0.5*twice_area
    centroids = vertices.mean(axis=1)
    return v0, e1, e2, normals, areas, centroids


def _blocked_vectorised(origin, direction, receiver_index, v0, e1, e2):
    """Test one ray against all blockers using vectorised Moller--Trumbore.

    `origin` has geometry-coordinate units; `direction` is dimensionless;
    intersection parameter t therefore has geometry-coordinate units.
    """
    d = np.asarray(direction, dtype=float)
    d /= np.linalg.norm(d)
    h = np.cross(np.broadcast_to(d, e2.shape), e2)
    a = np.einsum('ij,ij->i', e1, h)
    valid = np.abs(a) >= EPS
    valid[receiver_index] = False
    if not np.any(valid):
        return False

    f = np.zeros_like(a)
    f[valid] = 1.0/a[valid]
    q0 = origin-v0
    u = f*np.einsum('ij,ij->i', q0, h)
    valid &= (u >= 0.0) & (u <= 1.0)
    if not np.any(valid):
        return False

    q = np.cross(q0, e1)
    v = f*np.einsum('j,ij->i', d, q)
    valid &= (v >= 0.0) & ((u+v) <= 1.0)
    if not np.any(valid):
        return False

    t = f*np.einsum('ij,ij->i', e2, q)
    valid &= t > EPS
    return bool(np.any(valid))


def array_direct_power_factor_reference(triangles, sun_vector):
    """Original scalar algorithm, retained as a correctness oracle."""
    s = np.asarray(sun_vector, dtype=float)
    s /= np.linalg.norm(s)
    total = 0.0
    for i, tri in enumerate(triangles):
        mu = max(0.0, float(np.dot(tri.normal, s)))
        if mu > 0.0:
            total += tri.area*mu*direct_visibility(i, triangles, s)
    return total


def array_direct_power_factor(triangles, sun_vector):
    """Vectorised-blocker geometric direct-beam projected area.

    Returns
    -------
    float
        Sum A_i V_i max(n_i dot s, 0), in square metres when mesh coordinates
        are metres. This is a geometric projected-area quantity, not power.
    """
    s = np.asarray(sun_vector, dtype=float)
    s /= np.linalg.norm(s)
    v0, e1, e2, normals, areas, centroids = mesh_arrays(triangles)
    mu = normals@s
    candidates = np.flatnonzero(mu > 0.0)
    total = 0.0
    offset = EPS*RAY_OFFSET_MULTIPLIER*s
    for i in candidates:
        origin = centroids[i]+offset
        if not _blocked_vectorised(origin, s, i, v0, e1, e2):
            total += areas[i]*mu[i]
    return float(total)
