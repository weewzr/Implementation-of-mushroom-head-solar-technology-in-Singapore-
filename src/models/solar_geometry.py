"""Core first-principles geometry models for Singapore 3-D PV study.

The module intentionally separates analytical geometry from weather/yield modelling.
Equations correspond to the technical report and equations/derivations.md.
"""
import math
import numpy as np

SINGAPORE_LAT_DEG = 1.3521


def solar_declination(day):
    """Approximate solar declination [rad] for day-of-year."""
    return np.deg2rad(23.45) * np.sin(2*np.pi*(284+day)/365.0)


def solar_vector(day, solar_hour, lat_deg=SINGAPORE_LAT_DEG):
    """Unit vector toward sun in the model's local Cartesian convention."""
    lat = np.deg2rad(lat_deg)
    dec = solar_declination(day)
    H = np.deg2rad(15*(solar_hour-12))
    s = np.array([
        np.cos(dec)*np.sin(H),
        np.cos(lat)*np.sin(dec)-np.sin(lat)*np.cos(dec)*np.cos(H),
        np.sin(lat)*np.sin(dec)+np.cos(lat)*np.cos(dec)*np.cos(H),
    ])
    return s/np.linalg.norm(s)


def paraboloid_area_ratio(k):
    """Return A_PV/A_foot for z=h(1-r^2/R^2), k=h/R."""
    if abs(k) < 1e-12:
        return 1.0
    return ((1+4*k*k)**1.5-1)/(6*k*k)


def paraboloid_area(radius_m, height_m):
    """PV surface area [m^2] of the paraboloidal cap."""
    if radius_m <= 0:
        raise ValueError("radius_m must be positive")
    return math.pi * radius_m**2 * paraboloid_area_ratio(height_m/radius_m)


def isotropic_diffuse_multiplier(k):
    """Ideal diffuse collection / horizontal for an unoccluded cap."""
    return 0.5*(paraboloid_area_ratio(k)+1.0)


def land_multiplication(packing_ratio, packing_efficiency):
    """Return M_L = Pi * eta_pack."""
    return packing_ratio * packing_efficiency
