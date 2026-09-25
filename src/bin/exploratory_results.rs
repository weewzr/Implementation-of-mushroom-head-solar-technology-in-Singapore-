//! Exploratory analytical result sweeps.
//!
//! These outputs are NOT validated Singapore annual-yield predictions.
//! They use exact project geometry identities plus an ideal isotropic-sky
//! benchmark and explicitly supplied scenario assumptions.

use mushroom_solar::geometry::paraboloid_packing_ratio;

fn main() {
    println!("k,packing_ratio,ideal_diffuse_power_ratio_same_footprint");
    for i in 0..=20 {
        let k = i as f64 * 0.05;
        let packing = paraboloid_packing_ratio(1.0, k);
        // From P_diff = eta*DHI/2*(A_PV+A_foot).
        // Relative to a flat equal-footprint surface eta*DHI*A_foot:
        let diffuse_ratio = 0.5 * (packing + 1.0);
        println!("{k:.4},{packing:.8},{diffuse_ratio:.8}");
    }

    eprintln!("scenario_diffuse_share, k, ideal_total_ratio");
    // Historical/provisional sensitivity only. Assumes direct contribution
    // remains equal to the flat reference and only the ideal diffuse term
    // changes. This is deliberately NOT a physical annual 3-D yield model.
    for diffuse_share in [0.30_f64, 0.40, 0.50, 0.57, 0.60, 0.70] {
        for k in [0.25_f64, 0.50, 0.75, 1.00] {
            let packing = paraboloid_packing_ratio(1.0, k);
            let diffuse_ratio = 0.5 * (packing + 1.0);
            let ideal_total_ratio = (1.0 - diffuse_share) + diffuse_share * diffuse_ratio;
            eprintln!("{diffuse_share:.2},{k:.2},{ideal_total_ratio:.8}");
        }
    }
}
