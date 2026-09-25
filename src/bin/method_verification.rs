//! Synthetic verification cases for time integration and direct-incidence bookkeeping.
//! These cases verify numerical implementation only; they are not field validation.

use mushroom_solar::mesh::{direct_beam_intercept_w, Facet, Vec3};

fn integrate_piecewise_constant(power_w:&[f64], dt_s:&[f64])->f64{
    assert_eq!(power_w.len(),dt_s.len());
    power_w.iter().zip(dt_s).map(|(p,dt)|p*dt).sum()
}

fn main(){
    let f=Facet::new(2.0,Vec3{x:0.0,y:0.0,z:1.0},Vec3{x:0.0,y:0.0,z:0.0});
    let normal=direct_beam_intercept_w(&f,800.0,Vec3{x:0.0,y:0.0,z:1.0});
    let grazing=direct_beam_intercept_w(&f,800.0,Vec3{x:1.0,y:0.0,z:0.0});
    let back=direct_beam_intercept_w(&f,800.0,Vec3{x:0.0,y:0.0,z:-1.0});
    println!("case,value_w");
    println!("normal_incidence,{normal:.6}");
    println!("grazing,{grazing:.6}");
    println!("backside,{back:.6}");

    let energy_j=integrate_piecewise_constant(&[100.0,200.0,50.0],&[10.0,20.0,30.0]);
    println!("synthetic_piecewise_energy_j,{energy_j:.6}");
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn synthetic_time_integral_is_exact(){
        // 100*10 + 200*20 + 50*30 = 6500 J.
        assert!((integrate_piecewise_constant(&[100.0,200.0,50.0],&[10.0,20.0,30.0])-6500.0).abs()<1e-12);
    }
    #[test]
    fn direct_special_cases_match_geometry(){
        let f=Facet::new(2.0,Vec3{x:0.0,y:0.0,z:1.0},Vec3{x:0.0,y:0.0,z:0.0});
        assert!((direct_beam_intercept_w(&f,800.0,Vec3{x:0.0,y:0.0,z:1.0})-1600.0).abs()<1e-12);
        assert!(direct_beam_intercept_w(&f,800.0,Vec3{x:1.0,y:0.0,z:0.0}).abs()<1e-12);
        assert!(direct_beam_intercept_w(&f,800.0,Vec3{x:0.0,y:0.0,z:-1.0}).abs()<1e-12);
    }
}
