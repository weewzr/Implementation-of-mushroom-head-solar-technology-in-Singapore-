//! Deterministic orchestration between the frozen NMOT thermal foundation and
//! frozen electrical conversion equations. No weather-provider logic lives here.

use crate::electrical::{evaluate_step_at_module_temperature, ElectricalDesignBasis, ElectricalError, StepInput, StepOutput};
use crate::thermal::{module_temperature_nmot, NmotParameters, ThermalError, ThermalInput};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoupledInput {
    pub poa_w_m2: f64,
    pub ambient_temp_c: f64,
    pub dt_hours: f64,
    pub auxiliary_power_w: f64,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoupledOutput {
    pub module_temp_c: f64,
    pub efficiency: f64,
    pub ideal_dc_w: f64,
    pub delivered_dc_w: f64,
    pub ac_w: f64,
    pub auxiliary_power_w: f64,
    pub net_w: f64,
    pub ac_energy_wh: f64,
    pub net_energy_wh: f64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CouplingError { Thermal(ThermalError), Electrical(ElectricalError) }
impl From<ThermalError> for CouplingError { fn from(e:ThermalError)->Self{Self::Thermal(e)} }
impl From<ElectricalError> for CouplingError { fn from(e:ElectricalError)->Self{Self::Electrical(e)} }

pub fn evaluate_coupled_step(i:CoupledInput,t:NmotParameters,e:&ElectricalDesignBasis)->Result<CoupledOutput,CouplingError>{
    let tm=module_temperature_nmot(ThermalInput{poa_w_m2:i.poa_w_m2,ambient_temp_c:i.ambient_temp_c},t)?;
    let o:StepOutput=evaluate_step_at_module_temperature(StepInput{poa_w_m2:i.poa_w_m2,ambient_temp_c:i.ambient_temp_c,dt_hours:i.dt_hours,auxiliary_power_w:i.auxiliary_power_w},tm,e)?;
    Ok(CoupledOutput{module_temp_c:tm,efficiency:o.efficiency,ideal_dc_w:o.ideal_dc_w,delivered_dc_w:o.delivered_dc_w,ac_w:o.ac_w,auxiliary_power_w:i.auxiliary_power_w,net_w:o.net_w,ac_energy_wh:o.ac_energy_wh,net_energy_wh:o.net_energy_wh})
}
pub fn evaluate_coupled_series(inputs:&[CoupledInput],t:NmotParameters,e:&ElectricalDesignBasis)->Result<Vec<CoupledOutput>,CouplingError>{
    inputs.iter().copied().map(|i|evaluate_coupled_step(i,t,e)).collect()
}
pub fn integrate_net_energy_wh(outputs:&[CoupledOutput])->f64{outputs.iter().map(|o|o.net_energy_wh).sum()}

#[cfg(test)]
mod tests{
 use super::*; use crate::thermal::NmotParameters;
 fn e()->ElectricalDesignBasis{ElectricalDesignBasis::canadian_solar_cs62_48tm_460h()}
 fn n()->NmotParameters{NmotParameters::canadian_solar_cs62_48tm_460h_nominal()}
 fn close(a:f64,b:f64){assert!((a-b).abs()<1e-9_f64.max(b.abs()*1e-12));}
 #[test] fn zero_irradiance_chain_and_aux_semantics(){let o=evaluate_coupled_step(CoupledInput{poa_w_m2:0.,ambient_temp_c:30.,dt_hours:2.,auxiliary_power_w:50.},n(),&e()).unwrap();close(o.module_temp_c,30.);close(o.delivered_dc_w,0.);close(o.ac_w,0.);close(o.net_w,-50.);close(o.net_energy_wh,-100.);}
 #[test] fn nmot_reference_chain_hand_checked(){let o=evaluate_coupled_step(CoupledInput{poa_w_m2:800.,ambient_temp_c:20.,dt_hours:1.,auxiliary_power_w:0.},n(),&e()).unwrap();close(o.module_temp_c,42.);let eta=0.230*(1.-0.0029*(42.-25.));close(o.efficiency,eta);let dc=800.*(1.762*1.134)*eta;close(o.ideal_dc_w,dc);close(o.delivered_dc_w,dc);close(o.ac_w,dc);close(o.net_w,dc);}
 #[test] fn intermediate_case_hand_checked(){let o=evaluate_coupled_step(CoupledInput{poa_w_m2:400.,ambient_temp_c:30.,dt_hours:0.5,auxiliary_power_w:5.},n(),&e()).unwrap();close(o.module_temp_c,41.);let eta=0.230*(1.-0.0029*(41.-25.));let dc=400.*(1.762*1.134)*eta;close(o.efficiency,eta);close(o.ac_w,dc);close(o.net_w,dc-5.);close(o.net_energy_wh,(dc-5.)*0.5);}
 #[test] fn nmot_sensitivity_orders_temperature_and_power(){let i=CoupledInput{poa_w_m2:700.,ambient_temp_c:30.,dt_hours:1.,auxiliary_power_w:0.};let p=NmotParameters::manufacturer_sensitivity();let o=p.map(|x|evaluate_coupled_step(i,x,&e()).unwrap());assert!(o[0].module_temp_c<o[1].module_temp_c&&o[1].module_temp_c<o[2].module_temp_c);assert!(o[0].ac_w>o[1].ac_w&&o[1].ac_w>o[2].ac_w);}
 #[test] fn declared_losses_do_not_increase_power(){let mut x=e();x.dc_mismatch_loss=.02;x.dc_wiring_loss=.01;x.inverter_efficiency=.98;x.ac_system_loss=.03;let o=evaluate_coupled_step(CoupledInput{poa_w_m2:800.,ambient_temp_c:20.,dt_hours:1.,auxiliary_power_w:10.},n(),&x).unwrap();assert!(o.delivered_dc_w<=o.ideal_dc_w&&o.ac_w<=o.delivered_dc_w);close(o.net_w,o.ac_w-10.);}
 #[test] fn invalids_propagate(){assert_eq!(evaluate_coupled_step(CoupledInput{poa_w_m2:-1.,ambient_temp_c:20.,dt_hours:1.,auxiliary_power_w:0.},n(),&e()),Err(CouplingError::Thermal(ThermalError::NegativeIrradiance)));let mut x=e();x.inverter_efficiency=0.;assert_eq!(evaluate_coupled_step(CoupledInput{poa_w_m2:1.,ambient_temp_c:20.,dt_hours:1.,auxiliary_power_w:0.},n(),&x),Err(CouplingError::Electrical(ElectricalError::InvalidInverterEfficiency)));assert!(evaluate_coupled_step(CoupledInput{poa_w_m2:1.,ambient_temp_c:f64::NAN,dt_hours:1.,auxiliary_power_w:0.},n(),&e()).is_err());assert!(evaluate_coupled_step(CoupledInput{poa_w_m2:1.,ambient_temp_c:20.,dt_hours:0.,auxiliary_power_w:0.},n(),&e()).is_err());}
 #[test] fn tiny_series_preserves_rows_and_explicit_energy_sum(){let xs=[CoupledInput{poa_w_m2:0.,ambient_temp_c:27.,dt_hours:1.,auxiliary_power_w:0.},CoupledInput{poa_w_m2:400.,ambient_temp_c:30.,dt_hours:1.,auxiliary_power_w:0.},CoupledInput{poa_w_m2:800.,ambient_temp_c:31.,dt_hours:1.,auxiliary_power_w:0.},CoupledInput{poa_w_m2:0.,ambient_temp_c:28.,dt_hours:1.,auxiliary_power_w:0.}];let a=evaluate_coupled_series(&xs,n(),&e()).unwrap();let b=evaluate_coupled_series(&xs,n(),&e()).unwrap();assert_eq!(a.len(),xs.len());assert_eq!(a,b);close(a[0].ac_w,0.);close(a[3].ac_w,0.);close(integrate_net_energy_wh(&a),a.iter().map(|o|o.net_w).sum());}
}
