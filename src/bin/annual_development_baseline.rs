use std::{env,fs,path::Path};
use mushroom_solar::{weather::parse_nasa_power_hourly_csv,spa::{SpaInput,solar_position},irradiance::isotropic_poa};
const ALBEDO:f64=0.20;
fn parse_stamp(s:&str)->(i32,u8,u8,u8){(s[0..4].parse().unwrap(),s[5..7].parse().unwrap(),s[8..10].parse().unwrap(),s[11..13].parse().unwrap())}
fn svg_bars(path:&Path,monthly:&[[f64;4];12]){
 let w=1000.;let h=520.;let ml=70.;let mb=55.;let pw=w-100.;let ph=h-90.;let max=monthly.iter().map(|x|x[3]).fold(0f64,f64::max)*1.1;
 let mut s=format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}'><rect width='100%' height='100%' fill='white'/><text x='500' y='28' text-anchor='middle' font-family='sans-serif' font-size='18'>NASA POWER 2024 development baseline — horizontal POA</text>");
 for m in 0..12{let x=ml+(m as f64+.15)*pw/12.;let bw=.7*pw/12.;let mut y=h-mb;for k in 0..3{let bh=monthly[m][k]/max*ph;y-=bh;let fill=["#4472c4","#70ad47","#ed7d31"][k];s+=&format!("<rect x='{x:.1}' y='{y:.1}' width='{bw:.1}' height='{bh:.1}' fill='{fill}'/>");}s+=&format!("<text x='{:.1}' y='490' text-anchor='middle' font-family='sans-serif' font-size='12'>{}</text>",x+bw/2.,m+1);}
 s+="<text x='15' y='260' transform='rotate(-90 15 260)' text-anchor='middle' font-family='sans-serif' font-size='13'>Incident energy (Wh/m²/month)</text><text x='760' y='50' font-family='sans-serif' font-size='12'>direct / diffuse / ground (ground=0 for horizontal)</text></svg>";fs::write(path,s).unwrap();
}
fn main(){
 let input=env::args().nth(1).expect("usage: annual-development-baseline POWER.csv OUTDIR");let out=env::args().nth(2).unwrap_or("development-baseline-results".into());fs::create_dir_all(&out).unwrap();
 let raw=fs::read_to_string(input).unwrap();let(records,q)=parse_nasa_power_hourly_csv(&raw).unwrap();assert_eq!(records.len(),8784);assert!(q.is_empty());
 let configs:[(f64,f64);29]=[(0.,0.),(5.,0.),(5.,90.),(5.,180.),(5.,270.),(10.,0.),(10.,90.),(10.,180.),(10.,270.),(15.,0.),(15.,90.),(15.,180.),(15.,270.),(20.,0.),(20.,90.),(20.,180.),(20.,270.),(25.,0.),(25.,90.),(25.,180.),(25.,270.),(30.,0.),(30.,90.),(30.,180.),(30.,270.),(35.,0.),(35.,90.),(35.,180.),(35.,270.)];
 let mut month=vec![[[0f64;4];12];configs.len()];let mut annual=vec![[0f64;4];configs.len()];let mut night_direct_max=0f64;
 for rec in &records{let(y,m,d,h)=parse_stamp(&rec.timestamp);let pressure=rec.air_pressure_pa.unwrap_or(101000.)/100.;let pos=solar_position(&SpaInput{year:y,month:m,day:d,hour:h,minute:30,second:0.,utc_offset_h:0.,delta_t_s:69.,longitude_deg_east:103.8198,latitude_deg:1.3521,elevation_m:25.8,pressure_mbar:pressure,temperature_c:rec.ambient_temperature_c});
  let zen=pos.zenith_deg.to_radians();let az=pos.azimuth_deg.to_radians();let sun_up=pos.zenith_deg<90.;let dni=if sun_up{rec.dni_w_m2}else{0.};
  for(ci,(tilt,face))in configs.iter().enumerate(){let p=isotropic_poa(tilt.to_radians(),face.to_radians(),zen,az,dni,rec.ghi_w_m2,rec.dhi_w_m2,ALBEDO).unwrap();if !sun_up{night_direct_max=night_direct_max.max(p.direct_w_m2);}let vals=[p.direct_w_m2,p.sky_diffuse_w_m2,p.ground_diffuse_w_m2,p.global_w_m2];for k in 0..4{month[ci][m as usize-1][k]+=vals[k];annual[ci][k]+=vals[k];}}
 }
 assert!(night_direct_max<1e-12);for ci in 0..configs.len(){for k in 0..4{let sm:f64=month[ci].iter().map(|x|x[k]).sum();assert!((sm-annual[ci][k]).abs()<1e-7*annual[ci][k].abs().max(1.));}}
 assert!(annual[0][2].abs()<1e-9); // horizontal ground-view limit
 let mut a=String::from("dataset,status,tilt_deg,azimuth_deg,direct_wh_m2,diffuse_wh_m2,ground_wh_m2,total_wh_m2\n");for(ci,(t,z))in configs.iter().enumerate(){a+=&format!("NASA_POWER_SINGAPORE_2024,DEVELOPMENT_NOT_SERIS,{t},{z},{:.6},{:.6},{:.6},{:.6}\n",annual[ci][0],annual[ci][1],annual[ci][2],annual[ci][3]);}fs::write(Path::new(&out).join("annual_poa_sweep.csv"),a).unwrap();
 let mut mo=String::from("dataset,status,month,tilt_deg,azimuth_deg,direct_wh_m2,diffuse_wh_m2,ground_wh_m2,total_wh_m2\n");for(ci,(t,z))in configs.iter().enumerate(){for m in 0..12{mo+=&format!("NASA_POWER_SINGAPORE_2024,DEVELOPMENT_NOT_SERIS,{},{t},{z},{:.6},{:.6},{:.6},{:.6}\n",m+1,month[ci][m][0],month[ci][m][1],month[ci][m][2],month[ci][m][3]);}}fs::write(Path::new(&out).join("monthly_poa_sweep.csv"),mo).unwrap();
 svg_bars(&Path::new(&out).join("horizontal_monthly_components.svg"),&month[0]);
 let checks=format!("dataset=NASA_POWER_SINGAPORE_2024_DEVELOPMENT\nsamples={}\ninterval_hours=1\ninput_irradiance_semantics=POWER hourly Wh/m2; numerically equal to 1-hour mean W/m2 only because dt=1h\nspa_evaluation=hour midpoint UTC\nnight_direct_max_w_m2={:.12}\nmonthly_annual_conservation=PASS\nhorizontal_ground_limit=PASS\nleap_year_completeness=PASS\nground_albedo_assumption={}\nstatus=DEVELOPMENT_NOT_SERIS\n",records.len(),night_direct_max,ALBEDO);fs::write(Path::new(&out).join("checks.txt"),checks).unwrap();
 println!("annual-development-baseline PASS samples={} horizontal_total_wh_m2={:.3}",records.len(),annual[0][3]);
}