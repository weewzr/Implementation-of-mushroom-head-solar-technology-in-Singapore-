//! Geometry-agnostic module-temperature models.
//!
//! This module is deliberately independent of weather-provider ingestion and of
//! the frozen electrical kernel. Public field names encode units.
//! Celsius temperature differences are numerically equal to kelvin differences.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermalInput {
    /// Plane-of-array irradiance [W m^-2].
    pub poa_w_m2: f64,
    /// Ambient air temperature [degC].
    pub ambient_temp_c: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NmotParameters {
    /// Nominal module operating temperature [degC].
    pub nmot_c: f64,
    /// Reference ambient temperature for NMOT [degC].
    pub reference_ambient_c: f64,
    /// Reference irradiance for NMOT [W m^-2].
    pub reference_irradiance_w_m2: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FaimanParameters {
    /// Constant heat-loss coefficient [W m^-2 K^-1].
    pub u0_w_m2_k: f64,
    /// Wind-dependent heat-loss coefficient [W s m^-3 K^-1].
    pub u1_w_s_m3_k: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FaimanInput {
    pub poa_w_m2: f64,
    pub ambient_temp_c: f64,
    /// Wind speed [m s^-1]. Caller owns reference-height/local-flow provenance.
    pub wind_speed_m_s: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalError {
    NonFinite,
    NegativeIrradiance,
    NegativeWind,
    InvalidReferenceIrradiance,
    InvalidHeatLossCoefficient,
    NonPositiveHeatLossDenominator,
    NonFiniteOutput,
}

impl NmotParameters {
    /// Audit-63 baseline for Canadian Solar CS6.2-48TM-460H.
    /// Manufacturer NMOT: 42 +/- 3 degC at 800 W/m2, ambient 20 degC.
    pub fn canadian_solar_cs62_48tm_460h_nominal() -> Self {
        Self { nmot_c: 42.0, reference_ambient_c: 20.0, reference_irradiance_w_m2: 800.0 }
    }
    pub fn manufacturer_sensitivity() -> [Self; 3] {
        [39.0, 42.0, 45.0].map(|nmot_c| Self { nmot_c, ..Self::canadian_solar_cs62_48tm_460h_nominal() })
    }
    pub fn validate(&self) -> Result<(), ThermalError> {
        if !self.nmot_c.is_finite() || !self.reference_ambient_c.is_finite() || !self.reference_irradiance_w_m2.is_finite() {
            return Err(ThermalError::NonFinite);
        }
        if self.reference_irradiance_w_m2 <= 0.0 {
            return Err(ThermalError::InvalidReferenceIrradiance);
        }
        Ok(())
    }
}

pub fn module_temperature_nmot(input: ThermalInput, p: NmotParameters) -> Result<f64, ThermalError> {
    p.validate()?;
    if !input.poa_w_m2.is_finite() || !input.ambient_temp_c.is_finite() {
        return Err(ThermalError::NonFinite);
    }
    if input.poa_w_m2 < 0.0 { return Err(ThermalError::NegativeIrradiance); }
    let out = input.ambient_temp_c
        + (p.nmot_c - p.reference_ambient_c) * input.poa_w_m2 / p.reference_irradiance_w_m2;
    if !out.is_finite() { return Err(ThermalError::NonFiniteOutput); }
    Ok(out)
}

/// Wind-sensitive model-form sensitivity only. No default U0/U1 or wind source
/// is supplied because Audit 63 did not authorize a physical Faiman baseline.
pub fn module_temperature_faiman(input: FaimanInput, p: FaimanParameters) -> Result<f64, ThermalError> {
    if !input.poa_w_m2.is_finite() || !input.ambient_temp_c.is_finite() || !input.wind_speed_m_s.is_finite()
        || !p.u0_w_m2_k.is_finite() || !p.u1_w_s_m3_k.is_finite() { return Err(ThermalError::NonFinite); }
    if input.poa_w_m2 < 0.0 { return Err(ThermalError::NegativeIrradiance); }
    if input.wind_speed_m_s < 0.0 { return Err(ThermalError::NegativeWind); }
    if p.u0_w_m2_k <= 0.0 || p.u1_w_s_m3_k < 0.0 { return Err(ThermalError::InvalidHeatLossCoefficient); }
    let d = p.u0_w_m2_k + p.u1_w_s_m3_k * input.wind_speed_m_s;
    if !d.is_finite() || d <= 0.0 { return Err(ThermalError::NonPositiveHeatLossDenominator); }
    let out = input.ambient_temp_c + input.poa_w_m2 / d;
    if !out.is_finite() { return Err(ThermalError::NonFiniteOutput); }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn close(a:f64,b:f64){ assert!((a-b).abs() < 1e-10_f64.max(b.abs()*1e-12)); }
    fn n()->NmotParameters{NmotParameters::canadian_solar_cs62_48tm_460h_nominal()}
    #[test] fn nmot_zero_irradiance_equals_ambient(){close(module_temperature_nmot(ThermalInput{poa_w_m2:0.,ambient_temp_c:31.},n()).unwrap(),31.);}
    #[test] fn nmot_manufacturer_reference_condition(){close(module_temperature_nmot(ThermalInput{poa_w_m2:800.,ambient_temp_c:20.},n()).unwrap(),42.);}
    #[test] fn nmot_higher_irradiance_is_hotter(){let a=module_temperature_nmot(ThermalInput{poa_w_m2:300.,ambient_temp_c:30.},n()).unwrap();let b=module_temperature_nmot(ThermalInput{poa_w_m2:900.,ambient_temp_c:30.},n()).unwrap();assert!(b>a);}
    #[test] fn nmot_sensitivity_ordering(){let i=ThermalInput{poa_w_m2:700.,ambient_temp_c:30.};let p=NmotParameters::manufacturer_sensitivity();let t=p.map(|x|module_temperature_nmot(i,x).unwrap());assert!(t[2]>t[1]&&t[1]>t[0]);}
    #[test] fn nmot_hand_case(){close(module_temperature_nmot(ThermalInput{poa_w_m2:400.,ambient_temp_c:30.},n()).unwrap(),41.0);}
    #[test] fn nmot_invalids_rejected(){assert_eq!(module_temperature_nmot(ThermalInput{poa_w_m2:-1.,ambient_temp_c:20.},n()),Err(ThermalError::NegativeIrradiance));let mut p=n();p.reference_irradiance_w_m2=0.;assert_eq!(p.validate(),Err(ThermalError::InvalidReferenceIrradiance));p.reference_irradiance_w_m2=-1.;assert_eq!(p.validate(),Err(ThermalError::InvalidReferenceIrradiance));for x in [f64::NAN,f64::INFINITY]{assert_eq!(module_temperature_nmot(ThermalInput{poa_w_m2:x,ambient_temp_c:20.},n()),Err(ThermalError::NonFinite));}}
    #[test] fn nmot_deterministic(){let i=ThermalInput{poa_w_m2:623.,ambient_temp_c:32.};assert_eq!(module_temperature_nmot(i,n()),module_temperature_nmot(i,n()));}
    #[test] fn nmot_singapore_like_diagnostics_are_finite(){for ta in [25.,30.,35.]{for g in [0.,250.,500.,750.,1000.]{assert!(module_temperature_nmot(ThermalInput{poa_w_m2:g,ambient_temp_c:ta},n()).unwrap().is_finite());}}}
    #[test] fn faiman_limiting_and_monotonic_tests(){let p=FaimanParameters{u0_w_m2_k:25.,u1_w_s_m3_k:6.84};close(module_temperature_faiman(FaimanInput{poa_w_m2:0.,ambient_temp_c:30.,wind_speed_m_s:1.},p).unwrap(),30.);let low=module_temperature_faiman(FaimanInput{poa_w_m2:400.,ambient_temp_c:30.,wind_speed_m_s:1.},p).unwrap();let high=module_temperature_faiman(FaimanInput{poa_w_m2:800.,ambient_temp_c:30.,wind_speed_m_s:1.},p).unwrap();let windy=module_temperature_faiman(FaimanInput{poa_w_m2:800.,ambient_temp_c:30.,wind_speed_m_s:5.},p).unwrap();assert!(high>low&&windy<high);}
    #[test] fn faiman_requires_explicit_valid_inputs(){let p=FaimanParameters{u0_w_m2_k:25.,u1_w_s_m3_k:6.84};assert_eq!(module_temperature_faiman(FaimanInput{poa_w_m2:800.,ambient_temp_c:20.,wind_speed_m_s:-1.},p),Err(ThermalError::NegativeWind));assert_eq!(module_temperature_faiman(FaimanInput{poa_w_m2:-1.,ambient_temp_c:20.,wind_speed_m_s:1.},p),Err(ThermalError::NegativeIrradiance));assert_eq!(module_temperature_faiman(FaimanInput{poa_w_m2:800.,ambient_temp_c:20.,wind_speed_m_s:1.},FaimanParameters{u0_w_m2_k:0.,u1_w_s_m3_k:0.}),Err(ThermalError::InvalidHeatLossCoefficient));}
}
