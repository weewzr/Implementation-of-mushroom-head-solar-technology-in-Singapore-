use mushroom_solar_kernel::{array_direct_projected_area, paraboloid_mesh, solar_vector_preliminary};
use std::time::Instant;

const LATITUDE_DEG: f64 = 1.3521;
const PACKING_RATIO: f64 = 2.0;
const FOOTPRINT_AREA_M2: f64 = 1.0;

fn score(n_r: usize, n_phi: usize, day_step: usize, time_step_h: f64) -> (f64, usize, usize) {
    let mesh = paraboloid_mesh(PACKING_RATIO, FOOTPRINT_AREA_M2, n_r, n_phi);
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
                denominator += s.z;
                samples += 1;
            }
            hour += time_step_h;
        }
        day += day_step;
    }
    (numerator / denominator, samples, mesh.len())
}

fn emit(group: &str, level: &str, n_r: usize, n_phi: usize, day_step: usize, time_step_h: f64, previous: Option<f64>) -> f64 {
    let start = Instant::now();
    let (ratio, samples, triangles) = score(n_r, n_phi, day_step, time_step_h);
    let relative = previous.map(|p| (ratio - p).abs() / ratio);
    let relative_text = relative.map(|x| format!("{x:.12}")).unwrap_or_default();
    println!("{group},{level},{n_r},{n_phi},{day_step},{time_step_h:.6},{ratio:.12},{relative_text},{samples},{triangles},{:.6}", start.elapsed().as_secs_f64());
    ratio
}

fn main() {
    println!("group,level,n_r,n_phi,day_step_days,time_step_hours,direct_beam_geometry_ratio,relative_change_from_previous,solar_samples,triangles,elapsed_s");

    // Numerical settings only. Spatial ladder holds temporal quadrature fixed.
    let spatial = [("s1", 3, 12), ("s2", 5, 20), ("s3", 8, 32), ("s4", 12, 48)];
    let mut previous = None;
    for (level, n_r, n_phi) in spatial {
        let ratio = emit("spatial", level, n_r, n_phi, 5, 0.25, previous);
        previous = Some(ratio);
    }

    // Temporal ladder holds the mesh fixed to isolate quadrature refinement.
    let temporal = [("t1", 20, 1.0), ("t2", 10, 0.5), ("t3", 5, 0.25), ("t4", 2, 0.125)];
    previous = None;
    for (level, day_step, time_step_h) in temporal {
        let ratio = emit("temporal", level, 8, 32, day_step, time_step_h, previous);
        previous = Some(ratio);
    }
}
