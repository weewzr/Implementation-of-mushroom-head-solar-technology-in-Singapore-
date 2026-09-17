"""Minimal triangular-facet geometry and direct-beam visibility.

This is the first self-shadowing layer. It is intentionally dependency-light
and uses the Moller-Trumbore ray/triangle intersection algorithm.
"""
from __future__ import annotations
from dataclasses import dataclass
import numpy as np

EPS = 1e-9

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
    """Return positive ray distance to triangle, or None if no intersection."""
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
    s = o-v0
    u = f*np.dot(s, h)
    if u < 0.0 or u > 1.0:
        return None
    q = np.cross(s, e1)
    v = f*np.dot(d, q)
    if v < 0.0 or u+v > 1.0:
        return None
    t = f*np.dot(e2, q)
    return float(t) if t > EPS else None


def direct_visibility(index: int, triangles, sun_vector) -> float:
    """1 if the selected facet centroid can see the sun, otherwise 0."""
    tri = triangles[index]
    s = np.asarray(sun_vector, dtype=float)
    s /= np.linalg.norm(s)
    # Offset start point to avoid self-intersection numerical noise.
    origin = tri.centroid + EPS*100*s
    for j, blocker in enumerate(triangles):
        if j == index:
            continue
        if ray_triangle_distance(origin, s, blocker) is not None:
            return 0.0
    return 1.0


def array_direct_power_factor(triangles, sun_vector):
    """Geometric sum A_i V_i max(n_i.s,0), units of projected area."""
    s = np.asarray(sun_vector, dtype=float)
    s /= np.linalg.norm(s)
    total = 0.0
    for i, tri in enumerate(triangles):
        mu = max(0.0, float(np.dot(tri.normal, s)))
        if mu > 0:
            total += tri.area*mu*direct_visibility(i, triangles, s)
    return total
