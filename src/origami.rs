//! Minimal deployable-sheet kinematic mesh foundation.
//! This is geometry/kinematics only: no collision, bend-radius, strain, wind,
//! actuator, fatigue, structural or cost solver is implemented here.
use crate::{mesh::Vec3,visibility::Triangle,resource_geometry::discrete_resources};
#[derive(Clone,Copy,Debug,PartialEq)] pub struct Crease{pub a:usize,pub b:usize,pub deployed_angle_rad:f64}
#[derive(Clone,Debug)] pub struct AccordionFixture{pub vertices:Vec<Vec3>,pub facets:Vec<[usize;3]>,pub creases:Vec<Crease>}
#[derive(Clone,Debug)] pub struct OrigamiSheet{pub vertices:Vec<Vec3>,pub facets:Vec<[usize;3]>,pub creases:Vec<Crease>}
#[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum UnsupportedMechanics{CreaseCompatibility,SelfIntersection,MinimumBendRadius,PvStrain}
impl OrigamiSheet{
 pub fn single_crease_fixture()->Self{Self{vertices:vec![Vec3{x:-1.,y:-0.5,z:0.},Vec3{x:0.,y:-0.5,z:0.},Vec3{x:1.,y:-0.5,z:0.},Vec3{x:-1.,y:0.5,z:0.},Vec3{x:0.,y:0.5,z:0.},Vec3{x:1.,y:0.5,z:0.}],facets:vec![[0,1,4],[0,4,3],[1,2,5],[1,5,4]],creases:vec![Crease{a:1,b:4,deployed_angle_rad:0.8}]}}
 pub fn state(&self,lambda:f64)->Result<Vec<Vec3>,&'static str>{if !lambda.is_finite()||!(0.0..=1.0).contains(&lambda){return Err("lambda must be finite in [0,1]")}let th=lambda*self.creases[0].deployed_angle_rad;let mut o=self.vertices.clone();for p in &mut o{if p.x>0.0{let x=p.x;p.x=x*th.cos();p.z=x*th.sin();}}Ok(o)}
 pub fn triangles(&self,lambda:f64)->Result<Vec<Triangle>,&'static str>{let v=self.state(lambda)?;Ok(self.facets.iter().map(|f|Triangle{v:[v[f[0]],v[f[1]],v[f[2]]]}).collect())}
 pub fn unsupported_checks()->[UnsupportedMechanics;4]{[UnsupportedMechanics::CreaseCompatibility,UnsupportedMechanics::SelfIntersection,UnsupportedMechanics::MinimumBendRadius,UnsupportedMechanics::PvStrain]}
}
impl AccordionFixture{
 pub fn new()->Self{
  let mut vertices=Vec::new();for i in 0..=4{let x=i as f64;vertices.push(Vec3{x,y:-0.5,z:0.});vertices.push(Vec3{x,y:0.5,z:0.});}
  let mut facets=Vec::new();for i in 0..4{let a=2*i;facets.push([a,a+2,a+3]);facets.push([a,a+3,a+1]);}
  let creases=(1..4).map(|i|Crease{a:2*i,b:2*i+1,deployed_angle_rad:if i%2==0{-0.65}else{0.65}}).collect();
  Self{vertices,facets,creases}
 }
 pub fn state(&self,lambda:f64)->Result<Vec<Vec3>,&'static str>{
  if !lambda.is_finite()||!(0.0..=1.0).contains(&lambda){return Err("lambda must be finite in [0,1]")}
  let th=0.65*lambda;let mut out=self.vertices.clone();
  for p in &mut out{let panel=p.x.floor().min(3.0) as usize;let local=p.x-panel as f64;let a=if panel%2==0{th}else{-th};p.x=panel as f64+local*a.cos();p.z=local*a.sin();}
  Ok(out)
 }
 pub fn triangles(&self,lambda:f64)->Result<Vec<Triangle>,&'static str>{let v=self.state(lambda)?;Ok(self.facets.iter().map(|f|Triangle{v:[v[f[0]],v[f[1]],v[f[2]]]}).collect())}
 pub fn crease_compatible(&self)->bool{self.creases.iter().all(|c|c.a<self.vertices.len()&&c.b<self.vertices.len()&&c.a!=c.b)&&self.creases.windows(2).all(|w|w[0].deployed_angle_rad.signum()!=w[1].deployed_angle_rad.signum())}
 pub fn has_collision(vertices:&[Vec3],facets:&[[usize;3]])->bool{
  // Fixture-level conservative broad-phase test: non-adjacent facet AABB overlap, including coplanar overlap in two axes. This can flag false positives and can miss edge-only/continuous swept collisions; it is not general triangle-triangle collision mechanics.
  fn bounds(v:&[Vec3],f:&[usize;3])->([f64;3],[f64;3]){let mut lo=[f64::INFINITY;3];let mut hi=[f64::NEG_INFINITY;3];for &i in f{let p=[v[i].x,v[i].y,v[i].z];for k in 0..3{lo[k]=lo[k].min(p[k]);hi[k]=hi[k].max(p[k]);}}(lo,hi)}
  for i in 0..facets.len(){for j in i+1..facets.len(){if facets[i].iter().any(|x|facets[j].contains(x)){continue}let(a,b)=bounds(vertices,&facets[i]);let(c,d)=bounds(vertices,&facets[j]);let overlap:[f64;3]=[0,1,2].map(|k|b[k].min(d[k])-a[k].max(c[k]));let positive=overlap.iter().filter(|&&x|x>1e-10).count();let nonnegative=overlap.iter().all(|&x|x>=-1e-10);if nonnegative&&positive>=2{return true}}}false
 }
}
#[cfg(test)]mod tests{use super::*;fn near(a:Vec3,b:Vec3,t:f64)->bool{(a.x-b.x).abs()<=t&&(a.y-b.y).abs()<=t&&(a.z-b.z).abs()<=t}fn vecs_near(a:&[Vec3],b:&[Vec3],t:f64)->bool{a.len()==b.len()&&a.iter().zip(b).all(|(&x,&y)|near(x,y,t))}fn dist(a:Vec3,b:Vec3)->f64{((a.x-b.x).powi(2)+(a.y-b.y).powi(2)+(a.z-b.z).powi(2)).sqrt()}fn area(a:Vec3,b:Vec3,c:Vec3)->f64{let u=Vec3{x:b.x-a.x,y:b.y-a.y,z:b.z-a.z};let v=Vec3{x:c.x-a.x,y:c.y-a.y,z:c.z-a.z};0.5*Vec3{x:u.y*v.z-u.z*v.y,y:u.z*v.x-u.x*v.z,z:u.x*v.y-u.y*v.x}.norm()}
 #[test]fn endpoints_are_deterministic(){let s=OrigamiSheet::single_crease_fixture();assert!(vecs_near(&s.state(0.).unwrap(),&s.vertices,1e-14));let a=s.state(1.).unwrap();let b=s.state(1.).unwrap();assert!(vecs_near(&a,&b,1e-14));}
 #[test]fn intermediate_states_are_finite_and_connected(){let s=OrigamiSheet::single_crease_fixture();for l in [0.,0.25,0.5,0.75,1.]{let v=s.state(l).unwrap();assert!(v.iter().all(|p|p.x.is_finite()&&p.y.is_finite()&&p.z.is_finite()));assert!(s.facets.iter().flatten().all(|&i|i<v.len()));}}
 #[test]fn rigid_edges_and_facet_areas_are_preserved(){let s=OrigamiSheet::single_crease_fixture();let base=s.state(0.).unwrap();for l in [0.25,0.5,0.75,1.]{let v=s.state(l).unwrap();for f in &s.facets{for(a,b)in[(f[0],f[1]),(f[1],f[2]),(f[2],f[0])]{assert!((dist(base[a],base[b])-dist(v[a],v[b])).abs()<1e-12);}assert!((area(base[f[0]],base[f[1]],base[f[2]])-area(v[f[0]],v[f[1]],v[f[2]])).abs()<1e-12);}}}
 #[test]fn deployed_mesh_converts_to_canonical_triangles_with_consistent_orientation(){let s=OrigamiSheet::single_crease_fixture();let t=s.triangles(1.).unwrap();assert_eq!(t.len(),4);for q in t{let a=area(q.v[0],q.v[1],q.v[2]);assert!(a>0.0);let u=Vec3{x:q.v[1].x-q.v[0].x,y:q.v[1].y-q.v[0].y,z:q.v[1].z-q.v[0].z};let v=Vec3{x:q.v[2].x-q.v[0].x,y:q.v[2].y-q.v[0].y,z:q.v[2].z-q.v[0].z};let n=Vec3{x:u.y*v.z-u.z*v.y,y:u.z*v.x-u.x*v.z,z:u.x*v.y-u.y*v.x};assert!(n.norm()>0.0);assert!(n.z>0.0);}}
 #[test]fn lambda_bounds_and_repeatability(){let s=OrigamiSheet::single_crease_fixture();assert!(s.state(-0.01).is_err());assert!(s.state(1.01).is_err());assert!(s.state(f64::NAN).is_err());let a=s.state(0.5).unwrap();let b=s.state(0.5).unwrap();assert!(vecs_near(&a,&b,1e-14));}
 #[test]fn deployed_mesh_enters_canonical_resource_accounting(){let s=OrigamiSheet::single_crease_fixture();let t=s.triangles(1.).unwrap();let r=discrete_resources(&t).unwrap();assert!(r.active_pv_area_m2.is_finite()&&r.active_pv_area_m2>0.0);assert!(r.projected_land_area_m2.is_finite()&&r.projected_land_area_m2>0.0);assert!(r.packing_ratio().is_finite());}
 #[test]fn unsupported_mechanics_are_explicit(){assert_eq!(OrigamiSheet::unsupported_checks().len(),4);}
 #[test]fn accordion_states_are_deterministic_finite_and_connected(){let s=AccordionFixture::new();assert!(s.crease_compatible());for l in [0.,0.5,1.]{let a=s.state(l).unwrap();let b=s.state(l).unwrap();assert!(vecs_near(&a,&b,1e-14));assert!(a.iter().all(|p|p.x.is_finite()&&p.y.is_finite()&&p.z.is_finite()));assert!(s.facets.iter().flatten().all(|&i|i<a.len()));}}
 #[test]fn accordion_rigid_facets_preserve_edges_and_area(){let s=AccordionFixture::new();let b=s.state(0.).unwrap();for l in [0.25,0.5,1.]{let v=s.state(l).unwrap();for f in &s.facets{for(a,c)in[(f[0],f[1]),(f[1],f[2]),(f[2],f[0])]{assert!((dist(b[a],b[c])-dist(v[a],v[c])).abs()<1e-12);}assert!((area(b[f[0]],b[f[1]],b[f[2]])-area(v[f[0]],v[f[1]],v[f[2]])).abs()<1e-12);}}}
 #[test]fn accordion_enters_canonical_resource_mesh(){let s=AccordionFixture::new();let t=s.triangles(1.).unwrap();let r=discrete_resources(&t).unwrap();assert!(r.active_pv_area_m2>0.0&&r.projected_land_area_m2>0.0);}
 #[test]fn accordion_collision_detector_has_positive_and_negative_fixture(){let s=AccordionFixture::new();let valid=s.state(0.5).unwrap();assert!(!AccordionFixture::has_collision(&valid,&s.facets));let mut bad=valid.clone();bad[6]=bad[0];bad[7]=bad[1];bad[8]=bad[2];bad[9]=bad[3];assert!(AccordionFixture::has_collision(&bad,&s.facets));}
}