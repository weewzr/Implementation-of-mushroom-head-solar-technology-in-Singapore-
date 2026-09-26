//! Canonical discrete triangular-mesh resource accounting and normalization.
//!
//! All equal-resource numerical work must use this module rather than
//! reimplementing triangle area, projected land area, scaling, or packing in a binary.

use crate::{mesh::Vec3, visibility::Triangle};

/// Absolute tolerance used for normalized resource postconditions, in m^2.
pub const RESOURCE_AREA_TOL_M2: f64 = 1.0e-10;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiscreteResources {
    pub active_pv_area_m2: f64,
    pub projected_land_area_m2: f64,
}

impl DiscreteResources {
    pub fn packing_ratio(self) -> f64 {
        self.active_pv_area_m2 / self.projected_land_area_m2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceGeometryError {
    EmptyMesh,
    NonFiniteVertex,
    DegenerateTriangle,
    NonFiniteGeometry,
    NonUpwardOrientation,
    InvalidTarget,
    PostconditionFailed,
}

fn sub(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 { x: a.x-b.x, y: a.y-b.y, z: a.z-b.z }
}
fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.y*b.z-a.z*b.y,
        y: a.z*b.x-a.x*b.z,
        z: a.x*b.y-a.y*b.x,
    }
}
fn finite(v: Vec3) -> bool { v.x.is_finite() && v.y.is_finite() && v.z.is_finite() }

/// Return (triangle area, upward unit normal).
pub fn triangle_area_normal(t: &Triangle) -> Result<(f64, Vec3), ResourceGeometryError> {
    if !t.v.iter().copied().all(finite) { return Err(ResourceGeometryError::NonFiniteVertex); }
    let c = cross(sub(t.v[1],t.v[0]), sub(t.v[2],t.v[0]));
    let n2 = c.dot(c);
    if !n2.is_finite() { return Err(ResourceGeometryError::NonFiniteGeometry); }
    if n2 <= 0.0 { return Err(ResourceGeometryError::DegenerateTriangle); }
    let norm = n2.sqrt();
    let area = 0.5*norm;
    let n = Vec3 { x:c.x/norm, y:c.y/norm, z:c.z/norm };
    if !area.is_finite() || !finite(n) { return Err(ResourceGeometryError::NonFiniteGeometry); }
    if n.z <= 0.0 { return Err(ResourceGeometryError::NonUpwardOrientation); }
    Ok((area,n))
}

/// Measure active PV area and upward horizontal projected land area from exactly
/// the same accepted triangle mesh.
pub fn discrete_resources(mesh: &[Triangle]) -> Result<DiscreteResources, ResourceGeometryError> {
    if mesh.is_empty() { return Err(ResourceGeometryError::EmptyMesh); }
    let mut pv=0.0; let mut land=0.0;
    for t in mesh {
        let (a,n)=triangle_area_normal(t)?;
        pv += a;
        land += a*n.z;
    }
    if !pv.is_finite() || !land.is_finite() || pv<=0.0 || land<=0.0 {
        return Err(ResourceGeometryError::NonFiniteGeometry);
    }
    Ok(DiscreteResources { active_pv_area_m2:pv, projected_land_area_m2:land })
}

pub fn uniformly_scaled(mesh: &[Triangle], scale: f64) -> Result<Vec<Triangle>, ResourceGeometryError> {
    if !scale.is_finite() || scale<=0.0 { return Err(ResourceGeometryError::InvalidTarget); }
    discrete_resources(mesh)?;
    Ok(mesh.iter().map(|t| {
        let mut q=*t;
        for p in &mut q.v { p.x*=scale; p.y*=scale; p.z*=scale; }
        q
    }).collect())
}

fn normalize(mesh:&[Triangle], target:f64, land:bool) -> Result<Vec<Triangle>, ResourceGeometryError> {
    if !target.is_finite() || target<=0.0 { return Err(ResourceGeometryError::InvalidTarget); }
    let before=discrete_resources(mesh)?;
    let current=if land {before.projected_land_area_m2}else{before.active_pv_area_m2};
    let out=uniformly_scaled(mesh,(target/current).sqrt())?;
    let after=discrete_resources(&out)?;
    let actual=if land {after.projected_land_area_m2}else{after.active_pv_area_m2};
    let tol=RESOURCE_AREA_TOL_M2.max(target.abs()*1.0e-12);
    if (actual-target).abs()>tol { return Err(ResourceGeometryError::PostconditionFailed); }
    Ok(out)
}

pub fn normalize_to_land_area(mesh:&[Triangle], target_land_m2:f64) -> Result<Vec<Triangle>, ResourceGeometryError> {
    normalize(mesh,target_land_m2,true)
}
pub fn normalize_to_pv_area(mesh:&[Triangle], target_pv_m2:f64) -> Result<Vec<Triangle>, ResourceGeometryError> {
    normalize(mesh,target_pv_m2,false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    fn v(x:f64,y:f64,z:f64)->Vec3{Vec3{x,y,z}}
    fn paraboloid(k:f64,nr:usize,np:usize)->Vec<Triangle>{
        let r=(1.0/PI).sqrt(); let h=k*r;
        let p=|rr:f64,ph:f64|v(rr*ph.cos(),rr*ph.sin(),h*(1.0-(rr/r).powi(2)));
        let cen=v(0.0,0.0,h); let r1=r/nr as f64; let mut out=vec![];
        for j in 0..np { let a=2.0*PI*j as f64/np as f64; let b=2.0*PI*(j+1) as f64/np as f64; out.push(Triangle{v:[cen,p(r1,a),p(r1,b)]}); }
        for i in 1..nr { let ra=r*i as f64/nr as f64; let rb=r*(i+1) as f64/nr as f64;
            for j in 0..np { let a=2.0*PI*j as f64/np as f64; let b=2.0*PI*(j+1) as f64/np as f64; let q=[p(ra,a),p(rb,a),p(rb,b),p(ra,b)]; out.push(Triangle{v:[q[0],q[1],q[2]]}); out.push(Triangle{v:[q[0],q[2],q[3]]}); }
        } out
    }
    #[test] fn scale_invariance_of_packing(){
        let m=paraboloid(0.5,4,24); let a=discrete_resources(&m).unwrap();
        for s in [0.1,0.5,2.0,10.0] { let b=discrete_resources(&uniformly_scaled(&m,s).unwrap()).unwrap(); assert!((a.packing_ratio()-b.packing_ratio()).abs()<1e-12); }
    }
    #[test] fn exact_land_and_pv_postconditions_across_curvature(){
        for k in [0.05,0.5,1.5,3.0] {
            let m=paraboloid(k,5,30);
            let l=normalize_to_land_area(&m,1.0).unwrap(); let lr=discrete_resources(&l).unwrap(); assert!((lr.projected_land_area_m2-1.0).abs()<=RESOURCE_AREA_TOL_M2);
            let p=normalize_to_pv_area(&m,1.0).unwrap(); let pr=discrete_resources(&p).unwrap(); assert!((pr.active_pv_area_m2-1.0).abs()<=RESOURCE_AREA_TOL_M2);
        }
    }
    #[test] fn invalid_geometry_is_rejected(){
        assert_eq!(discrete_resources(&[]),Err(ResourceGeometryError::EmptyMesh));
        let down=Triangle{v:[v(0.0,0.0,0.0),v(0.0,1.0,0.0),v(1.0,0.0,0.0)]};
        assert_eq!(discrete_resources(&[down]),Err(ResourceGeometryError::NonUpwardOrientation));
        let deg=Triangle{v:[v(0.0,0.0,0.0),v(1.0,0.0,0.0),v(2.0,0.0,0.0)]};
        assert_eq!(discrete_resources(&[deg]),Err(ResourceGeometryError::DegenerateTriangle));
        let bad=Triangle{v:[v(f64::NAN,0.0,0.0),v(1.0,0.0,0.0),v(0.0,1.0,0.0)]};
        assert_eq!(discrete_resources(&[bad]),Err(ResourceGeometryError::NonFiniteVertex));
        assert!(matches!(normalize_to_land_area(&paraboloid(0.5,3,18),0.0),Err(ResourceGeometryError::InvalidTarget)));
    }
}
