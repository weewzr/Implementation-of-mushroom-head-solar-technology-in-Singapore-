//! Irradiance closure and baseline plane-of-array transposition.
//!
//! The isotropic-sky model here is a transparent validation baseline. It is not
//! evidence that Singapore's diffuse field is isotropic.

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PoaIrradiance {
    pub direct_w_m2: f64,
    pub sky_diffuse_w_m2: f64,
    pub ground_diffuse_w_m2: f64,
    pub global_w_m2: f64,
}

pub fn irradiance_closure_residual_w_m2(
    ghi_w_m2: f64, dhi_w_m2: f64, dni_w_m2: f64, zenith_rad: f64
) -> f64 {
    ghi_w_m2 - (dhi_w_m2 + dni_w_m2 * zenith_rad.cos().max(0.0))
}

/// Cosine of incidence for north-clockwise azimuth convention.
pub fn cosine_of_incidence(
    surface_tilt_rad: f64,
    surface_azimuth_rad: f64,
    solar_zenith_rad: f64,
    solar_azimuth_rad: f64,
) -> f64 {
    solar_zenith_rad.cos() * surface_tilt_rad.cos()
        + solar_zenith_rad.sin() * surface_tilt_rad.sin()
            * (solar_azimuth_rad - surface_azimuth_rad).cos()
}

/// Isotropic-sky POA decomposition, matching the standard component definitions
/// used by pvlib's isotropic total-irradiance path.
pub fn isotropic_poa(
    surface_tilt_rad: f64,
    surface_azimuth_rad: f64,
    solar_zenith_rad: f64,
    solar_azimuth_rad: f64,
    dni_w_m2: f64,
    ghi_w_m2: f64,
    dhi_w_m2: f64,
    ground_albedo: f64,
) -> Result<PoaIrradiance, String> {
    if !(0.0..=PI).contains(&surface_tilt_rad) { return Err("surface tilt must be in [0, pi]".into()); }
    if !(0.0..=1.0).contains(&ground_albedo) { return Err("ground albedo must be in [0, 1]".into()); }
    for (name, x) in [("DNI", dni_w_m2), ("GHI", ghi_w_m2), ("DHI", dhi_w_m2)] {
        if !x.is_finite() || x < 0.0 { return Err(format!("{name} must be finite and non-negative")); }
    }
    let cos_beta = surface_tilt_rad.cos();
    let direct = dni_w_m2 * cosine_of_incidence(
        surface_tilt_rad, surface_azimuth_rad, solar_zenith_rad, solar_azimuth_rad
    ).max(0.0);
    let sky = dhi_w_m2 * (1.0 + cos_beta) / 2.0;
    let ground = ghi_w_m2 * ground_albedo * (1.0 - cos_beta) / 2.0;
    Ok(PoaIrradiance {
        direct_w_m2: direct,
        sky_diffuse_w_m2: sky,
        ground_diffuse_w_m2: ground,
        global_w_m2: direct + sky + ground,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn d(x:f64)->f64{x*PI/180.0}
    const W_TOL:f64=1e-9;

    #[test]
    fn exact_ghi_dni_dhi_closure_cases() {
        assert!(irradiance_closure_residual_w_m2(900.0,100.0,800.0,d(0.0)).abs()<W_TOL);
        assert!(irradiance_closure_residual_w_m2(500.0,100.0,800.0,d(60.0)).abs()<W_TOL);
        assert!(irradiance_closure_residual_w_m2(100.0,100.0,800.0,d(90.0)).abs()<W_TOL);
    }

    #[test]
    fn horizontal_consistent_input_returns_ghi() {
        let p=isotropic_poa(d(0.0),d(180.0),d(60.0),d(180.0),800.0,500.0,100.0,0.25).unwrap();
        assert!((p.global_w_m2-500.0).abs()<W_TOL);
        assert!(p.ground_diffuse_w_m2.abs()<W_TOL);
    }

    #[test]
    fn vertical_surface_sun_normal_has_expected_components() {
        let p=isotropic_poa(d(90.0),d(180.0),d(90.0),d(180.0),800.0,500.0,100.0,0.20).unwrap();
        assert!((p.direct_w_m2-800.0).abs()<W_TOL);
        assert!((p.sky_diffuse_w_m2-50.0).abs()<W_TOL);
        assert!((p.ground_diffuse_w_m2-50.0).abs()<W_TOL);
        assert!((p.global_w_m2-900.0).abs()<W_TOL);
    }

    #[test]
    fn rear_facing_beam_is_clipped() {
        let p=isotropic_poa(d(90.0),d(0.0),d(90.0),d(180.0),800.0,500.0,100.0,0.20).unwrap();
        assert!(p.direct_w_m2.abs()<W_TOL);
    }
}
