use mushroom_solar_kernel::{
    horizontal_directional_response, paraboloid_absolute_gain,
    paraboloid_directional_response, paraboloid_packing,
};

/// Rust-native analytical dataset generator for the isolated fixed paraboloid.
///
/// Model status: analytical/model-generated, not measured and not a Singapore
/// annual-energy result. Visibility is V=1. Angles are emitted in degrees for
/// human-readable output but converted to radians for the kernel.
///
/// Category-4 design samples: k=H/R in {0, 0.25, 0.5, 1.0}. These values are
/// exploratory geometry samples, not optima.
///
/// Category-5 numerical setting: N_RADIAL=1024 Simpson intervals. This value is
/// not a physical constant and must be supported by convergence before
/// validated reporting.
const N_RADIAL: usize = 1024;

fn main() {
    let k_values = [0.0_f64, 0.25, 0.5, 1.0];

    println!(
        "theta_z_deg,k_h_over_r,packing_ratio_pi,c_paraboloid,c_horizontal,delta_c"
    );

    // Category-5 output grid: integer zenith angles from 0 to 90 degrees,
    // inclusive. One-degree spacing is for reproducible diagnostics only.
    for theta_deg in 0_u32..=90 {
        let theta_rad = (theta_deg as f64).to_radians();
        let horizontal = horizontal_directional_response(theta_rad);

        for k in k_values {
            let c = paraboloid_directional_response(theta_rad, k, N_RADIAL);
            let delta = paraboloid_absolute_gain(theta_rad, k, N_RADIAL);
            let pi = paraboloid_packing(k);

            println!(
                "{theta_deg},{k:.6},{pi:.12},{c:.12},{horizontal:.12},{delta:.12}"
            );
        }
    }
}
