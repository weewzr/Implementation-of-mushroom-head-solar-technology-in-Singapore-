use mushroom_solar_kernel::{array_direct_projected_area, Triangle, Vec3};
use std::time::Instant;

fn stacked_grid(n: usize) -> Vec<Triangle> {
    let mut out = Vec::new();
    let dx = 1.0 / n as f64;
    for layer in [0.0, 0.2] {
        for i in 0..n {
            for j in 0..n {
                let x = i as f64 * dx;
                let y = j as f64 * dx;
                let a = Vec3::new(x, y, layer);
                let b = Vec3::new(x + dx, y, layer);
                let c = Vec3::new(x + dx, y + dx, layer);
                let d = Vec3::new(x, y + dx, layer);
                out.push(Triangle { v: [a, b, c] });
                out.push(Triangle { v: [a, c, d] });
            }
        }
    }
    out
}

fn main() {
    // n=12 is a numerical benchmark setting, not a physical/design constant.
    let tris = stacked_grid(12);
    let sun = Vec3::new(0.3, -0.2, 0.93).unit();
    let start = Instant::now();
    let mut value = 0.0;
    // 100 repetitions are solely for stable wall-clock measurement.
    for _ in 0..100 {
        value = array_direct_projected_area(&tris, sun);
    }
    println!(
        "triangles={}, projected_area_m2={:.12}, repetitions=100, elapsed_s={:.6}",
        tris.len(),
        value,
        start.elapsed().as_secs_f64()
    );
}
