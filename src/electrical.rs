//! Geometry-agnostic PV electrical conversion kernel.
//! Units are explicit in public field names. This is a bounded engineering model,
//! not a diode/MPPT model and not an annual geometry-performance claim.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElectricalDesignBasis {
    pub reference_efficiency: f64,
    pub module_area_m2: f64,
    pub pmax_temp_coeff_per_c: f64,
    pub reference_temp_c: f64,
    pub nmot_c: f64,
    pub nmot_irradiance_w_m2: f64,
    pub nmot_ambient_c: f64,
    pub dc_mismatch_loss: f64,
    pub dc_wiring_loss: f64,
    pub inverter_efficiency: f64,
    pub ac_system_loss: f64,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StepInput { pub poa_w_m2:f64, pub ambient_temp_c:f64, pub dt_hours:f64, pub auxiliary_power_w:f64 }
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StepOutput { pub module_temp_c:f64, pub efficiency:f64, pub ideal_dc_w:f64, pub delivered_dc_w:f64, pub ac_w:f64, pub net_w:f64, pub ac_energy_wh:f64, pub net_energy_wh:f64 }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElectricalError { NonFinite, NegativeIrradiance, InvalidArea, InvalidEfficiency, InvalidLoss, InvalidInverterEfficiency, InvalidTimeStep, InvalidAuxiliary, InvalidNmotBasis }

impl ElectricalDesignBasis {
    pub fn canadian_solar_cs62_48tm_460h() -> Self {
        Self { reference_efficiency:0.230, module_area_m2:1.762*1.134, pmax_temp_coeff_per_c:-0.0029, reference_temp_c:25.0,
            nmot_c:42.0, nmot_irradiance_w_m2:800.0, nmot_ambient_c:20.0,
            dc_mismatch_loss:0.0, dc_wiring_loss:0.0, inverter_efficiency:1.0, ac_system_loss:0.0 }
    }
    pub fn validate(&self)->Result<(),ElectricalError>{
        let xs=[self.reference_efficiency,self.module_area_m2,self.pmax_temp_coeff_per_c,self.reference_temp_c,self.nmot_c,self.nmot_irradiance_w_m2,self.nmot_ambient_c,self.dc_mismatch_loss,self.dc_wiring_loss,self.inverter_efficiency,self.ac_system_loss];
        if xs.iter().any(|x|!x.is_finite()){return Err(ElectricalError::NonFinite)}
        if self.module_area_m2<=0.0{return Err(ElectricalError::InvalidArea)}
        if !(self.reference_efficiency>0.0 && self.reference_efficiency<=1.0){return Err(ElectricalError::InvalidEfficiency)}
        if self.nmot_irradiance_w_m2<=0.0{return Err(ElectricalError::InvalidNmotBasis)}
        for l in [self.dc_mismatch_loss,self.dc_wiring_loss,self.ac_system_loss]{if !(0.0..1.0).contains(&l){return Err(ElectricalError::InvalidLoss)}}
        if !(self.inverter_efficiency>0.0 && self.inverter_efficiency<=1.0){return Err(ElectricalError::InvalidInverterEfficiency)}
        Ok(())
    }
}
pub fn module_temperature_nmot(poa_w_m2:f64, ambient_temp_c:f64, b:&ElectricalDesignBasis)->Result<f64,ElectricalError>{
    b.validate()?; if !poa_w_m2.is_finite()||!ambient_temp_c.is_finite(){return Err(ElectricalError::NonFinite)} if poa_w_m2<0.0{return Err(ElectricalError::NegativeIrradiance)}
    Ok(ambient_temp_c + (b.nmot_c-b.nmot_ambient_c)*poa_w_m2/b.nmot_irradiance_w_m2)
}
pub fn temperature_adjusted_efficiency(module_temp_c:f64,b:&ElectricalDesignBasis)->Result<f64,ElectricalError>{
    b.validate()?; if !module_temp_c.is_finite(){return Err(ElectricalError::NonFinite)}
    let e=b.reference_efficiency*(1.0+b.pmax_temp_coeff_per_c*(module_temp_c-b.reference_temp_c));
    if !e.is_finite()||e<0.0{return Err(ElectricalError::InvalidEfficiency)} Ok(e)
}
pub fn evaluate_step(i:StepInput,b:&ElectricalDesignBasis)->Result<StepOutput,ElectricalError>{
    b.validate()?; if !i.poa_w_m2.is_finite()||!i.ambient_temp_c.is_finite()||!i.dt_hours.is_finite()||!i.auxiliary_power_w.is_finite(){return Err(ElectricalError::NonFinite)}
    if i.poa_w_m2<0.0{return Err(ElectricalError::NegativeIrradiance)} if i.dt_hours<=0.0{return Err(ElectricalError::InvalidTimeStep)} if i.auxiliary_power_w<0.0{return Err(ElectricalError::InvalidAuxiliary)}
    let tm=module_temperature_nmot(i.poa_w_m2,i.ambient_temp_c,b)?; let eff=temperature_adjusted_efficiency(tm,b)?;
    let ideal=i.poa_w_m2*b.module_area_m2*eff; let dc=ideal*(1.0-b.dc_mismatch_loss)*(1.0-b.dc_wiring_loss);
    let ac=dc*b.inverter_efficiency*(1.0-b.ac_system_loss); if !ac.is_finite()||ac<0.0{return Err(ElectricalError::InvalidEfficiency)}
    let net=ac-i.auxiliary_power_w;
    Ok(StepOutput{module_temp_c:tm,efficiency:eff,ideal_dc_w:ideal,delivered_dc_w:dc,ac_w:ac,net_w:net,ac_energy_wh:ac*i.dt_hours,net_energy_wh:net*i.dt_hours})
}
pub fn integrate(inputs:&[StepInput],b:&ElectricalDesignBasis)->Result<(f64,f64),ElectricalError>{
    let mut ac=0.0;let mut net=0.0;for &i in inputs{let o=evaluate_step(i,b)?;ac+=o.ac_energy_wh;net+=o.net_energy_wh;}Ok((ac,net))
}
#[cfg(test)]
mod tests {
 use super::*; fn b()->ElectricalDesignBasis{ElectricalDesignBasis::canadian_solar_cs62_48tm_460h()} fn close(a:f64,c:f64){assert!((a-c).abs()<1e-9_f64.max(c.abs()*1e-12));}
 #[test] fn zero_irradiance_zero_gross(){let o=evaluate_step(StepInput{poa_w_m2:0.,ambient_temp_c:25.,dt_hours:1.,auxiliary_power_w:0.},&b()).unwrap();close(o.ideal_dc_w,0.);close(o.ac_w,0.);}
 #[test] fn reference_temperature_reference_efficiency(){close(temperature_adjusted_efficiency(25.,&b()).unwrap(),0.230);}
 #[test] fn hotter_reduces_and_colder_increases(){let x=b();assert!(temperature_adjusted_efficiency(45.,&x).unwrap()<x.reference_efficiency);assert!(temperature_adjusted_efficiency(5.,&x).unwrap()>x.reference_efficiency);}
 #[test] fn zero_losses_ideal_dc(){let x=b();let o=evaluate_step(StepInput{poa_w_m2:1000.,ambient_temp_c:3.,dt_hours:1.,auxiliary_power_w:0.},&x).unwrap();close(o.delivered_dc_w,o.ideal_dc_w);close(o.ac_w,o.ideal_dc_w);}
 #[test] fn losses_monotonic(){let x=b();let mut y=x;y.dc_mismatch_loss=.02;y.dc_wiring_loss=.01;y.inverter_efficiency=.98;y.ac_system_loss=.03;let i=StepInput{poa_w_m2:800.,ambient_temp_c:20.,dt_hours:1.,auxiliary_power_w:0.};assert!(evaluate_step(i,&y).unwrap().ac_w<evaluate_step(i,&x).unwrap().ac_w);}
 #[test] fn invalids_rejected(){let mut x=b();x.module_area_m2=0.;assert_eq!(x.validate(),Err(ElectricalError::InvalidArea));let mut x=b();x.dc_wiring_loss=1.;assert_eq!(x.validate(),Err(ElectricalError::InvalidLoss));assert_eq!(evaluate_step(StepInput{poa_w_m2:1.,ambient_temp_c:20.,dt_hours:0.,auxiliary_power_w:0.},&b()),Err(ElectricalError::InvalidTimeStep));assert_eq!(evaluate_step(StepInput{poa_w_m2:-1.,ambient_temp_c:20.,dt_hours:1.,auxiliary_power_w:0.},&b()),Err(ElectricalError::NegativeIrradiance));}
 #[test] fn inverter_finite_nonnegative(){let o=evaluate_step(StepInput{poa_w_m2:800.,ambient_temp_c:20.,dt_hours:1.,auxiliary_power_w:0.},&b()).unwrap();assert!(o.ac_w.is_finite()&&o.ac_w>=0.);}
 #[test] fn integration_is_explicit_sum(){let i=StepInput{poa_w_m2:800.,ambient_temp_c:20.,dt_hours:.5,auxiliary_power_w:10.};let o=evaluate_step(i,&b()).unwrap();let (ac,net)=integrate(&[i,i,i],&b()).unwrap();close(ac,3.*o.ac_energy_wh);close(net,3.*o.net_energy_wh);}
 #[test] fn auxiliary_and_net_import_explicit(){let o=evaluate_step(StepInput{poa_w_m2:0.,ambient_temp_c:20.,dt_hours:2.,auxiliary_power_w:50.},&b()).unwrap();close(o.ac_w,0.);close(o.net_w,-50.);close(o.net_energy_wh,-100.);}
 #[test] fn deterministic_repeatability(){let i=StepInput{poa_w_m2:713.,ambient_temp_c:31.,dt_hours:.25,auxiliary_power_w:3.};assert_eq!(evaluate_step(i,&b()).unwrap(),evaluate_step(i,&b()).unwrap());}
 #[test] fn nmot_fixture(){close(module_temperature_nmot(800.,20.,&b()).unwrap(),42.);}
}
