"""Correctness tests for accelerated direct-beam geometry evaluation."""
import numpy as np
from src.models.facets import (
    Triangle,
    array_direct_power_factor,
    array_direct_power_factor_reference,
)
from src.models.meshes import paraboloid_mesh, accordion_mesh


def assert_matches(mesh, sun):
    reference = array_direct_power_factor_reference(mesh, sun)
    accelerated = array_direct_power_factor(mesh, sun)
    assert np.isclose(accelerated, reference, rtol=1e-10, atol=1e-12)


def test_stacked_triangles_match_reference():
    lower = Triangle(np.array([[0,0,0],[1,0,0],[0,1,0]], float))
    upper = Triangle(np.array([[0,0,1],[1,0,1],[0,1,1]], float))
    assert_matches([lower, upper], np.array([0.,0.,1.]))


def test_paraboloid_multiple_sun_directions_match_reference():
    mesh = paraboloid_mesh(2.0, n_r=3, n_phi=12)
    for sun in (
        np.array([0.,0.,1.]),
        np.array([0.7,0.,0.71414284]),
        np.array([-0.4,0.5,0.76811457]),
    ):
        assert_matches(mesh, sun)


def test_accordion_multiple_sun_directions_match_reference():
    mesh = accordion_mesh(2.0, folds=3)
    for sun in (
        np.array([0.,0.,1.]),
        np.array([0.8,0.,0.6]),
        np.array([-0.8,0.,0.6]),
    ):
        assert_matches(mesh, sun)
