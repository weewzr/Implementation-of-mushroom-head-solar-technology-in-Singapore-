//! High-performance geometry kernel for the mushroom-head solar project.
//!
//! Coordinate convention: x east, y north, z up. Geometry coordinates are
//! metres in project simulations. Unit vectors are dimensionless.

pub const EPS: f64 = 1.0e-9;
pub const RAY_OFFSET_MULTIPLIER: f64 = 100.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }

impl Vec3 {
    pub fn new(x:f64,y:f64,z:f64)->Self { Self{x,y,z} }
    pub fn add(self,o:Self)->Self { Self::new(self.x+o.x,self.y+o.y,self.z+o.z) }
    pub fn sub(self,o:Self)->Self { Self::new(self.x-o.x,self.y-o.y,self.z-o.z) }
    pub fn mul(self,a:f64)->Self { Self::new(self.x*a,self.y*a,self.z*a) }
    pub fn dot(self,o:Self)->f64 { self.x*o.x+self.y*o.y+self.z*o.z }
    pub fn cross(self,o:Self)->Self { Self::new(self.y*o.z-self.z*o.y,self.z*o.x-self.x*o.z,self.x*o.y-self.y*o.x) }
    pub fn norm(self)->f64 { self.dot(self).sqrt() }
    pub fn unit(self)->Self { self.mul(1.0/self.norm()) }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle { pub v:[Vec3;3] }

impl Triangle {
    pub fn centroid(&self)->Vec3 { self.v[0].add(self.v[1]).add(self.v[2]).mul(1.0/3.0) }
    pub fn cross(&self)->Vec3 { self.v[1].sub(self.v[0]).cross(self.v[2].sub(self.v[0])) }
    pub fn area(&self)->f64 { 0.5*self.cross().norm() }
    pub fn normal(&self)->Vec3 { self.cross().unit() }
}

/// Scalar Moller--Trumbore ray/triangle intersection.
/// Returns positive distance parameter t in geometry-coordinate units.
pub fn ray_triangle_distance(origin:Vec3,direction:Vec3,tri:&Triangle)->Option<f64> {
    let d=direction.unit();
    let v0=tri.v[0];
    let e1=tri.v[1].sub(v0); let e2=tri.v[2].sub(v0);
    let h=d.cross(e2); let a=e1.dot(h);
    if a.abs()<EPS { return None; }
    let f=1.0/a; let s=origin.sub(v0); let u=f*s.dot(h);
    if !(0.0..=1.0).contains(&u) { return None; }
    let q=s.cross(e1); let v=f*d.dot(q);
    if v<0.0 || u+v>1.0 { return None; }
    let t=f*e2.dot(q);
    if t>EPS { Some(t) } else { None }
}

pub fn direct_visibility(index:usize,triangles:&[Triangle],sun:Vec3)->f64 {
    let s=sun.unit();
    let origin=triangles[index].centroid().add(s.mul(EPS*RAY_OFFSET_MULTIPLIER));
    for (j,blocker) in triangles.iter().enumerate() {
        if j!=index && ray_triangle_distance(origin,s,blocker).is_some() { return 0.0; }
    }
    1.0
}

/// Sum A_i V_i max(n_i dot s,0).
/// Units: m^2 when triangle coordinates are metres.
pub fn array_direct_projected_area(triangles:&[Triangle],sun:Vec3)->f64 {
    let s=sun.unit();
    triangles.iter().enumerate().map(|(i,t)| {
        let mu=t.normal().dot(s).max(0.0);
        if mu>0.0 { t.area()*mu*direct_visibility(i,triangles,s) } else { 0.0 }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn horizontal_unit_triangle_area() {
        let t=Triangle{v:[Vec3::new(0.,0.,0.),Vec3::new(1.,0.,0.),Vec3::new(0.,1.,0.)]};
        assert!((t.area()-0.5).abs()<1e-12);
        assert!((t.normal().z-1.0).abs()<1e-12);
    }
    #[test]
    fn stacked_upper_blocks_lower() {
        let low=Triangle{v:[Vec3::new(0.,0.,0.),Vec3::new(1.,0.,0.),Vec3::new(0.,1.,0.)]};
        let high=Triangle{v:[Vec3::new(0.,0.,1.),Vec3::new(1.,0.,1.),Vec3::new(0.,1.,1.)]};
        let tris=[low,high];
        assert_eq!(direct_visibility(0,&tris,Vec3::new(0.,0.,1.)),0.0);
        assert_eq!(direct_visibility(1,&tris,Vec3::new(0.,0.,1.)),1.0);
        assert!((array_direct_projected_area(&tris,Vec3::new(0.,0.,1.))-0.5).abs()<1e-12);
    }
}
