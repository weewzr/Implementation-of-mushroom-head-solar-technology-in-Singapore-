//! Analytical geometry models.
//!
//! Implements the paraboloidal-cap packing relation derived in the technical
//! report. No empirical constants are used in this module.

use std::f64::consts::PI;

/// Active surface area of the paraboloidal cap
/// z(r) = h(1-r^2/R^2).
///
/// Units: if radius and height are in metres, the result is square metres.
/// The h=0 limit is handled exactly as a flat disk.
pub fn paraboloid_area(radius: f64, height: f64) -> f64 {
    assert!(radius > 0.0, "radius must be positive");
    assert!(height >= 0.0, "height must be non-negative");
    if height == 0.0 {
        return PI * radius * radius;
    }
    let k = height / radius;
    let x = 4.0 * k * k;
    // Evaluate (1+x)^(3/2)-1 without catastrophic cancellation for shallow caps.
    // ln_1p/expm1 preserve the small increment as k -> 0.
    let numerator = (1.5 * x.ln_1p()).exp_m1();
    PI * radius * radius * numerator / (6.0 * k * k)
}

/// PV active-area / horizontal-footprint packing ratio.
pub fn paraboloid_packing_ratio(radius: f64, height: f64) -> f64 {
    paraboloid_area(radius, height) / (PI * radius * radius)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_limit_is_one() {
        assert!((paraboloid_packing_ratio(1.0, 0.0) - 1.0).abs() < 1e-14);
    }

    #[test]
    fn shallow_cap_approaches_flat_disk() {
        let p = paraboloid_packing_ratio(1.0, 1e-5);
        assert!((p - 1.0).abs() < 1e-8);
    }

    #[test]
    fn scaling_radius_and_height_preserves_packing_ratio() {
        let a = paraboloid_packing_ratio(1.0, 0.5);
        let b = paraboloid_packing_ratio(2.0, 1.0);
        assert!((a - b).abs() < 1e-12);
    }

    #[test]
    fn curved_cap_has_more_area_than_footprint() {
        assert!(paraboloid_packing_ratio(1.0, 0.5) > 1.0);
    }
}
