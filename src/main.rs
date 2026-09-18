use mushroom_solar::geometry::{paraboloid_area, paraboloid_packing_ratio};

fn main() {
    // Canonical analytical demonstration only; not a Singapore yield prediction.
    let radius_m = 1.0;
    let height_m = 0.5;
    let area_m2 = paraboloid_area(radius_m, height_m);
    let packing = paraboloid_packing_ratio(radius_m, height_m);

    println!("Paraboloidal mushroom analytical check");
    println!("radius = {radius_m:.3} m");
    println!("height = {height_m:.3} m");
    println!("active PV area = {area_m2:.6} m^2");
    println!("packing ratio = {packing:.6}");
}
