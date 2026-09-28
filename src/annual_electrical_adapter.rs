//! Strict timestamp/resource adapter for future annual electrical coupling.
//! It aligns already-normalized irradiance rows with canonical weather records;
//! it does not compute irradiance, thermal physics, or electrical physics.

use std::collections::HashMap;
use crate::electrical::ElectricalDesignBasis;
use crate::thermal::NmotParameters;
use crate::thermal_electrical::{evaluate_coupled_step,CoupledInput,CoupledOutput,CouplingError};
use crate::weather::WeatherRecord;

#[derive(Debug,Clone,PartialEq)]
pub struct AnnualIrradianceRow {
 pub timestamp_utc_s:i64, pub geometry:String, pub resource_contract:String,
 pub poa_w_m2:f64, pub accepted:bool, pub irradiance_provenance:String,
}
#[derive(Debug,Clone,PartialEq)]
pub struct AnnualAdapterConfig {
 pub nmot:NmotParameters, pub nmot_scenario_c:f64, pub dt_hours:f64,
 pub auxiliary_power_w:f64, pub design_basis_id:String, pub status:String,
}
#[derive(Debug,Clone,PartialEq)]
pub struct AnnualElectricalOutputRow {
 pub timestamp_utc_s:i64,pub geometry:String,pub resource_contract:String,
 pub poa_w_m2:f64,pub ambient_temp_c:f64,pub nmot_c:f64,pub module_temp_c:f64,
 pub efficiency:f64,pub delivered_dc_w:f64,pub ac_w:f64,pub auxiliary_power_w:f64,
 pub net_w:f64,pub dt_hours:f64,pub net_energy_wh:f64,pub design_basis_id:String,
 pub irradiance_provenance:String,pub weather_provenance:String,pub status:String,
}
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum AnnualAdapterError {
 DuplicateIrradianceTimestamp(i64),DuplicateWeatherTimestamp(i64),MissingWeatherTimestamp(i64),
 NonMonotonicIrradiance(i64),NonMonotonicWeather(i64),UnexpectedInterval{previous:i64,current:i64,expected_s:i64},
 RejectedIrradianceRow(i64),InvalidConfig,Coupling(CouplingError),
}
impl From<CouplingError> for AnnualAdapterError{fn from(e:CouplingError)->Self{Self::Coupling(e)}}

fn validate_strict_times(xs:&[i64],expected_s:i64,weather:bool)->Result<(),AnnualAdapterError>{
 if expected_s<=0{return Err(AnnualAdapterError::InvalidConfig)}
 for w in xs.windows(2){if w[1]==w[0]{return Err(if weather{AnnualAdapterError::DuplicateWeatherTimestamp(w[1])}else{AnnualAdapterError::DuplicateIrradianceTimestamp(w[1])})}
  if w[1]<w[0]{return Err(if weather{AnnualAdapterError::NonMonotonicWeather(w[1])}else{AnnualAdapterError::NonMonotonicIrradiance(w[1])})}
  if w[1]-w[0]!=expected_s{return Err(AnnualAdapterError::UnexpectedInterval{previous:w[0],current:w[1],expected_s})}}
 Ok(())
}
pub fn join_and_evaluate(rows:&[AnnualIrradianceRow],weather:&[WeatherRecord],cfg:&AnnualAdapterConfig,e:&ElectricalDesignBasis,weather_provenance:&str)->Result<Vec<AnnualElectricalOutputRow>,AnnualAdapterError>{
 if !cfg.dt_hours.is_finite()||cfg.dt_hours<=0.||!cfg.auxiliary_power_w.is_finite()||cfg.auxiliary_power_w<0.||cfg.design_basis_id.is_empty()||cfg.status.is_empty(){return Err(AnnualAdapterError::InvalidConfig)}
 let expected_s=(cfg.dt_hours*3600.).round() as i64;if ((expected_s as f64)/3600.-cfg.dt_hours).abs()>1e-12{return Err(AnnualAdapterError::InvalidConfig)}
 validate_strict_times(&rows.iter().map(|x|x.timestamp_utc_s).collect::<Vec<_>>(),expected_s,false)?;
 validate_strict_times(&weather.iter().map(|x|x.timestamp_utc_s).collect::<Vec<_>>(),expected_s,true)?;
 let mut wm=HashMap::new();for w in weather{if wm.insert(w.timestamp_utc_s,w).is_some(){return Err(AnnualAdapterError::DuplicateWeatherTimestamp(w.timestamp_utc_s));}}
 let mut out=Vec::with_capacity(rows.len());
 for r in rows{if !r.accepted{return Err(AnnualAdapterError::RejectedIrradianceRow(r.timestamp_utc_s))}let w=wm.get(&r.timestamp_utc_s).ok_or(AnnualAdapterError::MissingWeatherTimestamp(r.timestamp_utc_s))?;
  let x:CoupledOutput=evaluate_coupled_step(CoupledInput{poa_w_m2:r.poa_w_m2,ambient_temp_c:w.ambient_temperature_c,dt_hours:cfg.dt_hours,auxiliary_power_w:cfg.auxiliary_power_w},cfg.nmot,e)?;
  out.push(AnnualElectricalOutputRow{timestamp_utc_s:r.timestamp_utc_s,geometry:r.geometry.clone(),resource_contract:r.resource_contract.clone(),poa_w_m2:r.poa_w_m2,ambient_temp_c:w.ambient_temperature_c,nmot_c:cfg.nmot_scenario_c,module_temp_c:x.module_temp_c,efficiency:x.efficiency,delivered_dc_w:x.delivered_dc_w,ac_w:x.ac_w,auxiliary_power_w:x.auxiliary_power_w,net_w:x.net_w,dt_hours:cfg.dt_hours,net_energy_wh:x.net_energy_wh,design_basis_id:cfg.design_basis_id.clone(),irradiance_provenance:r.irradiance_provenance.clone(),weather_provenance:weather_provenance.into(),status:cfg.status.clone()});}
 Ok(out)
}
pub fn wh_to_kwh(wh:f64)->f64{wh/1000.0}

#[cfg(test)]
mod tests{
 use super::*;use crate::weather::WeatherRecord;
 fn wr(t:i64,ta:f64)->WeatherRecord{WeatherRecord{timestamp:format!("{t}"),timestamp_utc_s:t,ghi_w_m2:0.,dhi_w_m2:0.,dni_w_m2:0.,ambient_temperature_c:ta,wind_speed_m_s:0.,wind_direction_deg:0.,relative_humidity_percent:None,air_pressure_pa:None,provider_quality_flag:None}}
 fn ir(t:i64,g:f64)->AnnualIrradianceRow{AnnualIrradianceRow{timestamp_utc_s:t,geometry:"flat_reference".into(),resource_contract:"equal_land".into(),poa_w_m2:g,accepted:true,irradiance_provenance:"synthetic".into()}}
 fn cfg(n:f64)->AnnualAdapterConfig{AnnualAdapterConfig{nmot:NmotParameters{nmot_c:n,reference_ambient_c:20.,reference_irradiance_w_m2:800.},nmot_scenario_c:n,dt_hours:1.,auxiliary_power_w:0.,design_basis_id:"CS6.2-48TM-460H".into(),status:"DEVELOPMENT_NOT_SERIS_SYNTHETIC_ADAPTER_VALIDATION".into()}}
 fn e()->ElectricalDesignBasis{ElectricalDesignBasis::canadian_solar_cs62_48tm_460h()}
 #[test]fn perfect_join_and_identity_preservation(){let r=[ir(0,0.),ir(3600,400.),ir(7200,800.)];let w=[wr(0,27.),wr(3600,30.),wr(7200,31.)];let o=join_and_evaluate(&r,&w,&cfg(42.),&e(),"synthetic_weather").unwrap();assert_eq!(o.len(),3);assert_eq!(o[1].timestamp_utc_s,3600);assert_eq!(o[1].geometry,"flat_reference");assert_eq!(o[1].resource_contract,"equal_land");assert_eq!(o[0].module_temp_c,27.);assert_eq!(o[0].ac_w,0.);assert_eq!(o[1].net_energy_wh,o[1].net_w);}
 #[test]fn missing_weather_fails(){let r=[ir(0,0.),ir(3600,400.),ir(7200,800.)];let w=[wr(0,27.),wr(7200,31.)];assert!(join_and_evaluate(&r,&w,&cfg(42.),&e(),"w").is_err());}
 #[test]fn duplicate_weather_fails(){let r=[ir(0,0.),ir(3600,400.)];let w=[wr(0,27.),wr(0,28.)];assert!(matches!(join_and_evaluate(&r,&w,&cfg(42.),&e(),"w"),Err(AnnualAdapterError::DuplicateWeatherTimestamp(0))));}
 #[test]fn duplicate_irradiance_fails(){let r=[ir(0,0.),ir(0,400.)];let w=[wr(0,27.),wr(3600,28.)];assert!(matches!(join_and_evaluate(&r,&w,&cfg(42.),&e(),"w"),Err(AnnualAdapterError::DuplicateIrradianceTimestamp(0))));}
 #[test]fn one_hour_offset_fails_not_shifted(){let r=[ir(0,0.),ir(3600,400.),ir(7200,800.)];let w=[wr(3600,27.),wr(7200,30.),wr(10800,31.)];assert!(matches!(join_and_evaluate(&r,&w,&cfg(42.),&e(),"w"),Err(AnnualAdapterError::MissingWeatherTimestamp(0))));}
 #[test]fn nonmonotonic_fails(){let r=[ir(3600,400.),ir(0,0.)];let w=[wr(0,27.),wr(3600,30.)];assert!(matches!(join_and_evaluate(&r,&w,&cfg(42.),&e(),"w"),Err(AnnualAdapterError::NonMonotonicIrradiance(0))));}
 #[test]fn nmot_sensitivity_orders_power(){let r=[ir(0,700.)];let w=[wr(0,30.)];let a=join_and_evaluate(&r,&w,&cfg(39.),&e(),"w").unwrap();let b=join_and_evaluate(&r,&w,&cfg(42.),&e(),"w").unwrap();let c=join_and_evaluate(&r,&w,&cfg(45.),&e(),"w").unwrap();assert!(a[0].module_temp_c<b[0].module_temp_c&&b[0].module_temp_c<c[0].module_temp_c);assert!(a[0].ac_w>b[0].ac_w&&b[0].ac_w>c[0].ac_w);}
 #[test]fn unit_conversion_explicit(){assert_eq!(1.0*1.0,1.0);assert_eq!(wh_to_kwh(1000.0*1.0),1.0);}
 #[test]fn rejected_row_fails(){let mut r=ir(0,1.);r.accepted=false;assert!(matches!(join_and_evaluate(&[r],&[wr(0,20.)],&cfg(42.),&e(),"w"),Err(AnnualAdapterError::RejectedIrradianceRow(0))));}
}