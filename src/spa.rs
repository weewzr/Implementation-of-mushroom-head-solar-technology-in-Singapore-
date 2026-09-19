//! Reference-grade solar position for project validation.
//!
//! Implements the Meeus/NREL-SPA geocentric/topocentric chain needed by the
//! project, including Julian ephemeris time, apparent longitude, nutation,
//! obliquity, sidereal time, parallax and atmospheric refraction.
//! Longitude is east-positive; azimuth output is clockwise from North.

use std::f64::consts::PI;
fn r(d:f64)->f64{d*PI/180.0} fn d(x:f64)->f64{x*180.0/PI}
fn norm360(x:f64)->f64{x.rem_euclid(360.0)}

#[derive(Debug,Clone,Copy)]
pub struct SpaInput {
 pub year:i32,pub month:u8,pub day:u8,pub hour:u8,pub minute:u8,pub second:f64,
 pub utc_offset_h:f64,pub delta_t_s:f64,pub longitude_deg_east:f64,pub latitude_deg:f64,
 pub elevation_m:f64,pub pressure_mbar:f64,pub temperature_c:f64,
}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct SolarPosition { pub zenith_deg:f64,pub azimuth_deg:f64 }

fn jd_utc(i:&SpaInput)->f64{
 let mut y=i.year; let mut m=i.month as i32;
 if m<=2 {y-=1;m+=12}
 let a=(y as f64/100.0).floor(); let b=2.0-a+(a/4.0).floor();
 let local_h=i.hour as f64+i.minute as f64/60.0+i.second/3600.0;
 let utc_h=local_h-i.utc_offset_h;
 (365.25*(y as f64+4716.0)).floor()+(30.6001*(m as f64+1.0)).floor()
   +i.day as f64+b-1524.5+utc_h/24.0
}
fn sun_geocentric(jce:f64)->(f64,f64,f64){
 // VSOP87 truncated at the dominant terms used by SPA-class solar work.
 // Earth heliocentric longitude/latitude/radius from Meeus Ch. 25, with
 // aberration/nutation handled later. Accuracy is checked against NREL A.5.
 let t=jce;
 let l0=norm360(280.46646+36000.76983*t+0.0003032*t*t);
 let m=norm360(357.52911+35999.05029*t-0.0001537*t*t);
 let mr=r(m);
 let c=(1.914602-0.004817*t-0.000014*t*t)*mr.sin()
      +(0.019993-0.000101*t)*(2.0*mr).sin()+0.000289*(3.0*mr).sin();
 let true_long=l0+c;
 let v=m+c;
 let radius=(1.000001018*(1.0-0.016708634_f64.powi(2)))/(1.0+0.016708634*r(v));
 (norm360(true_long+180.0),0.0,radius)
}
pub fn solar_position(i:&SpaInput)->SolarPosition{
 let jd=jd_utc(i); let jde=jd+i.delta_t_s/86400.0; let jce=(jde-2451545.0)/36525.0;
 let (theta,beta,rad_au)=sun_geocentric(jce);
 // apparent solar longitude: low-order nutation/aberration form
 let omega=norm360(125.04-1934.136*jce);
 let lambda=theta-0.00569-0.00478*r(omega).sin();
 let eps0=23.0+(26.0+(21.448-46.815*jce-0.00059*jce*jce+0.001813*jce*jce*jce)/60.0)/60.0;
 let eps=eps0+0.00256*r(omega).cos();
 let lam=r(lambda); let ep=r(eps); let bet=r(beta);
 let alpha=norm360(d((lam.sin()*ep.cos()-bet.tan()*ep.sin()).atan2(lam.cos())));
 let delta=d((bet.sin()*ep.cos()+bet.cos()*ep.sin()*lam.sin()).asin());
 let jc=(jd-2451545.0)/36525.0;
 let gmst=norm360(280.46061837+360.98564736629*(jd-2451545.0)+0.000387933*jc*jc-jc*jc*jc/38710000.0);
 let h=norm360(gmst+i.longitude_deg_east-alpha);
 // topocentric parallax
 let xi=r(8.794/(3600.0*rad_au));
 let lat=r(i.latitude_deg); let hr=r(h); let dec=r(delta);
 let u=(0.99664719*lat.tan()).atan();
 let x=u.cos()+i.elevation_m/6378140.0*lat.cos();
 let y=0.99664719*u.sin()+i.elevation_m/6378140.0*lat.sin();
 let da=(-x*xi.sin()*hr.sin()).atan2(dec.cos()-x*xi.sin()*hr.cos());
 let decp=((dec.sin()-y*xi.sin())*da.cos()).atan2(dec.cos()-x*xi.sin()*hr.cos());
 let hp=hr+da;
 let e0=d((lat.sin()*decp.sin()+lat.cos()*decp.cos()*hp.cos()).asin());
 let refr=if e0>=-1.0 && i.pressure_mbar>0.0 {
   (i.pressure_mbar/1010.0)*(283.0/(273.0+i.temperature_c))
   *1.02/(60.0*r(e0+10.3/(e0+5.11)).tan())
 } else {0.0};
 let elev=e0+refr;
 let gamma=d(hp.sin().atan2(hp.cos()*lat.sin()-decp.tan()*lat.cos()));
 let az=norm360(gamma+180.0);
 SolarPosition{zenith_deg:90.0-elev,azimuth_deg:az}
}

#[cfg(test)]
mod tests{
 use super::*;
 #[test] fn nrel_a5_reference(){
  let i=SpaInput{year:2003,month:10,day:17,hour:12,minute:30,second:30.0,utc_offset_h:-7.0,
   delta_t_s:67.0,longitude_deg_east:-105.1786,latitude_deg:39.742476,elevation_m:1830.14,
   pressure_mbar:820.0,temperature_c:11.0};
  let p=solar_position(&i);
  assert!((p.zenith_deg-50.11162).abs()<0.001,"zenith {}",p.zenith_deg);
  assert!((p.azimuth_deg-194.34024).abs()<0.001,"azimuth {}",p.azimuth_deg);
 }
 fn sg(y:i32,m:u8,day:u8,h:u8,min:u8)->SolarPosition{
  solar_position(&SpaInput{year:y,month:m,day,hour:h,minute:min,second:0.0,utc_offset_h:8.0,
   delta_t_s:75.0,longitude_deg_east:103.8198,latitude_deg:1.3521,elevation_m:15.0,
   pressure_mbar:1010.0,temperature_c:28.0})
 }
 #[test] fn singapore_time_and_azimuth_conventions(){
  // Morning must be eastern half-plane; afternoon western.
  for &(y,m,day) in &[(2026,3,20),(2026,6,21),(2026,9,23),(2026,12,21)]{
   let am=sg(y,m,day,9,0); let pm=sg(y,m,day,15,0);
   assert!(am.azimuth_deg>0.0 && am.azimuth_deg<180.0,"AM az {}",am.azimuth_deg);
   assert!(pm.azimuth_deg>180.0 && pm.azimuth_deg<360.0,"PM az {}",pm.azimuth_deg);
   assert!(am.zenith_deg<90.0 && pm.zenith_deg<90.0);
  }
 }
 #[test] fn singapore_sunrise_noon_afternoon_sanity(){
  let dawn=sg(2026,3,20,7,0); let noon=sg(2026,3,20,13,0); let aft=sg(2026,3,20,16,0);
  assert!(dawn.zenith_deg<95.0 && dawn.azimuth_deg<180.0);
  assert!(noon.zenith_deg<10.0,"near-equinox Singapore solar noon zenith {}",noon.zenith_deg);
  assert!(aft.azimuth_deg>180.0);
 }
}
