//! Preliminary solar-geometry layer.
//!
//! This module implements a transparent engineering approximation for
//! declination and solar elevation. It is useful for unit tests and analytical
//! reasoning, but it is NOT the validated production solar-position algorithm.
//! Validated work will be checked against the NREL Solar Position Algorithm
//! (Reda & Andreas) before annual-yield claims are made.

use std::f64::consts::PI;

pub fn deg_to_rad(deg: f64) -> f64 { deg * PI / 180.0 }
pub fn rad_to_deg(rad: f64) -> f64 { rad * 180.0 / PI }

/// Cooper (1969) engineering approximation for solar declination.
///
/// Input: ordinal day n in [1,365]. Output: declination in radians.
/// Constants 23.45 deg, 365 and phase 284 belong to the published
/// approximation; they are not fitted to this project.
pub fn cooper_declination(day_of_year: u16) -> f64 {
    assert!((1..=365).contains(&day_of_year));
    let argument_deg = (360.0 / 365.0) * (284.0 + day_of_year as f64);
    deg_to_rad(23.45) * deg_to_rad(argument_deg).sin()
}

/// Solar hour angle from local apparent solar time.
///
/// Input hours are apparent solar time, not Singapore civil clock time.
/// 15 deg/hour = 360 deg / 24 h exactly under the solar-time definition.
pub fn hour_angle(apparent_solar_time_h: f64) -> f64 {
    deg_to_rad(15.0 * (apparent_solar_time_h - 12.0))
}

/// Geometric solar elevation from latitude, declination and hour angle.
///
/// All angular inputs and output are radians.
pub fn solar_elevation(latitude: f64, declination: f64, hour_angle: f64) -> f64 {
    let sin_alpha = latitude.sin() * declination.sin()
        + latitude.cos() * declination.cos() * hour_angle.cos();
    sin_alpha.clamp(-1.0, 1.0).asin()
}

/// Unit vector pointing from the observer toward the Sun in local ENU axes.
///
/// Convention:
/// - +x = East, +y = North, +z = Up;
/// - azimuth is measured clockwise from North;
/// - elevation is measured upward from the local horizon.
///
/// This constructor is deliberately independent of the preliminary Cooper
/// declination model so a validated solar-position algorithm can later supply
/// azimuth/elevation without changing downstream geometry code.
pub fn solar_direction_enu(azimuth: f64, elevation: f64) -> [f64; 3] {
    let cos_el = elevation.cos();
    [
        cos_el * azimuth.sin(),
        cos_el * azimuth.cos(),
        elevation.sin(),
    ]
}


/// Diagnostic irradiance closure residual for a horizontal plane.
///
/// The ideal geometric relation is GHI = DHI + DNI * cos(theta_z), where
/// theta_z is solar zenith angle. Inputs are irradiances in W/m^2 and zenith
/// angle in radians. This function is a QC diagnostic: it does not overwrite
/// measurements and it does not choose an acceptance tolerance.
///
/// Returns GHI - (DHI + DNI*cos(theta_z)) in W/m^2. For sun at/below the
/// horizon (cos(theta_z) <= 0), no direct horizontal contribution is admitted
/// and the diagnostic reduces to GHI - DHI.
pub fn irradiance_closure_residual_w_m2(
    ghi_w_m2: f64,
    dhi_w_m2: f64,
    dni_w_m2: f64,
    solar_zenith_rad: f64,
) -> f64 {
    let cos_zenith = solar_zenith_rad.cos().max(0.0);
    ghi_w_m2 - (dhi_w_m2 + dni_w_m2 * cos_zenith)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apparent_noon_has_zero_hour_angle() {
        assert!(hour_angle(12.0).abs() < 1e-15);
    }

    #[test]
    fn one_solar_hour_is_fifteen_degrees() {
        assert!((rad_to_deg(hour_angle(13.0)) - 15.0).abs() < 1e-12);
    }

    #[test]
    fn equator_equinox_noon_is_zenith() {
        let elevation = solar_elevation(0.0, 0.0, 0.0);
        assert!((rad_to_deg(elevation) - 90.0).abs() < 1e-12);
    }

    #[test]
    fn enu_cardinal_and_zenith_directions_match_convention() {
        let tol = 1e-12;
        let north = solar_direction_enu(0.0, 0.0);
        assert!((north[0] - 0.0).abs() < tol);
        assert!((north[1] - 1.0).abs() < tol);
        assert!((north[2] - 0.0).abs() < tol);

        let east = solar_direction_enu(deg_to_rad(90.0), 0.0);
        assert!((east[0] - 1.0).abs() < tol);
        assert!(east[1].abs() < tol);
        assert!(east[2].abs() < tol);

        let zenith = solar_direction_enu(0.0, deg_to_rad(90.0));
        assert!(zenith[0].abs() < tol);
        assert!(zenith[1].abs() < tol);
        assert!((zenith[2] - 1.0).abs() < tol);
    }

    #[test]
    fn enu_solar_direction_is_unit_length() {
        let tol = 1e-12;
        for az_deg in [0.0, 37.0, 90.0, 180.0, 271.0] {
            for el_deg in [-5.0, 0.0, 23.0, 67.0, 90.0] {
                let v = solar_direction_enu(deg_to_rad(az_deg), deg_to_rad(el_deg));
                let norm = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                assert!((norm - 1.0).abs() < tol);
            }
        }
    }

    #[test]
    fn irradiance_closure_matches_exact_synthetic_cases() {
        let tol = 1e-12;

        // Zenith Sun: direct horizontal contribution equals DNI.
        assert!(irradiance_closure_residual_w_m2(
            900.0, 100.0, 800.0, 0.0
        ).abs() < tol);

        // 60-degree zenith: cos(theta_z)=0.5.
        assert!(irradiance_closure_residual_w_m2(
            500.0, 100.0, 800.0, deg_to_rad(60.0)
        ).abs() < tol);

        // At the horizon there is no direct horizontal contribution.
        assert!(irradiance_closure_residual_w_m2(
            100.0, 100.0, 800.0, deg_to_rad(90.0)
        ).abs() < tol);

        // Below the horizon the diagnostic must not create a negative direct term.
        assert!(irradiance_closure_residual_w_m2(
            20.0, 20.0, 800.0, deg_to_rad(100.0)
        ).abs() < tol);
    }

    #[test]
    fn irradiance_closure_residual_preserves_mismatch_sign() {
        let residual = irradiance_closure_residual_w_m2(
            510.0, 100.0, 800.0, deg_to_rad(60.0)
        );
        assert!((residual - 10.0).abs() < 1e-12);
    }

    #[test]
    fn cooper_declination_stays_physically_bounded() {
        for day in 1..=365 {
            assert!(rad_to_deg(cooper_declination(day)).abs() <= 23.45 + 1e-12);
        }
    }
}
