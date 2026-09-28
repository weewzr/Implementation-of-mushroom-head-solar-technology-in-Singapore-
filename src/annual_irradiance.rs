//! Shared fixed-geometry annual irradiance evaluator.
//! Geometry-specific branches are intentionally forbidden here.

use crate::{mesh::Vec3, resource_geometry::{discrete_resources,triangle_area_normal,DiscreteResources,ResourceGeometryError}, spa::{solar_position,SpaInput}, visibility::{direct_visibility,sky_view_factor,Triangle}, weather::WeatherRecord};

#[derive(Clone,Copy,Debug)]
pub struct AnnualIrradianceSettings { pub sky_n:usize, pub albedo:f64, pub quarter_hour:bool }
impl Default for AnnualIrradianceSettings { fn default()->Self{Self{sky_n:16,albedo:0.20,quarter_hour:false}} }
#[derive(Clone,Copy,Debug)]
pub struct TimestepIrradianceResult { pub timestamp_utc_s:i64,pub direct_wh:f64,pub diffuse_wh:f64,pub ground_wh:f64,pub total_wh:f64 }
#[derive(Clone,Copy,Debug)]
pub struct AnnualIrradianceResult { pub direct_wh:f64,pub diffuse_wh:f64,pub ground_wh:f64,pub total_wh:f64,pub resources:DiscreteResources }
fn sun(z:f64,a:f64)->Vec3{Vec3{x:z.sin()*a.sin(),y:z.sin()*a.cos(),z:z.cos()}}
fn stamp(s:&str)->(i32,u8,u8,u8){(s[0..4].parse().unwrap(),s[5..7].parse().unwrap(),s[8..10].parse().unwrap(),s[11..13].parse().unwrap())}

pub fn evaluate_timestep_irradiance(rs:&[WeatherRecord],mesh:&[Triangle],settings:AnnualIrradianceSettings)->Result<Vec<TimestepIrradianceResult>,ResourceGeometryError>{
 if !settings.albedo.is_finite()||!(0.0..=1.0).contains(&settings.albedo)||settings.sky_n==0{return Err(ResourceGeometryError::InvalidTarget)}
 let ga:Vec<_>=mesh.iter().map(triangle_area_normal).collect::<Result<_,_>>()?;
 let sv:Vec<_>=mesh.iter().enumerate().map(|(i,t)|sky_view_factor(t.centroid(),ga[i].1,mesh,Some(i),settings.sky_n,4*settings.sky_n)).collect();
 let mut out=Vec::with_capacity(rs.len());
 for w in rs{let(y,m,d,h)=stamp(&w.timestamp);let mins:&[u8]=if settings.quarter_hour{&[7,22,37,52]}else{&[30]};let wt=1.0/mins.len() as f64;let mut e=[0.0;3];
  for &minute in mins{let p=solar_position(&SpaInput{year:y,month:m,day:d,hour:h,minute,second:if settings.quarter_hour{30.0}else{0.0},utc_offset_h:0.0,delta_t_s:69.0,longitude_deg_east:103.8198,latitude_deg:1.3521,elevation_m:25.8,pressure_mbar:w.air_pressure_pa.unwrap_or(101000.0)/100.0,temperature_c:w.ambient_temperature_c});let ss=sun(p.zenith_deg.to_radians(),p.azimuth_deg.to_radians());
   for(i,_)in mesh.iter().enumerate(){let(a,n)=ga[i];if p.zenith_deg<90.0{e[0]+=wt*w.dni_w_m2*a*n.dot(ss).max(0.0)*direct_visibility(i,mesh,ss)}e[1]+=wt*w.dhi_w_m2*a*sv[i];e[2]+=wt*w.ghi_w_m2*settings.albedo*a*(1.0-n.z)/2.0;}
  }
  let total:f64=e.iter().sum();if e.iter().any(|x|!x.is_finite()||*x<0.0)||!total.is_finite(){return Err(ResourceGeometryError::NonFiniteGeometry)}
  out.push(TimestepIrradianceResult{timestamp_utc_s:w.timestamp_utc_s,direct_wh:e[0],diffuse_wh:e[1],ground_wh:e[2],total_wh:total});
 }
 Ok(out)
}

pub fn evaluate_annual_irradiance(rs:&[WeatherRecord],mesh:&[Triangle],settings:AnnualIrradianceSettings)->Result<AnnualIrradianceResult,ResourceGeometryError>{
 if !settings.albedo.is_finite()||!(0.0..=1.0).contains(&settings.albedo)||settings.sky_n==0{return Err(ResourceGeometryError::InvalidTarget)}
 let resources=discrete_resources(mesh)?;
 let ts=evaluate_timestep_irradiance(rs,mesh,settings)?;
 let e=[ts.iter().map(|x|x.direct_wh).sum::<f64>(),ts.iter().map(|x|x.diffuse_wh).sum::<f64>(),ts.iter().map(|x|x.ground_wh).sum::<f64>()];
 let total:f64=e.iter().sum();if !total.is_finite(){return Err(ResourceGeometryError::NonFiniteGeometry)}
 Ok(AnnualIrradianceResult{direct_wh:e[0],diffuse_wh:e[1],ground_wh:e[2],total_wh:total,resources})
}
