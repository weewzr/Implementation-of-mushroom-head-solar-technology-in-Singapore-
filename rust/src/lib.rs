//! High-performance geometry kernel for the mushroom-head solar project.
//!
//! Coordinate convention: x east, y north, z up. Geometry coordinates are
//! metres in project simulations. Unit vectors are dimensionless.

use std::f64::consts::PI;

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

pub fn ray_triangle_distance(origin:Vec3,direction:Vec3,tri:&Triangle)->Option<f64> {
    let d=direction.unit(); let v0=tri.v[0];
    let e1=tri.v[1].sub(v0); let e2=tri.v[2].sub(v0);
    let h=d.cross(e2); let a=e1.dot(h);
    if a.abs()<EPS { return None; }
    let f=1.0/a; let q0=origin.sub(v0); let u=f*q0.dot(h);
    if !(0.0..=1.0).contains(&u) { return None; }
    let q=q0.cross(e1); let v=f*d.dot(q);
    if v<0.0 || u+v>1.0 { return None; }
    let t=f*e2.dot(q); if t>EPS { Some(t) } else { None }
}

pub fn direct_visibility(index:usize,triangles:&[Triangle],sun:Vec3)->f64 {
    let s=sun.unit();
    let origin=triangles[index].centroid().add(s.mul(EPS*RAY_OFFSET_MULTIPLIER));
    for (j,blocker) in triangles.iter().enumerate() {
        if j!=index && ray_triangle_distance(origin,s,blocker).is_some() { return 0.0; }
    }
    1.0
}

/// Sum A_i V_i max(n_i dot s,0). Units: m^2 when coordinates are metres.
pub fn array_direct_projected_area(triangles:&[Triangle],sun:Vec3)->f64 {
    let s=sun.unit();
    triangles.iter().enumerate().map(|(i,t)| {
        let mu=t.normal().dot(s).max(0.0);
        if mu>0.0 { t.area()*mu*direct_visibility(i,triangles,s) } else { 0.0 }
    }).sum()
}

/// Analytical paraboloid packing ratio Pi(k), with k=h/R dimensionless.
///
/// Exact form: Pi=[(1+4k^2)^(3/2)-1]/(6k^2).
/// For small |k|, direct subtraction loses precision. The binomial expansion
/// Pi=1+k^2-(2/3)k^4+k^6+O(k^8) is used below a numerical switch threshold.
pub fn paraboloid_packing(k:f64)->f64 {
    let k2=k*k;
    // 1e-3 is a numerical implementation switch, not a physical/design value.
    // At |k|=1e-3 the first omitted term is O(k^8)=O(1e-24), far below f64
    // relative precision for a quantity near unity.
    if k.abs()<1.0e-3 {
        return 1.0+k2-(2.0/3.0)*k2*k2+k2*k2*k2;
    }
    ((1.0+4.0*k2).powf(1.5)-1.0)/(6.0*k2)
}

/// Invert Pi(k) by bisection. 100 iterations is a numerical setting.
pub fn paraboloid_k_from_packing(pi_target:f64)->f64 {
    assert!(pi_target>=1.0);
    if (pi_target-1.0).abs()<1.0e-12 { return 0.0; }
    let (mut lo,mut hi)=(0.0,16.0);
    for _ in 0..100 {
        let mid=0.5*(lo+hi);
        if paraboloid_packing(mid)<pi_target { lo=mid; } else { hi=mid; }
    }
    0.5*(lo+hi)
}

pub fn paraboloid_mesh(packing_ratio:f64,footprint_area:f64,n_r:usize,n_phi:usize)->Vec<Triangle> {
    assert!(n_r>=1 && n_phi>=3);
    let radius=(footprint_area/PI).sqrt();
    let k=paraboloid_k_from_packing(packing_ratio); let height=k*radius;
    let point=|r:f64,phi:f64| Vec3::new(r*phi.cos(),r*phi.sin(),height*(1.0-(r/radius).powi(2)));
    let mut tris=Vec::with_capacity(n_phi*(2*n_r-1));
    let centre=Vec3::new(0.0,0.0,height); let r1=radius/n_r as f64;
    for j in 0..n_phi {
        let p0=2.0*PI*j as f64/n_phi as f64; let p1=2.0*PI*(j+1) as f64/n_phi as f64;
        tris.push(Triangle{v:[centre,point(r1,p0),point(r1,p1)]});
    }
    for ir in 1..n_r {
        let ra=radius*ir as f64/n_r as f64; let rb=radius*(ir+1) as f64/n_r as f64;
        for j in 0..n_phi {
            let p0=2.0*PI*j as f64/n_phi as f64; let p1=2.0*PI*(j+1) as f64/n_phi as f64;
            let a=point(ra,p0); let b=point(rb,p0); let c=point(rb,p1); let d=point(ra,p1);
            tris.push(Triangle{v:[a,b,c]}); tris.push(Triangle{v:[a,c,d]});
        }
    }
    tris
}

pub fn cooper_declination_rad(n:u32)->f64 {
    let arg_deg=(360.0/365.0)*(284.0+n as f64);
    23.45_f64.to_radians()*arg_deg.to_radians().sin()
}

pub fn solar_vector_preliminary(latitude_deg:f64,n:u32,solar_time_h:f64)->Vec3 {
    let phi=latitude_deg.to_radians(); let delta=cooper_declination_rad(n);
    let h=(15.0*(solar_time_h-12.0)).to_radians();
    let east=-delta.cos()*h.sin();
    let north=delta.sin()*phi.cos()-delta.cos()*phi.sin()*h.cos();
    let up=phi.sin()*delta.sin()+phi.cos()*delta.cos()*h.cos();
    Vec3::new(east,north,up).unit()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn horizontal_unit_triangle_area() {
        let t=Triangle{v:[Vec3::new(0.,0.,0.),Vec3::new(1.,0.,0.),Vec3::new(0.,1.,0.)]};
        assert!((t.area()-0.5).abs()<1e-12); assert!((t.normal().z-1.0).abs()<1e-12);
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
    #[test]
    fn paraboloid_flat_limit() {
        assert!((paraboloid_packing(1.0e-8)-1.0).abs()<1.0e-12);
        assert_eq!(paraboloid_packing(0.0),1.0);
    }
    #[test]
    fn small_k_series_matches_exact_away_from_cancellation() {
        let k=1.0e-2;
        let k2=k*k;
        let series=1.0+k2-(2.0/3.0)*k2*k2+k2*k2*k2;
        let exact=((1.0+4.0*k2).powf(1.5)-1.0)/(6.0*k2);
        assert!((series-exact).abs()<1.0e-12);
    }
    #[test]
    fn paraboloid_inverse_pi_two() {
        let k=paraboloid_k_from_packing(2.0); assert!((paraboloid_packing(k)-2.0).abs()<1.0e-12);
    }
    #[test]
    fn paraboloid_mesh_area_converges() {
        let mesh=paraboloid_mesh(2.0,1.0,16,64); let area:f64=mesh.iter().map(Triangle::area).sum();
        assert!((area-2.0).abs()<0.02);
    }
    #[test]
    fn equatorial_equinox_noon_is_overhead_approximately() {
        let s=solar_vector_preliminary(0.0,81,12.0); assert!(s.z>0.999);
    }
}
