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
    fn cooper_declination_stays_physically_bounded() {
        for day in 1..=365 {
            assert!(rad_to_deg(cooper_declination(day)).abs() <= 23.45 + 1e-12);
        }
    }
}
