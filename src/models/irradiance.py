"""Time-resolved irradiance primitives.

The module intentionally separates measured/validated weather input from geometry.
No annual diffuse fraction is hard-coded.  Users provide GHI/DHI/DNI for each
instant; if DNI is unavailable it may be reconstructed only when solar zenith
is sufficiently far from the horizon.
"""
from __future__ import annotations
from dataclasses import dataclass
import numpy as np

@dataclass(frozen=True)
class IrradianceState:
    ghi: float
    dhi: float
    dni: float


def reconstruct_dni(ghi: float, dhi: float, cos_zenith: float,
                    min_cos_zenith: float = 0.1) -> float:
    """DNI=(GHI-DHI)/cos(zenith), guarded near the horizon."""
    if cos_zenith <= min_cos_zenith:
        return 0.0
    return max(0.0, (ghi-dhi)/cos_zenith)


def incidence_cosine(normal, sun_vector) -> float:
    n = np.asarray(normal, dtype=float)
    s = np.asarray(sun_vector, dtype=float)
    n /= np.linalg.norm(n)
    s /= np.linalg.norm(s)
    return max(0.0, float(np.dot(n, s)))


def isotropic_sky_factor(normal) -> float:
    """Isotropic diffuse sky factor for an unobstructed planar front side."""
    n = np.asarray(normal, dtype=float)
    n /= np.linalg.norm(n)
    cos_beta = float(n[2])
    return 0.5*(1.0+cos_beta)


def ground_view_factor(normal) -> float:
    n = np.asarray(normal, dtype=float)
    n /= np.linalg.norm(n)
    cos_beta = float(n[2])
    return 0.5*(1.0-cos_beta)


def poa_isotropic(normal, sun_vector, state: IrradianceState,
                  albedo: float = 0.2, visibility: float = 1.0) -> dict:
    """First POA model: direct + isotropic sky + ground-reflected GHI.

    `visibility` applies only to the direct beam here. Future ray tracing will
    provide separate beam, sky-patch and ground visibility factors.
    """
    mu = incidence_cosine(normal, sun_vector)
    beam = state.dni * mu * float(visibility)
    diffuse = state.dhi * isotropic_sky_factor(normal)
    ground = state.ghi * albedo * ground_view_factor(normal)
    return {"beam": beam, "diffuse": diffuse, "ground": ground,
            "total": beam+diffuse+ground}


def bifacial_effective_poa(front_normal, sun_vector, state: IrradianceState,
                            bifaciality: float = 0.8, albedo: float = 0.2,
                            front_visibility: float = 1.0,
                            rear_visibility: float = 1.0) -> dict:
    """Two-sided first approximation; no mutual occlusion yet."""
    n = np.asarray(front_normal, dtype=float)
    front = poa_isotropic(n, sun_vector, state, albedo, front_visibility)
    rear = poa_isotropic(-n, sun_vector, state, albedo, rear_visibility)
    effective = front["total"] + bifaciality*rear["total"]
    return {"front": front, "rear": rear, "effective": effective}
