use std::{env,fs,path::Path};
use mushroom_solar::{
 annual_irradiance::{evaluate_annual_irradiance,AnnualIrradianceSettings,AnnualIrradianceResult},
 fixed_geometry::{apply_contract,generate_candidate,resources,ComparisonContract,FixedGeometry},
 resource_geometry::RESOURCE_AREA_TOL_M2,
 weather::{parse_nasa_power_hourly_csv,WeatherRecord},
};
const STATUS:&str="DEVELOPMENT_NOT_SERIS";
const LAND:f64=1.0;
const PV:f64=1.0;
fn candidates()->[(&'static str,FixedGeometry);5]{[
 ("flat_reference",FixedGeometry::Flat),
 ("frozen_paraboloid",FixedGeometry::Paraboloid{height_over_radius:0.5}),
 ("hemisphere",FixedGeometry::Hemisphere),
 ("faceted_canopy",FixedGeometry::FacetedCanopy{height_over_radius:0.5,facets:24}),
 ("folded_surface",FixedGeometry::FoldedSurface{tilt_rad:0.4}),
]}
fn eval(records:&[WeatherRecord],mesh:&[mushroom_solar::visibility::Triangle])->AnnualIrradianceResult{
 evaluate_annual_irradiance(records,mesh,AnnualIrradianceSettings{sky_n:8,albedo:0.20,quarter_hour:false}).unwrap()
}
fn main(){
 let inp=env::args().nth(1).expect("usage: fixed-geometry-comparison POWER.csv OUTDIR");
 let out=env::args().nth(2).unwrap_or("fixed-geometry-comparison".into());fs::create_dir_all(&out).unwrap();
 let raw=fs::read_to_string(inp).unwrap();let(records,q)=parse_nasa_power_hourly_csv(&raw).unwrap();assert_eq!(records.len(),8784);assert!(q.is_empty());
 let mut annual=String::from("status,contract,geometry,nr,nphi,facets,pv_area_m2,land_area_m2,packing_ratio,direct_wh,diffuse_wh,ground_wh,total_wh,pv_norm_wh_m2pv,land_norm_wh_m2land\n");
 let mut monthly=String::from("status,contract,geometry,month,direct_wh,diffuse_wh,ground_wh,total_wh\n");
 let mut inv=String::from("status,contract,geometry,target,actual,abs_error,tolerance,accepted\n");
 let mut geom=String::from("status,geometry,nr,nphi,facets,pv_area_m2,land_area_m2,packing_ratio,rel_packing_to_finest\n");
 let mut packing=String::from("status,geometry,native_packing_ratio,matched_target,feasible,reason\n");
 for(name,g) in candidates(){
   let mut prs=Vec::new();
   for &(nr,np) in &[(2usize,12usize),(3,18),(4,24)]{
     let m=generate_candidate(g,nr,np).unwrap();let r=resources(&m).unwrap();prs.push((nr,np,m.len(),r));
   }
   let finest=prs.last().unwrap().3.packing_ratio();
   for(nr,np,nf,r) in prs{geom+=&format!("{STATUS},{name},{nr},{np},{nf},{:.12},{:.12},{:.12},{:.12}\n",r.active_pv_area_m2,r.projected_land_area_m2,r.packing_ratio(),(r.packing_ratio()-finest)/finest);}
   let native=resources(&generate_candidate(g,2,12).unwrap()).unwrap().packing_ratio();
   let target=1.0;let feasible=(native-target).abs()<=1e-8;
   packing+=&format!("{STATUS},{name},{native:.12},{target:.12},{feasible},{}\n",if feasible{"intrinsic discrete packing matches target"}else{"REJECTED: frozen topology packing differs; uniform scaling cannot alter packing ratio"});
   for(contract,c,target_value,is_land) in [
     ("equal_land",ComparisonContract::EqualLand{land_m2:LAND},LAND,true),
     ("equal_pv",ComparisonContract::EqualPv{pv_m2:PV},PV,false),
   ]{
     let base=generate_candidate(g,2,12).unwrap();let m=apply_contract(&base,c).unwrap();let rr=resources(&m).unwrap();
     let actual=if is_land{rr.projected_land_area_m2}else{rr.active_pv_area_m2};let tol=RESOURCE_AREA_TOL_M2.max(target_value.abs()*1e-12);
     assert!((actual-target_value).abs()<=tol);
     inv+=&format!("{STATUS},{contract},{name},{target_value:.12},{actual:.12},{:.3e},{tol:.3e},true\n",(actual-target_value).abs());
     let a=eval(&records,&m);
     annual+=&format!("{STATUS},{contract},{name},2,12,{},{:.12},{:.12},{:.12},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}\n",m.len(),rr.active_pv_area_m2,rr.projected_land_area_m2,rr.packing_ratio(),a.direct_wh,a.diffuse_wh,a.ground_wh,a.total_wh,a.total_wh/rr.active_pv_area_m2,a.total_wh/rr.projected_land_area_m2);
     for mo in 1..=12{let subset:Vec<_>=records.iter().filter(|r|r.timestamp[5..7].parse::<usize>().unwrap()==mo).cloned().collect();let x=eval(&subset,&m);monthly+=&format!("{STATUS},{contract},{name},{mo},{:.6},{:.6},{:.6},{:.6}\n",x.direct_wh,x.diffuse_wh,x.ground_wh,x.total_wh);}
   }
   if feasible{
     let base=generate_candidate(g,2,12).unwrap();let m=apply_contract(&base,ComparisonContract::MatchedPackingRatio{packing_ratio:target,land_m2:LAND}).unwrap();let rr=resources(&m).unwrap();let a=eval(&records,&m);
     annual+=&format!("{STATUS},matched_packing_ratio,{name},2,12,{},{:.12},{:.12},{:.12},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}\n",m.len(),rr.active_pv_area_m2,rr.projected_land_area_m2,rr.packing_ratio(),a.direct_wh,a.diffuse_wh,a.ground_wh,a.total_wh,a.total_wh/rr.active_pv_area_m2,a.total_wh/rr.projected_land_area_m2);
   }
 }
 fs::write(Path::new(&out).join("annual_results.csv"),annual).unwrap();
 fs::write(Path::new(&out).join("monthly_results.csv"),monthly).unwrap();
 fs::write(Path::new(&out).join("resource_invariants.csv"),inv).unwrap();
 fs::write(Path::new(&out).join("geometry_mesh_checks.csv"),geom).unwrap();
 fs::write(Path::new(&out).join("matched_packing_feasibility.csv"),packing).unwrap();
 fs::write(Path::new(&out).join("README.txt"),"Controlled fixed-geometry comparison. DEVELOPMENT_NOT_SERIS. NASA POWER Singapore 2024; frozen SPA/shared annual irradiance evaluator; albedo 0.20; sky_n 8; hourly midpoint. Equal-land and equal-PV contracts execute through canonical resource_geometry normalization. Matched-packing target Pi=1 is accepted only where the frozen topology intrinsically satisfies it; no geometry parameter is optimized or altered to force feasibility. Folded surface remains attached to the deployable/foldable solar-sheet implementation concept; no mechanical/deployment/cost penalty is charged here. No thermal/electrical conversion, tracking, economics or topology optimization. Results characterize irradiance only and are not a winner declaration.\n").unwrap();
 println!("fixed-geometry-comparison PASS {STATUS}; see machine-readable artifacts");
}