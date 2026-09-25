//! Canonical Rust visibility and isotropic diffuse sky-view foundation.
//! Rays use ENU coordinates. Sky patches are equal-solid-angle midpoint cells.

use std::f64::consts::PI;
use crate::mesh::Vec3;

pub const RAY_EPS:f64=1.0e-9;
#[derive(Clone,Copy,Debug)]
pub struct Triangle{pub v:[Vec3;3]}
impl Triangle{
 pub fn centroid(&self)->Vec3{Vec3{x:(self.v[0].x+self.v[1].x+self.v[2].x)/3.0,y:(self.v[0].y+self.v[1].y+self.v[2].y)/3.0,z:(self.v[0].z+self.v[1].z+self.v[2].z)/3.0}}
}
fn sub(a:Vec3,b:Vec3)->Vec3{Vec3{x:a.x-b.x,y:a.y-b.y,z:a.z-b.z}}
fn add(a:Vec3,b:Vec3)->Vec3{Vec3{x:a.x+b.x,y:a.y+b.y,z:a.z+b.z}}
fn mul(a:Vec3,k:f64)->Vec3{Vec3{x:a.x*k,y:a.y*k,z:a.z*k}}
fn cross(a:Vec3,b:Vec3)->Vec3{Vec3{x:a.y*b.z-a.z*b.y,y:a.z*b.x-a.x*b.z,z:a.x*b.y-a.y*b.x}}

pub fn ray_triangle_distance(origin:Vec3,direction:Vec3,t:&Triangle)->Option<f64>{
 let d=direction.unit();let e1=sub(t.v[1],t.v[0]);let e2=sub(t.v[2],t.v[0]);let h=cross(d,e2);let a=e1.dot(h);
 if a.abs()<RAY_EPS{return None} let f=1.0/a;let s=sub(origin,t.v[0]);let u=f*s.dot(h);if !(0.0..=1.0).contains(&u){return None}
 let q=cross(s,e1);let v=f*d.dot(q);if v<0.0||u+v>1.0{return None} let dist=f*e2.dot(q);if dist>RAY_EPS{Some(dist)}else{None}
}
pub fn visible_from(origin:Vec3,direction:Vec3,triangles:&[Triangle],skip:Option<usize>)->bool{
 let d=direction.unit();let o=add(origin,mul(d,100.0*RAY_EPS));
 !triangles.iter().enumerate().any(|(j,t)|Some(j)!=skip&&ray_triangle_distance(o,d,t).is_some())
}
pub fn direct_visibility(index:usize,triangles:&[Triangle],sun:Vec3)->f64{if visible_from(triangles[index].centroid(),sun,triangles,Some(index)){1.0}else{0.0}}
pub fn direct_beam_visible_w(index:usize,triangles:&[Triangle],normal:Vec3,area_m2:f64,dni:f64,sun:Vec3)->f64{
 assert!(dni>=0.0&&area_m2>0.0);dni*area_m2*normal.unit().dot(sun.unit()).max(0.0)*direct_visibility(index,triangles,sun)
}
#[derive(Clone,Copy,Debug)]pub struct SkyPatch{pub direction:Vec3,pub solid_angle_sr:f64}
pub fn isotropic_sky_patches(n_mu:usize,n_phi:usize)->Vec<SkyPatch>{
 assert!(n_mu>0&&n_phi>=4);let dmu=1.0/n_mu as f64;let dphi=2.0*PI/n_phi as f64;let mut out=Vec::with_capacity(n_mu*n_phi);
 for i in 0..n_mu{let mu=(i as f64+0.5)*dmu;let st=(1.0-mu*mu).sqrt();for j in 0..n_phi{let ph=(j as f64+0.5)*dphi;out.push(SkyPatch{direction:Vec3{x:st*ph.cos(),y:st*ph.sin(),z:mu},solid_angle_sr:dmu*dphi});}}out
}
/// Dimensionless isotropic sky factor: (1/pi) integral_visible max(n.s,0) dOmega.
pub fn sky_view_factor(origin:Vec3,normal:Vec3,triangles:&[Triangle],skip:Option<usize>,n_mu:usize,n_phi:usize)->f64{
 let n=normal.unit();isotropic_sky_patches(n_mu,n_phi).iter().map(|p|if visible_from(origin,p.direction,triangles,skip){n.dot(p.direction).max(0.0)*p.solid_angle_sr/PI}else{0.0}).sum()
}
#[cfg(test)]mod tests{use super::*;
 fn v(x:f64,y:f64,z:f64)->Vec3{Vec3{x,y,z}}
 fn tri(z:f64)->Triangle{Triangle{v:[v(-1.,-1.,z),v(1.,-1.,z),v(0.,1.,z)]}}
 #[test]fn stacked_shadow_and_escape(){let ts=[tri(0.),tri(1.)];assert_eq!(direct_visibility(0,&ts,v(0.,0.,1.)),0.0);assert_eq!(direct_visibility(1,&ts,v(0.,0.,1.)),1.0);assert_eq!(direct_visibility(0,&ts,v(1.,0.,0.)),1.0);}
 #[test]fn beam_visibility_conserves_unblocked_and_blocks_shadow(){let ts=[tri(0.),tri(1.)];let n=v(0.,0.,1.);assert!((direct_beam_visible_w(1,&ts,n,0.5,800.,v(0.,0.,1.))-400.).abs()<1e-12);assert_eq!(direct_beam_visible_w(0,&ts,n,0.5,800.,v(0.,0.,1.)),0.0);}
 #[test]fn sky_patch_solid_angle_is_hemisphere(){for q in [4,8,16,32]{let s:f64=isotropic_sky_patches(q,4*q).iter().map(|p|p.solid_angle_sr).sum();assert!((s-2.0*PI).abs()<1e-12);}}
 #[test]fn unobstructed_horizontal_sky_is_exact(){for q in [2,4,8,16]{let f=sky_view_factor(v(0.,0.,0.),v(0.,0.,1.),&[],None,q,4*q);assert!((f-1.0).abs()<1e-12);}}
 #[test]fn unobstructed_vertical_converges_to_half(){let mut prev=1.0;for q in [4,8,16,32]{let f=sky_view_factor(v(0.,0.,0.),v(1.,0.,0.),&[],None,q,4*q);let e=(f-0.5).abs();assert!(e<prev);prev=e;}assert!(prev<5e-4);}
 #[test]fn full_overhead_plane_blocks_upward_sky(){let roof=[Triangle{v:[v(-100.,-100.,1.),v(100.,-100.,1.),v(100.,100.,1.)]},Triangle{v:[v(-100.,-100.,1.),v(100.,100.,1.),v(-100.,100.,1.)]}];let f=sky_view_factor(v(0.,0.,0.),v(0.,0.,1.),&roof,None,16,64);assert!(f<1e-12);}
 #[test]fn finite_roof_sky_view_converges(){let roof=[Triangle{v:[v(-1.,-1.,1.),v(1.,-1.,1.),v(1.,1.,1.)]},Triangle{v:[v(-1.,-1.,1.),v(1.,1.,1.),v(-1.,1.,1.)]}];let f1=sky_view_factor(v(0.,0.,0.),v(0.,0.,1.),&roof,None,8,32);let f2=sky_view_factor(v(0.,0.,0.),v(0.,0.,1.),&roof,None,16,64);let f3=sky_view_factor(v(0.,0.,0.),v(0.,0.,1.),&roof,None,32,128);assert!((f3-f2).abs()<(f2-f1).abs());assert!((0.0..1.0).contains(&f3));}
}