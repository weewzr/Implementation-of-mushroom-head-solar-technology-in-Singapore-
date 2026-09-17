"""Sanity tests for analytical geometry relations."""
import math
from src.models.candidate_geometries import (
    accordion_from_packing, cone_from_packing,
    paraboloid_area_ratio, paraboloid_k_from_packing,
)

def test_flat_limits():
    assert abs(paraboloid_area_ratio(0)-1) < 1e-12
    assert abs(accordion_from_packing(1).characteristic_tilt_deg) < 1e-12
    assert abs(cone_from_packing(1).characteristic_tilt_deg) < 1e-12

def test_accordion_pi_two_is_sixty_deg():
    assert abs(accordion_from_packing(2).characteristic_tilt_deg-60) < 1e-10

def test_cone_pi_two():
    g=cone_from_packing(2)
    assert abs(g.characteristic_tilt_deg-60) < 1e-10

def test_paraboloid_inverse():
    for pi in (1.25,1.5,2,3,4):
        k=paraboloid_k_from_packing(pi)
        assert abs(paraboloid_area_ratio(k)-pi) < 1e-8
