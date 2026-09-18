//! Minimal facet representation shared by baseline geometries.
//!
//! Coordinate order is canonical ENU: x=east, y=north, z=up. Vector angles
//! used by higher-level constructors must be radians. See
//! docs/coordinate_conventions.md.
#[derive(Debug, Clone, Copy)] pub struct Vec3 { pub x:f64, pub y:f64, pub z:f64 }
impl Vec3 { pub fn dot(self,o:Self)->f64{self.x*o.x+self.y*o.y+self.z*o.z} pub fn norm(self)->f64{self.dot(self).sqrt()} pub fn unit(self)->Self{let n=self.norm(); assert!(n>0.0); Self{x:self.x/n,y:self.y/n,z:self.z/n}} }
#[derive(Debug, Clone, Copy)] pub struct Facet { pub area_m2:f64, pub normal:Vec3, pub centroid_m:Vec3 }
impl Facet { pub fn new(area_m2:f64,normal:Vec3,centroid_m:Vec3)->Self{assert!(area_m2>0.0);Self{area_m2,normal:normal.unit(),centroid_m}} }
/// Beam power intercepted before PV conversion and visibility/shading losses.
pub fn direct_beam_intercept_w(facet:&Facet,dni_w_m2:f64,sun:Vec3)->f64{ assert!(dni_w_m2>=0.0); dni_w_m2*facet.area_m2*facet.normal.dot(sun.unit()).max(0.0) }
#[cfg(test)] mod tests { use super::*;
 #[test] fn normal_incidence(){let f=Facet::new(2.0,Vec3{x:0.0,y:0.0,z:1.0},Vec3{x:0.0,y:0.0,z:0.0}); assert!((direct_beam_intercept_w(&f,1000.0,Vec3{x:0.0,y:0.0,z:1.0})-2000.0).abs()<1e-12);}
 #[test] fn grazing_incidence_zero(){let f=Facet::new(1.0,Vec3{x:0.0,y:0.0,z:1.0},Vec3{x:0.0,y:0.0,z:0.0}); assert!(direct_beam_intercept_w(&f,1000.0,Vec3{x:1.0,y:0.0,z:0.0}).abs()<1e-12);}
 #[test] fn backside_zero(){let f=Facet::new(1.0,Vec3{x:0.0,y:0.0,z:1.0},Vec3{x:0.0,y:0.0,z:0.0}); assert_eq!(direct_beam_intercept_w(&f,1000.0,Vec3{x:0.0,y:0.0,z:-1.0}),0.0);}
}