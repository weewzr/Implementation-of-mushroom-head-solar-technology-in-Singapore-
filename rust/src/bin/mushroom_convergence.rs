use mushroom_solar_kernel::{array_direct_projected_area, paraboloid_mesh, solar_vector_preliminary};
use std::time::Instant;

const LATITUDE_DEG: f64 = 1.3521; // provisional representative Singapore latitude; sourced/site-specific later
const PACKING_RATIO: f64 = 2.0; // canonical engineering test condition, dimensionless

fn score(n_r: usize, n_phi: usize, day_step: usize, time_step_h: f64) -> (f64, usize, usize) {
    let mesh = paraboloid_mesh(PACKING_RATIO, 1.0, n_r, n_phi);
    let mut numerator = 0.0;
    let mut denominator = 0.0;
    let mut samples = 0usize;
    let mut day = 1usize;
    while day <= 365 {
        let mut hour = 6.0;
        while hour <= 18.0 + 0.5 * time_step_h {
            let s = solar_vector_preliminary(LATITUDE_DEG, day as u32, hour);
            if s.z > 0.0 {
                numerator += array_direct_projected_area(&mesh, s);
                denominator += s.z; // 1 m^2 horizontal reference projected area
                samples += 1;
            }
            hour += time_step_h;
        }
        day += day_step;
    }
    (numerator / denominator, samples, mesh.len())
}

fn main() {
    // All ladder values below are numerical settings requiring convergence, not physical constants.
    let cases = [
        ("coarse", 3, 12, 20, 1.0),
        ("medium", 5, 20, 10, 0.5),
        ("fine", 8, 32, 5, 0.25),
        ("finer", 12, 48, 2, 0.125),
    ];
    println!("level,n_r,n_phi,day_step_days,time_step_hours,direct_beam_geometry_ratio,solar_samples,triangles,elapsed_s");
    for (name, nr, np, dn, dt) in cases {
        let start = Instant::now();
        let (ratio, samples, triangles) = score(nr, np, dn, dt);
        println!("{name},{nr},{np},{dn},{dt:.6},{ratio:.12},{samples},{triangles},{:.6}", start.elapsed().as_secs_f64());
    }
}
