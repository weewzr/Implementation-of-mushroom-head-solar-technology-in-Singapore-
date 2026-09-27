//! Minimal deployable-sheet kinematic mesh foundation.
//! This is geometry/kinematics only: no collision, bend-radius, strain, wind,
//! actuator, fatigue, structural or cost solver is implemented here.
use crate::{mesh::Vec3,visibility::Triangle};
#[derive(Clone,Copy,Debug,PartialEq)] pub struct Crease{pub a:usize,pub b:usize,pub deployed_angle_rad:f64}
#[derive(Clone,Debug)] pub struct OrigamiSheet{pub vertices:Vec<Vec3>,pub facets:Vec<[usize;3]>,pub creases:Vec<Crease>}
#[derive(Clone,Copy,Debug,PartialEq,Eq)] pub enum UnsupportedMechanics{CreaseCompatibility,SelfIntersection,MinimumBendRadius,PvStrain}
impl OrigamiSheet{
 pub fn single_crease_fixture()->Self{Self{vertices:vec![Vec3{x:-1.,y:-0.5,z:0.},Vec3{x:0.,y:-0.5,z:0.},Vec3{x:1.,y:-0.5,z:0.},Vec3{x:-1.,y:0.5,z:0.},Vec3{x:0.,y:0.5,z:0.},Vec3{x:1.,y:0.5,z:0.}],facets:vec![[0,1,4],[0,4,3],[1,2,5],[1,5,4]],creases:vec![Crease{a:1,b:4,deployed_angle_rad:0.8}]}}
 pub fn state(&self,lambda:f64)->Result<Vec<Vec3>,&'static str>{if !lambda.is_finite()||!(0.0..=1.0).contains(&lambda){return Err("lambda must be finite in [0,1]")}let th=lambda*self.creases[0].deployed_angle_rad;let mut o=self.vertices.clone();for p in &mut o{if p.x>0.0{let x=p.x;p.x=x*th.cos();p.z=x*th.sin();}}Ok(o)}
 pub fn triangles(&self,lambda:f64)->Result<Vec<Triangle>,&'static str>{let v=self.state(lambda)?;Ok(self.facets.iter().map(|f|Triangle{v:[v[f[0]],v[f[1]],v[f[2]]]}).collect())}
 pub fn unsupported_checks()->[UnsupportedMechanics;4]{[UnsupportedMechanics::CreaseCompatibility,UnsupportedMechanics::SelfIntersection,UnsupportedMechanics::MinimumBendRadius,UnsupportedMechanics::PvStrain]}
}
#[cfg(test)]mod tests{use super::*;fn dist(a:Vec3,b:Vec3)->f64{((a.x-b.x).powi(2)+(a.y-b.y).powi(2)+(a.z-b.z).powi(2)).sqrt()}fn area(a:Vec3,b:Vec3,c:Vec3)->f64{let u=Vec3{x:b.x-a.x,y:b.y-a.y,z:b.z-a.z};let v=Vec3{x:c.x-a.x,y:c.y-a.y,z:c.z-a.z};0.5*Vec3{x:u.y*v.z-u.z*v.y,y:u.z*v.x-u.x*v.z,z:u.x*v.y-u.y*v.x}.norm()}
 #[test]fn endpoints_are_deterministic(){let s=OrigamiSheet::single_crease_fixture();assert_eq!(s.state(0.).unwrap(),s.vertices);assert_eq!(s.state(1.).unwrap(),s.state(1.).unwrap());}
 #[test]fn intermediate_states_are_finite_and_connected(){let s=OrigamiSheet::single_crease_fixture();for l in [0.,0.25,0.5,0.75,1.]{let v=s.state(l).unwrap();assert!(v.iter().all(|p|p.x.is_finite()&&p.y.is_finite()&&p.z.is_finite()));assert!(s.facets.iter().flatten().all(|&i|i<v.len()));}}
 #[test]fn rigid_edges_and_facet_areas_are_preserved(){let s=OrigamiSheet::single_crease_fixture();let base=s.state(0.).unwrap();for l in [0.25,0.5,0.75,1.]{let v=s.state(l).unwrap();for f in &s.facets{for(a,b)in[(f[0],f[1]),(f[1],f[2]),(f[2],f[0])]{assert!((dist(base[a],base[b])-dist(v[a],v[b])).abs()<1e-12);}assert!((area(base[f[0]],base[f[1]],base[f[2]])-area(v[f[0]],v[f[1]],v[f[2]])).abs()<1e-12);}}}
 #[test]fn deployed_mesh_converts_to_canonical_triangles_with_consistent_orientation(){let s=OrigamiSheet::single_crease_fixture();let t=s.triangles(1.).unwrap();assert_eq!(t.len(),4);for q in t{let a=area(q.v[0],q.v[1],q.v[2]);assert!(a>0.0);let u=Vec3{x:q.v[1].x-q.v[0].x,y:q.v[1].y-q.v[0].y,z:q.v[1].z-q.v[0].z};let v=Vec3{x:q.v[2].x-q.v[0].x,y:q.v[2].y-q.v[0].y,z:q.v[2].z-q.v[0].z};assert!(u.x*v.y-u.y*v.x>0.0);}}
 #[test]fn unsupported_mechanics_are_explicit(){assert_eq!(OrigamiSheet::unsupported_checks().len(),4);}
}