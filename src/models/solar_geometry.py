"""Core first-principles geometry models for Singapore 3-D PV study."""
import numpy as np

SINGAPORE_LAT_DEG = 1.3521

def solar_declination(day):
    return np.deg2rad(23.45) * np.sin(2*np.pi*(284+day)/365.0)

def solar_vector(day, solar_hour, lat_deg=SINGAPORE_LAT_DEG):
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
    """A_PV/A_footprint for z=h(1-r^2/R^2), k=h/R."""
    if abs(k) < 1e-12:
        return 1.0
    return ((1+4*k*k)**1.5-1)/(6*k*k)

def isotropic_diffuse_multiplier(k):
    """Ideal diffuse collection / horizontal, no occlusion."""
    return 0.5*(paraboloid_area_ratio(k)+1.0)
