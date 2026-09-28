use mushroom_solar::{electrical::ElectricalDesignBasis,thermal::NmotParameters,thermal_electrical::{evaluate_coupled_step,CoupledInput}};
fn main(){
 println!("case_id,poa_w_m2,ambient_temp_c,nmot_c,module_temp_c,eta_module,p_dc_w,p_ac_w,p_aux_w,p_net_w,dt_h,e_net_wh,status");
 let cases=[("zero",0.,30.,42.,2.,50.),("nmot_reference",800.,20.,42.,1.,0.),("hand_400",400.,30.,42.,0.5,5.),("sensitivity_39",700.,30.,39.,1.,0.),("sensitivity_42",700.,30.,42.,1.,0.),("sensitivity_45",700.,30.,45.,1.,0.)];
 let e=ElectricalDesignBasis::canadian_solar_cs62_48tm_460h();
 for(id,g,ta,nmot,dt,aux) in cases{let t=NmotParameters{nmot_c:nmot,reference_ambient_c:20.,reference_irradiance_w_m2:800.};let o=evaluate_coupled_step(CoupledInput{poa_w_m2:g,ambient_temp_c:ta,dt_hours:dt,auxiliary_power_w:aux},t,&e).unwrap();println!("{id},{g:.6},{ta:.6},{nmot:.6},{:.9},{:.12},{:.9},{:.9},{aux:.9},{:.9},{dt:.6},{:.9},DEVELOPMENT_NOT_SERIS",o.module_temp_c,o.efficiency,o.delivered_dc_w,o.ac_w,o.net_w,o.net_energy_wh);}
}