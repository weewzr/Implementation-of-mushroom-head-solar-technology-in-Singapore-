//! Discrete surface verification against the analytical paraboloid.
//!
//! This is numerical-method verification, not a Singapore annual-yield model.

use std::f64::consts::PI;
use mushroom_solar::geometry::paraboloid_area;
use mushroom_solar::mesh::{Facet, Vec3};

fn sub(a:Vec3,b:Vec3)->Vec3{Vec3{x:a.x-b.x,y:a.y-b.y,z:a.z-b.z}}
fn cross(a:Vec3,b:Vec3)->Vec3{Vec3{x:a.y*b.z-a.z*b.y,y:a.z*b.x-a.x*b.z,z:a.x*b.y-a.y*b.x}}

fn point(r:f64, phi:f64, radius:f64, height:f64)->Vec3{
    Vec3{x:r*phi.cos(),y:r*phi.sin(),z:height*(1.0-r*r/(radius*radius))}
}

fn tri(a:Vec3,b:Vec3,c:Vec3)->Facet{
    let e1=sub(b,a); let e2=sub(c,a); let cr=cross(e1,e2);
    let area=0.5*cr.norm();
    let centroid=Vec3{x:(a.x+b.x+c.x)/3.0,y:(a.y+b.y+c.y)/3.0,z:(a.z+b.z+c.z)/3.0};
    Facet::new(area,cr,centroid)
}

fn mesh(radius:f64,height:f64,nr:usize,nphi:usize)->Vec<Facet>{
    assert!(nr>0 && nphi>=3);
    let mut out=Vec::new();
    // Centre fan avoids degenerate triangles at r=0.
    let center=point(0.0,0.0,radius,height);
    let r1=radius/(nr as f64);
    for j in 0..nphi {
        let p0=2.0*PI*(j as f64)/(nphi as f64);
        let p1=2.0*PI*((j+1) as f64)/(nphi as f64);
        out.push(tri(center,point(r1,p1,radius,height),point(r1,p0,radius,height)));
    }
    for ir in 1..nr {
        let ra=radius*(ir as f64)/(nr as f64);
        let rb=radius*((ir+1) as f64)/(nr as f64);
        for j in 0..nphi {
            let p0=2.0*PI*(j as f64)/(nphi as f64);
            let p1=2.0*PI*((j+1) as f64)/(nphi as f64);
            let a=point(ra,p0,radius,height); let b=point(rb,p0,radius,height);
            let c=point(rb,p1,radius,height); let d=point(ra,p1,radius,height);
            out.push(tri(a,c,b)); out.push(tri(a,d,c));
        }
    }
    out
}

fn main(){
    let radius=1.0; let height=0.5;
    let exact=paraboloid_area(radius,height);
    let footprint=PI*radius*radius;
    let exact_diffuse_ratio=0.5*(exact/footprint+1.0);
    println!("nr,nphi,facets,area_m2,area_rel_error,ideal_diffuse_ratio,diffuse_rel_error");
    for nr in [2usize,4,8,16,32,64] {
        let nphi=8*nr;
        let facets=mesh(radius,height,nr,nphi);
        let area:f64=facets.iter().map(|f|f.area_m2).sum();
        // For a consistently upward-oriented triangulated graph surface,
        // integral cos(beta)dA equals sum(n_z A).
        let projected:f64=facets.iter().map(|f|f.normal.z*f.area_m2).sum();
        let diffuse_ratio=0.5*(area+projected)/footprint;
        println!("{nr},{nphi},{},{:.10},{:.6e},{:.10},{:.6e}",
            facets.len(),area,(area-exact).abs()/exact,diffuse_ratio,
            (diffuse_ratio-exact_diffuse_ratio).abs()/exact_diffuse_ratio);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn discrete_area_converges_toward_exact(){
        let exact=paraboloid_area(1.0,0.5);
        let coarse:f64=mesh(1.0,0.5,4,32).iter().map(|f|f.area_m2).sum();
        let fine:f64=mesh(1.0,0.5,32,256).iter().map(|f|f.area_m2).sum();
        assert!((fine-exact).abs() < (coarse-exact).abs());
        assert!((fine-exact).abs()/exact < 1e-3);
    }

    #[test]
    fn projected_area_recovers_footprint(){
        let facets=mesh(1.0,0.5,32,256);
        let projected:f64=facets.iter().map(|f|f.area_m2*f.normal.z).sum();
        let exact=PI;
        assert!((projected-exact).abs()/exact < 2e-4);
    }

    #[test]
    fn ideal_diffuse_converges_to_analytical_benchmark(){
        let exact_area=paraboloid_area(1.0,0.5);
        let exact_ratio=0.5*(exact_area/PI+1.0);
        let calc=|nr:usize|{
            let fs=mesh(1.0,0.5,nr,8*nr);
            let area:f64=fs.iter().map(|f|f.area_m2).sum();
            let projected:f64=fs.iter().map(|f|f.area_m2*f.normal.z).sum();
            0.5*(area+projected)/PI
        };
        let coarse=calc(4);
        let fine=calc(32);
        assert!((fine-exact_ratio).abs() < (coarse-exact_ratio).abs());
        assert!((fine-exact_ratio).abs()/exact_ratio < 1e-3);
    }

    #[test]
    fn all_paraboloid_facets_face_upward(){
        for f in mesh(1.0,0.5,16,128) { assert!(f.normal.z > 0.0); }
    }
}
