//! Common fixed-geometry mesh generation and matched-resource contracts.
//! Candidate generators own topology only; resource accounting is canonical.

use std::f64::consts::PI;
use crate::{mesh::Vec3,resource_geometry::{discrete_resources,normalize_to_land_area,normalize_to_pv_area,DiscreteResources,ResourceGeometryError},visibility::Triangle};

#[derive(Debug,Clone,Copy,PartialEq)] pub enum FixedGeometry{Flat,Paraboloid{height_over_radius:f64},Hemisphere,FacetedCanopy{height_over_radius:f64,facets:usize},FoldedSurface{tilt_rad:f64}}
#[derive(Debug,Clone,Copy,PartialEq)] pub enum ComparisonContract{EqualLand{land_m2:f64},EqualPv{pv_m2:f64},MatchedPackingRatio{packing_ratio:f64,land_m2:f64}}
fn v(x:f64,y:f64,z:f64)->Vec3{Vec3{x,y,z}}
fn fan_surface<F:Fn(f64,f64)->Vec3>(nr:usize,np:usize,p:F)->Vec<Triangle>{assert!(nr>=2&&np>=8);let mut o=vec![];let cen=p(0.0,0.0);let r1=1.0/nr as f64;for j in 0..np{let a=2.0*PI*j as f64/np as f64;let b=2.0*PI*(j+1)as f64/np as f64;o.push(Triangle{v:[cen,p(r1,a),p(r1,b)]});}for i in 1..nr{let ra=i as f64/nr as f64;let rb=(i+1)as f64/nr as f64;for j in 0..np{let a=2.0*PI*j as f64/np as f64;let b=2.0*PI*(j+1)as f64/np as f64;let q=[p(ra,a),p(rb,a),p(rb,b),p(ra,b)];o.push(Triangle{v:[q[0],q[1],q[2]]});o.push(Triangle{v:[q[0],q[2],q[3]]});}}o}
pub fn generate_candidate(g:FixedGeometry,nr:usize,np:usize)->Result<Vec<Triangle>,ResourceGeometryError>{
 let r=(1.0/PI).sqrt();
 let m=match g{
  FixedGeometry::Flat=>fan_surface(nr,np,|u,p|v(r*u*p.cos(),r*u*p.sin(),0.0)),
  FixedGeometry::Paraboloid{height_over_radius:k}=>{if !k.is_finite()||k<0.0{return Err(ResourceGeometryError::InvalidTarget)}let h=k*r;fan_surface(nr,np,|u,p|v(r*u*p.cos(),r*u*p.sin(),h*(1.0-u*u)))},
  FixedGeometry::Hemisphere=>fan_surface(nr,np,|u,p|v(r*u*p.cos(),r*u*p.sin(),r*(1.0-u*u).sqrt())),
  FixedGeometry::FacetedCanopy{height_over_radius:k,facets}=>{if !k.is_finite()||k<0.0||facets<4{return Err(ResourceGeometryError::InvalidTarget)}let h=k*r;fan_surface(nr,facets,|u,p|v(r*u*p.cos(),r*u*p.sin(),h*(1.0-u)))},
  FixedGeometry::FoldedSurface{tilt_rad:t}=>{if !t.is_finite()||t<0.0||t>=PI/2.0{return Err(ResourceGeometryError::InvalidTarget)}let w=0.5;let l=1.0;let z=w*t.tan();vec![Triangle{v:[v(-w,-l/2.0,z),v(0.0,-l/2.0,0.0),v(0.0,l/2.0,0.0)]},Triangle{v:[v(-w,-l/2.0,z),v(0.0,l/2.0,0.0),v(-w,l/2.0,z)]},Triangle{v:[v(0.0,-l/2.0,0.0),v(w,-l/2.0,z),v(w,l/2.0,z)]},Triangle{v:[v(0.0,-l/2.0,0.0),v(w,l/2.0,z),v(0.0,l/2.0,0.0)]}]}
 };discrete_resources(&m)?;Ok(m)
}
pub fn apply_contract(mesh:&[Triangle],c:ComparisonContract)->Result<Vec<Triangle>,ResourceGeometryError>{match c{
 ComparisonContract::EqualLand{land_m2}=>normalize_to_land_area(mesh,land_m2),
 ComparisonContract::EqualPv{pv_m2}=>normalize_to_pv_area(mesh,pv_m2),
 ComparisonContract::MatchedPackingRatio{packing_ratio,land_m2}=>{if !packing_ratio.is_finite()||packing_ratio<=0.0{return Err(ResourceGeometryError::InvalidTarget)}let r=discrete_resources(mesh)?;if (r.packing_ratio()-packing_ratio).abs()>1e-8{return Err(ResourceGeometryError::PostconditionFailed)}normalize_to_land_area(mesh,land_m2)}
}}
pub fn resources(mesh:&[Triangle])->Result<DiscreteResources,ResourceGeometryError>{discrete_resources(mesh)}

#[cfg(test)]mod tests{use super::*;use crate::resource_geometry::{triangle_area_normal,RESOURCE_AREA_TOL_M2};
 fn families()->[FixedGeometry;5]{[FixedGeometry::Flat,FixedGeometry::Paraboloid{height_over_radius:0.5},FixedGeometry::Hemisphere,FixedGeometry::FacetedCanopy{height_over_radius:0.5,facets:24},FixedGeometry::FoldedSurface{tilt_rad:0.4}]}
 #[test]fn all_families_are_finite_upward_and_positive(){for g in families(){let m=generate_candidate(g,5,30).unwrap();let r=resources(&m).unwrap();assert!(r.active_pv_area_m2.is_finite()&&r.active_pv_area_m2>0.0&&r.projected_land_area_m2>0.0);for t in &m{let(a,n)=triangle_area_normal(t).unwrap();assert!(a>0.0&&n.z>0.0);}}}
 #[test]fn equal_resource_contracts_are_exact(){for g in families(){let m=generate_candidate(g,5,30).unwrap();let l=apply_contract(&m,ComparisonContract::EqualLand{land_m2:1.0}).unwrap();assert!((resources(&l).unwrap().projected_land_area_m2-1.0).abs()<=RESOURCE_AREA_TOL_M2);let p=apply_contract(&m,ComparisonContract::EqualPv{pv_m2:1.0}).unwrap();assert!((resources(&p).unwrap().active_pv_area_m2-1.0).abs()<=RESOURCE_AREA_TOL_M2);}}
 #[test]fn flat_limit_has_unit_packing(){let m=generate_candidate(FixedGeometry::Flat,6,36).unwrap();assert!((resources(&m).unwrap().packing_ratio()-1.0).abs()<1e-12);}
 #[test]fn hemisphere_area_ratio_converges_to_two(){let a=resources(&generate_candidate(FixedGeometry::Hemisphere,8,48).unwrap()).unwrap().packing_ratio();let b=resources(&generate_candidate(FixedGeometry::Hemisphere,16,96).unwrap()).unwrap().packing_ratio();assert!((2.0-b).abs()<(2.0-a).abs());assert!((b-2.0).abs()<0.01);}
 #[test]fn paraboloid_shallow_limit_approaches_flat(){let p=resources(&generate_candidate(FixedGeometry::Paraboloid{height_over_radius:1e-4},6,36).unwrap()).unwrap().packing_ratio();assert!((p-1.0).abs()<1e-6);}
 #[test]fn folded_packing_matches_secant(){let t=0.4;let p=resources(&generate_candidate(FixedGeometry::FoldedSurface{tilt_rad:t},2,8).unwrap()).unwrap().packing_ratio();assert!((p-1.0/t.cos()).abs()<1e-12);}
 #[test]fn matched_packing_rejects_shape_mismatch(){let m=generate_candidate(FixedGeometry::Flat,4,24).unwrap();assert!(matches!(apply_contract(&m,ComparisonContract::MatchedPackingRatio{packing_ratio:2.0,land_m2:1.0}),Err(ResourceGeometryError::PostconditionFailed)));}
}
