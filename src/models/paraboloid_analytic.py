"""Semi-analytical direct-beam solver for an isolated paraboloidal PV canopy.

Model status: analytical/exploratory cross-check, not validated Singapore yield.
Assumptions: axisymmetric graph surface, one-sided PV, no self-occlusion (V=1),
unit horizontal footprint normalization. Angles are radians.
"""
from __future__ import annotations

import math
from typing import Callable


def paraboloid_slope(rho: float, h: float) -> float:
    """Return F'(rho) for F(rho)=h(1-rho**2)."""
    if not 0.0 <= rho <= 1.0:
        raise ValueError("rho must lie in [0, 1]")
    if h < 0.0:
        raise ValueError("h=H/R must be non-negative")
    return -2.0 * h * rho


def azimuth_integral(theta_z: float, slope: float) -> float:
    """Exact azimuthal integral of positive direct-beam projection.

    Returns I = integral_0^(2*pi) [a-B*cos(phi)]_+ dphi, where
    a=cos(theta_z) and B=abs(slope)*sin(theta_z).
    """
    if not 0.0 <= theta_z <= 0.5 * math.pi:
        raise ValueError("theta_z must lie between zenith and horizon")
    a = math.cos(theta_z)
    b = abs(slope) * math.sin(theta_z)
    if b <= a:
        return 2.0 * math.pi * a
    # Partial front-side illumination. Clamp only for round-off safety.
    x = max(0.0, min(1.0, a / b))
    return 2.0 * a * (math.pi - math.acos(x)) + 2.0 * math.sqrt(max(0.0, b * b - a * a))


def _simpson(f: Callable[[float], float], n: int) -> float:
    """Composite Simpson quadrature on [0,1]; n must be positive and even."""
    if n <= 0 or n % 2:
        raise ValueError("n must be a positive even integer")
    dx = 1.0 / n
    total = f(0.0) + f(1.0)
    total += 4.0 * sum(f(i * dx) for i in range(1, n, 2))
    total += 2.0 * sum(f(i * dx) for i in range(2, n, 2))
    return total * dx / 3.0


def directional_response(theta_z: float, h: float, n_radial: int = 1024) -> float:
    """Return C=A_eff/(pi R^2) for the isolated paraboloid.

    n_radial=1024 is a category-5 computational default. It is intentionally
    exposed and must be justified by convergence before validated reporting.
    """
    if h < 0.0:
        raise ValueError("h=H/R must be non-negative")

    def integrand(rho: float) -> float:
        return rho * azimuth_integral(theta_z, paraboloid_slope(rho, h))

    return _simpson(integrand, n_radial) / math.pi


def horizontal_response(theta_z: float) -> float:
    """Equal-footprint horizontal direct-beam response."""
    if not 0.0 <= theta_z <= 0.5 * math.pi:
        raise ValueError("theta_z must lie between zenith and horizon")
    return max(0.0, math.cos(theta_z))


def absolute_gain(theta_z: float, h: float, n_radial: int = 1024) -> float:
    """Return Delta C = C_mushroom - C_horizontal."""
    return directional_response(theta_z, h, n_radial) - horizontal_response(theta_z)


def pv_area_ratio(h: float) -> float:
    """Return Gamma=A_PV/(pi R^2) for the paraboloid."""
    if h < 0.0:
        raise ValueError("h=H/R must be non-negative")
    if h == 0.0:
        return 1.0
    return ((1.0 + 4.0 * h * h) ** 1.5 - 1.0) / (6.0 * h * h)
