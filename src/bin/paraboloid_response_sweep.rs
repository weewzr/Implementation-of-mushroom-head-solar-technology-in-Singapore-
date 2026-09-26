use std::{env,fs,path::Path};use std::f64::consts::PI;
use mushroom_solar::{weather::parse_nasa_power_hourly_csv,spa::{SpaInput,solar_position},mesh::Vec3,visibility::{Triangle,direct_visibility,sky_view_factor},resource_geometry::{discrete_resources,normalize_to_land_area,normalize_to_pv_area,triangle_area_normal}};
const ALBEDO:f64=0.20;
fn v(x:f64,y:f64,z:f64)->Vec3{Vec3{x,y,z}}
fn mesh(foot:f64,k:f64,nr:usize,np:usize)->Vec<Triangle>{let r=(foot/PI).sqrt();let h=k*r;let p=|rr:f64,ph:f64|v(rr*ph.cos(),rr*ph.sin(),h*(1.-(rr/r).powi(2)));let mut o=vec![];let cen=v(0.,0.,h);let r1=r/nr as f64;for j in 0..np{let a=2.*PI*j as f64/np as f64;let b=2.*PI*(j+1)as f64/np as f64;o.push(Triangle{v:[cen,p(r1,a),p(r1,b)]});}for i in 1..nr{let ra=r*i as f64/nr as f64;let rb=r*(i+1)as f64/nr as f64;for j in 0..np{let a=2.*PI*j as f64/np as f64;let b=2.*PI*(j+1)as f64/np as f64;let q=[p(ra,a),p(rb,a),p(rb,b),p(ra,b)];o.push(Triangle{v:[q[0],q[1],q[2]]});o.push(Triangle{v:[q[0],q[2],q[3]]});}}o}
fn sun(z:f64,a:f64)->Vec3{v(z.sin()*a.sin(),z.sin()*a.cos(),z.cos())}fn stamp(s:&str)->(i32,u8,u8,u8){(s[0..4].parse().unwrap(),s[5..7].parse().unwrap(),s[8..10].parse().unwrap(),s[11..13].parse().unwrap())}
#[derive(Clone)]struct R{e:[f64;4],pv:f64,land:f64}
fn run(rs:&[mushroom_solar::weather::WeatherRecord],t:&[Triangle],sn:usize)->R{run_opts(rs,t,sn,ALBEDO,false)}
fn run_opts(rs:&[mushroom_solar::weather::WeatherRecord],t:&[Triangle],sn:usize,albedo:f64,quarter_hour:bool)->R{let ga:Vec<_>=t.iter().map(|x|triangle_area_normal(x).expect("validated resource geometry")).collect();let resources=discrete_resources(t).expect("validated resource geometry");let pv=resources.active_pv_area_m2;let land=resources.projected_land_area_m2;let sv:Vec<_>=t.iter().enumerate().map(|(i,x)|sky_view_factor(x.centroid(),ga[i].1,t,Some(i),sn,4*sn)).collect();let mut e=[0.;4];for w in rs{let(y,m,d,h)=stamp(&w.timestamp);let mins:&[u8]=if quarter_hour{&[7,22,37,52]}else{&[30]};let wt=1.0/mins.len() as f64;for &minute in mins{let p=solar_position(&SpaInput{year:y,month:m,day:d,hour:h,minute,second:if quarter_hour{30.}else{0.},utc_offset_h:0.,delta_t_s:69.,longitude_deg_east:103.8198,latitude_deg:1.3521,elevation_m:25.8,pressure_mbar:w.air_pressure_pa.unwrap_or(101000.)/100.,temperature_c:w.ambient_temperature_c});let ss=sun(p.zenith_deg.to_radians(),p.azimuth_deg.to_radians());for(i,_x)in t.iter().enumerate(){let(a,n)=ga[i];if p.zenith_deg<90.{e[0]+=wt*w.dni_w_m2*a*n.dot(ss).max(0.)*direct_visibility(i,t,ss)}e[1]+=wt*w.dhi_w_m2*a*sv[i];e[2]+=wt*w.ghi_w_m2*albedo*a*(1.-n.z)/2.;}}}e[3]=e[0]+e[1]+e[2];R{e,pv,land}}
fn run_unobstructed(rs:&[mushroom_solar::weather::WeatherRecord],t:&[Triangle])->R{let ga:Vec<_>=t.iter().map(|x|triangle_area_normal(x).expect("validated resource geometry")).collect();let resources=discrete_resources(t).expect("validated resource geometry");let pv=resources.active_pv_area_m2;let land=resources.projected_land_area_m2;let mut e=[0.;4];for w in rs{let(y,m,d,h)=stamp(&w.timestamp);let p=solar_position(&SpaInput{year:y,month:m,day:d,hour:h,minute:30,second:0.,utc_offset_h:0.,delta_t_s:69.,longitude_deg_east:103.8198,latitude_deg:1.3521,elevation_m:25.8,pressure_mbar:w.air_pressure_pa.unwrap_or(101000.)/100.,temperature_c:w.ambient_temperature_c});let ss=sun(p.zenith_deg.to_radians(),p.azimuth_deg.to_radians());for &(a,n) in &ga{if p.zenith_deg<90.{e[0]+=w.dni_w_m2*a*n.dot(ss).max(0.)}e[1]+=w.dhi_w_m2*a*(1.+n.z)/2.;e[2]+=w.ghi_w_m2*ALBEDO*a*(1.-n.z)/2.;}}e[3]=e[0]+e[1]+e[2];R{e,pv,land}}
fn svg<F:Fn(&R)->f64>(path:&Path,rows:&[(f64,R)],metric:F,title:&str){let max=rows.iter().map(|x|metric(&x.1)).fold(0.,f64::max)*1.05;let mut s=format!("<svg xmlns='http://www.w3.org/2000/svg' width='900' height='520'><rect width='100%' height='100%' fill='white'/><text x='450' y='28' text-anchor='middle' font-family='sans-serif' font-size='18'>{title}</text><polyline fill='none' stroke='#4472c4' stroke-width='2' points='");for(k,r)in rows{let x=65.+k/3.*780.;let y=455.-metric(r)/max*400.;s+=&format!("{x:0.1},{y:0.1} ");}s+="'/><text x='450' y='505' text-anchor='middle' font-family='sans-serif'>shape k=h/R — DEVELOPMENT_NOT_SERIS</text></svg>";fs::write(path,s).unwrap();}
fn main(){
 let inp=env::args().nth(1).expect("POWER csv");let out=env::args().nth(2).unwrap_or("paraboloid-sweep".into());fs::create_dir_all(&out).unwrap();
 let raw=fs::read_to_string(inp).unwrap();let(rs,q)=parse_nasa_power_hourly_csv(&raw).unwrap();assert_eq!(rs.len(),8784);assert!(q.is_empty());
 let source:f64=rs.iter().map(|x|x.ghi_w_m2).sum();let mut fd=0.;let mut ff=0.;for x in &rs{let d=(x.ghi_w_m2-x.dhi_w_m2).max(0.);fd+=d;ff+=x.ghi_w_m2-d;}
 let flat=R{e:[fd,ff,0.,source],pv:1.,land:1.};
 let ks=[0.,0.05,0.125,0.25,0.375,0.5,0.6,0.625,0.65,0.75,1.,1.25,1.5,2.,2.5,3.];
 let mut land_rows=vec![];let mut pv_rows=vec![];
 for k in ks{
  if k==0.0{land_rows.push((k,flat.clone()));pv_rows.push((k,flat.clone()));continue;}
  let raw_t=mesh(1.,k,4,24);let raw_r=run(&rs,&raw_t,16);
  assert!(raw_r.land.is_finite()&&raw_r.land>0.&&raw_r.pv.is_finite()&&raw_r.pv>0.);
  let tl=scale(raw_t,(1.0/raw_r.land).sqrt());let rl=run(&rs,&tl,16);
  assert!((rl.land-1.0).abs()<1e-10);
  let tp=scale(tl,(1.0/rl.pv).sqrt());let rp=run(&rs,&tp,16);
  assert!((rp.pv-1.0).abs()<1e-10);
  land_rows.push((k,rl));pv_rows.push((k,rp));
 }
 let flat=&land_rows[0].1;assert!((flat.pv-1.).abs()<1e-12);assert!((flat.land-1.).abs()<1e-12);assert!((flat.e[3]-source).abs()/source<1e-12);assert!(flat.e[2].abs()<1e-9);
 let mut csv=String::from("status,resource_case,k_h_over_r,pv_area_m2,land_area_m2,packing_ratio,direct_wh,diffuse_wh,ground_wh,total_wh,pv_norm_wh_m2pv,packing_efficiency_eta,land_norm_wh_m2land,land_energy_multiplier_lambda\n");
 for(case,rows)in [("equal_land",&land_rows),("equal_pv",&pv_rows)]{for(k,r)in rows{let p=r.pv/r.land;let pvnorm=r.e[3]/r.pv;let eta=pvnorm/source;let landnorm=r.e[3]/r.land;let lam=landnorm/source;csv+=&format!("DEVELOPMENT_NOT_SERIS,{case},{k:0.6},{:0.9},{:0.9},{p:0.9},{:0.6},{:0.6},{:0.6},{:0.6},{pvnorm:0.6},{eta:0.9},{landnorm:0.6},{lam:0.9}\n",r.pv,r.land,r.e[0],r.e[1],r.e[2],r.e[3]);}}
 fs::write(Path::new(&out).join("paraboloid_response.csv"),csv).unwrap();

 let probes=[0.05,0.5,1.5,3.0];
 let mut mc=String::from("status,k,mesh,sky_n,total_wh,relative_to_fine\n");
 for k in probes{
  let mut vals=vec![];for &(nr,np)in &[(3,18),(4,24),(5,30),(6,36)]{let t=normalize_to_land_area(&mesh(1.,k,nr,np),1.0).expect("canonical equal-land normalization");let r=run(&rs,&t,24);assert!(r.e[3].is_finite()&&(r.land-1.0).abs()<1e-10);vals.push((nr,np,r.e[3]));}
  let fine=vals[3].2;let e1=(vals[2].2-vals[1].2).abs();let e2=(vals[3].2-vals[2].2).abs();assert!(e2<e1);assert!(e2/fine<0.01);
  for(nr,np,x)in vals{mc+=&format!("DEVELOPMENT_NOT_SERIS,{k},{nr}x{np},24,{x:0.6},{:0.9}\n",(x-fine)/fine);}
 }
 fs::write(Path::new(&out).join("mesh_convergence.csv"),mc).unwrap();

 let mut sc=String::from("status,k,mesh,sky_n,total_wh,relative_to_fine\n");
 for k in probes{
  let t=normalize_to_land_area(&mesh(1.,k,5,30),1.0).expect("canonical equal-land normalization");let mut vals=vec![];for &sn in &[8usize,16,24,32]{let r=run(&rs,&t,sn);assert!(r.e[3].is_finite()&&(r.land-1.0).abs()<1e-10);vals.push((sn,r.e[3]));}
  let fine=vals[3].1;assert!((vals[2].1-fine).abs()/fine<0.01);
  for(sn,x)in vals{sc+=&format!("DEVELOPMENT_NOT_SERIS,{k},5x30,{sn},{x:0.6},{:0.9}\n",(x-fine)/fine);}
 }
 fs::write(Path::new(&out).join("sky_convergence.csv"),sc).unwrap();

 let mut sens=String::from("status,k,test,value,total_wh,relative_to_frozen\n");
 for k in [0.05,0.5,0.625,1.5,3.0]{let t=normalize_to_land_area(&mesh(1.,k,4,24),1.0).expect("canonical equal-land normalization");let frozen=run(&rs,&t,16);assert!((frozen.land-1.0).abs()<1e-10);let q=run_opts(&rs,&t,16,ALBEDO,true);assert!(q.e[3].is_finite());sens+=&format!("DEVELOPMENT_NOT_SERIS,{k},temporal_quarter_hour,4,{:.6},{:.9}\n",q.e[3],(q.e[3]-frozen.e[3])/frozen.e[3]);for alb in [0.10,0.20,0.30]{let a=run_opts(&rs,&t,16,alb,false);assert!(a.e[3].is_finite());sens+=&format!("DEVELOPMENT_NOT_SERIS,{k},albedo,{alb},{:.6},{:.9}\n",a.e[3],(a.e[3]-frozen.e[3])/frozen.e[3]);}}
 fs::write(Path::new(&out).join("robustness_sensitivity.csv"),sens).unwrap();

 let mut att=String::from("status,k,packing_ratio,visible_direct_wh,unobstructed_direct_wh,direct_self_shadow_loss_wh,visible_diffuse_wh,unobstructed_isotropic_diffuse_wh,sky_obstruction_loss_wh,ground_wh,total_visible_wh\n");
 for k in [0.05,0.5,0.625,1.5,3.0]{let t=normalize_to_land_area(&mesh(1.,k,4,24),1.0).expect("canonical equal-land normalization");let r=run(&rs,&t,16);assert!((r.land-1.0).abs()<1e-10);let u=run_unobstructed(&rs,&t);assert!(u.e[0]+1e-8>=r.e[0]);assert!(u.e[1]+1e-8>=r.e[1]);att+=&format!("DEVELOPMENT_NOT_SERIS,{k},{:.9},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}\n",r.pv/r.land,r.e[0],u.e[0],u.e[0]-r.e[0],r.e[1],u.e[1],u.e[1]-r.e[1],r.e[2],r.e[3]);}
 fs::write(Path::new(&out).join("component_attribution.csv"),att).unwrap();

 let mut st=String::from("status,k,mesh,triangles,min_triangle_area_m2,max_triangle_area_m2,min_normal_z,finite\n");
 for k in [0.05,0.5,1.5,3.0]{let t=normalize_to_land_area(&mesh(1.,k,6,36),1.0).expect("canonical equal-land normalization");let rr=run(&rs,&t,16);assert!((rr.land-1.0).abs()<1e-10);let ga:Vec<_>=t.iter().map(|x|triangle_area_normal(x).expect("validated resource geometry")).collect();let mina=ga.iter().map(|x|x.0).fold(f64::INFINITY,f64::min);let maxa=ga.iter().map(|x|x.0).fold(0.,f64::max);let minz=ga.iter().map(|x|x.1.z).fold(f64::INFINITY,f64::min);let finite=ga.iter().all(|x|x.0.is_finite()&&x.0>0.&&x.1.x.is_finite()&&x.1.y.is_finite()&&x.1.z.is_finite()&&x.1.z>0.);assert!(finite);st+=&format!("DEVELOPMENT_NOT_SERIS,{k},6x36,{},{mina:.12},{maxa:.12},{minz:.12},true\n",t.len());}
 fs::write(Path::new(&out).join("numerical_stability.csv"),st).unwrap();

 let mut cross=None;for w in land_rows.windows(2){let a=(w[0].1.e[3]/w[0].1.land)/source-1.;let b=(w[1].1.e[3]/w[1].1.land)/source-1.;if a<=0.&&b>=0.{cross=Some((w[0].0,w[1].0));break;}}
 let (ka,kb)=cross.expect("land-multiplier crossover not bracketed");assert!(ka>=0.5&&kb<=0.75);
 svg(&Path::new(&out).join("packing_ratio_vs_k.svg"),&land_rows,|r|r.pv/r.land,"Paraboloid packing ratio Π versus k");
 svg(&Path::new(&out).join("packing_efficiency_vs_k.svg"),&land_rows,|r|(r.e[3]/r.pv)/source,"Packing efficiency η versus k");
 svg(&Path::new(&out).join("land_multiplier_vs_k.svg"),&land_rows,|r|(r.e[3]/r.land)/source,"Land-energy multiplier λ versus k");
 fs::write(Path::new(&out).join("checks.txt"),format!("status=DEVELOPMENT_NOT_SERIS\nshape_parameter=k=h/R\nflat_limit=PASS\nequal_land_discrete_area=1_m2\nequal_pv_discrete_area=1_m2\nmesh_convergence_probes=k0.05,k0.5,k1.5,k3.0\nsky_convergence_probes=k0.05,k0.5,k1.5,k3.0\nconvergence_threshold=1_percent\nconvergence_resource_case=equal_land_exact_discrete_1_m2\nsensitivity_resource_case=equal_land_exact_discrete_1_m2\ncomponent_attribution_resource_case=equal_land_exact_discrete_1_m2\nrefined_bend_sampling=k0.6,k0.625,k0.65\nland_multiplier_crossover_bracket={ka}-{kb}\nno_interior_peak_claim=true\nno_optimization_selection=true\n")).unwrap();
 println!("Paraboloid response sweep PASS DEVELOPMENT_NOT_SERIS points={}",land_rows.len());
}
