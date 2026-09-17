"""Analytical candidate geometries for equal-footprint PV packing comparisons."""
from __future__ import annotations
import math
from dataclasses import dataclass

@dataclass(frozen=True)
class GeometryPoint:
    name: str
    packing_ratio: float
    characteristic_tilt_deg: float
    height_over_width: float
    notes: str

def accordion_from_packing(pi: float) -> GeometryPoint:
    """Symmetric roof/accordion facets: Pi=sec(beta)."""
    if pi < 1:
        raise ValueError("packing ratio must be >= 1")
    beta = math.degrees(math.acos(1.0/pi))
    return GeometryPoint("accordion", pi, beta, 0.5*math.tan(math.radians(beta)),
                         "Ideal geometric fold before row-to-row shading.")

def cone_from_packing(pi: float) -> GeometryPoint:
    """Cone with footprint radius R: Pi=sqrt(1+(h/R)^2)."""
    if pi < 1:
        raise ValueError("packing ratio must be >= 1")
    h_over_r = math.sqrt(pi*pi-1.0)
    beta = math.degrees(math.atan(h_over_r))
    return GeometryPoint("cone", pi, beta, h_over_r/2.0,
                         "Radially symmetric constant-slope surface.")

def paraboloid_area_ratio(k: float) -> float:
    if abs(k) < 1e-12:
        return 1.0
    return ((1+4*k*k)**1.5-1)/(6*k*k)

def paraboloid_k_from_packing(pi: float, tol: float=1e-10) -> float:
    """Invert paraboloid area ratio by monotonic bisection."""
    if pi < 1:
        raise ValueError("packing ratio must be >= 1")
    if abs(pi-1) < tol:
        return 0.0
    lo, hi = 0.0, 1.0
    while paraboloid_area_ratio(hi) < pi:
        hi *= 2
    for _ in range(100):
        mid = 0.5*(lo+hi)
        if paraboloid_area_ratio(mid) < pi:
            lo = mid
        else:
            hi = mid
    return 0.5*(lo+hi)

def paraboloid_from_packing(pi: float) -> GeometryPoint:
    k = paraboloid_k_from_packing(pi)
    edge_tilt = math.degrees(math.atan(2*k))
    # Circular footprint of area one has diameter 2/sqrt(pi); h/diameter=k/2.
    return GeometryPoint("paraboloid", pi, edge_tilt, k/2.0,
                         "Tilt varies continuously from 0 at centre to edge tilt.")
