"""Sanity tests for the semi-analytical paraboloid direct-beam solver.

Tolerances here are category-5 computational settings, not physical constants.
They test exact limiting identities to floating-point precision; convergence of
production quadrature must be documented separately before validated results.
"""
import math

from src.models.paraboloid_analytic import (
    absolute_gain,
    directional_response,
    horizontal_response,
    pv_area_ratio,
)

ABS_TOL = 1.0e-10


def test_flat_limit_matches_horizontal_cosine():
    for degrees in (0.0, 15.0, 30.0, 45.0, 60.0, 75.0, 90.0):
        theta = math.radians(degrees)
        assert math.isclose(
            directional_response(theta, h=0.0),
            horizontal_response(theta),
            rel_tol=0.0,
            abs_tol=ABS_TOL,
        )


def test_zenith_response_is_unity_for_multiple_shapes():
    for h in (0.0, 0.1, 0.25, 0.5, 1.0, 2.0):
        assert math.isclose(
            directional_response(0.0, h), 1.0, rel_tol=0.0, abs_tol=ABS_TOL
        )


def test_flat_pv_area_ratio_is_unity():
    assert pv_area_ratio(0.0) == 1.0


def test_response_is_scale_free_by_construction():
    # R does not enter the dimensionless solver. This test documents that the
    # public API depends only on theta_z, h and the numerical resolution.
    theta = math.radians(60.0)
    value = directional_response(theta, h=0.5)
    assert math.isfinite(value)
    assert value >= 0.0


def test_absolute_gain_is_zero_at_zenith():
    for h in (0.0, 0.25, 0.5, 1.0):
        assert math.isclose(
            absolute_gain(0.0, h), 0.0, rel_tol=0.0, abs_tol=ABS_TOL
        )


def test_radial_quadrature_converges_for_representative_case():
    theta = math.radians(70.0)
    h = 0.5
    c256 = directional_response(theta, h, n_radial=256)
    c512 = directional_response(theta, h, n_radial=512)
    c1024 = directional_response(theta, h, n_radial=1024)
    # This is deliberately a monotonic error-reduction check rather than a
    # claimed universal acceptance threshold.
    assert abs(c1024 - c512) < abs(c512 - c256)
