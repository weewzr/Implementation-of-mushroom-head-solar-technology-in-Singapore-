use mushroom_solar::thermal::{module_temperature_nmot,NmotParameters,ThermalInput};
fn main(){
 println!("case_id,poa_w_m2,ambient_temp_c,nmot_c,module_temp_c,model,status");
 let cases=[("zero",0.,30.,42.),("nmot_reference",800.,20.,42.),("hand_400",400.,30.,42.),("sensitivity_39",700.,30.,39.),("sensitivity_42",700.,30.,42.),("sensitivity_45",700.,30.,45.)];
 for (id,g,ta,nmot) in cases {
   let p=NmotParameters{nmot_c:nmot,reference_ambient_c:20.,reference_irradiance_w_m2:800.};
   let tm=module_temperature_nmot(ThermalInput{poa_w_m2:g,ambient_temp_c:ta},p).expect("audited deterministic fixture");
   println!("{id},{g:.6},{ta:.6},{nmot:.6},{tm:.9},simple_nmot,DEVELOPMENT_NOT_SERIS");
 }
}