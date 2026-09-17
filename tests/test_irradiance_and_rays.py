import numpy as np
from src.models.irradiance import (IrradianceState, reconstruct_dni,
    incidence_cosine, isotropic_sky_factor, poa_isotropic)
from src.models.facets import Triangle, direct_visibility

def test_dni_reconstruction():
    assert abs(reconstruct_dni(800, 200, 0.75)-800) < 1e-12
    assert reconstruct_dni(100, 120, 0.5) == 0
    assert reconstruct_dni(500, 100, 0.05) == 0

def test_horizontal_normal_overhead_sun():
    assert abs(incidence_cosine([0,0,1], [0,0,1])-1) < 1e-12
    assert abs(isotropic_sky_factor([0,0,1])-1) < 1e-12

def test_vertical_isotropic_sky_factor():
    assert abs(isotropic_sky_factor([1,0,0])-0.5) < 1e-12

def test_poa_horizontal_identity():
    state = IrradianceState(ghi=800, dhi=200, dni=600)
    poa = poa_isotropic([0,0,1], [0,0,1], state, albedo=0.2)
    assert abs(poa['total']-800) < 1e-12

def test_blocked_and_unblocked_visibility():
    lower = Triangle([[0,0,0],[1,0,0],[0,1,0]])
    upper = Triangle([[0,0,1],[1,0,1],[0,1,1]])
    assert direct_visibility(0, [lower, upper], [0,0,1]) == 0
    assert direct_visibility(1, [lower, upper], [0,0,1]) == 1
