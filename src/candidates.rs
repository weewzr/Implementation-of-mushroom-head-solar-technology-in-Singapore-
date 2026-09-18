//! Equal-resource candidate geometry definitions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeometryKind {
    Horizontal, FixedTilt { tilt_rad: f64 }, Sphere, Hemisphere,
    Cone { height_m: f64 }, Paraboloid { height_m: f64 },
    EastWestFold { tilt_rad: f64 }, FacetedCanopy { facets: usize },
}
#[derive(Debug, Clone, Copy)]
pub struct ResourceEnvelope { pub footprint_m2: f64, pub pv_area_m2: f64, pub max_height_m: f64 }
impl ResourceEnvelope {
    pub fn canonical() -> Self { Self { footprint_m2: 1.0, pv_area_m2: 2.0, max_height_m: 2.0 } }
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.footprint_m2 <= 0.0 { return Err("footprint must be positive"); }
        if self.pv_area_m2 <= 0.0 { return Err("PV area must be positive"); }
        if self.max_height_m < 0.0 { return Err("maximum height must be non-negative"); }
        Ok(())
    }
    pub fn packing_ratio(&self) -> f64 { self.pv_area_m2 / self.footprint_m2 }
}
#[derive(Debug, Clone, Copy)]
pub struct Candidate { pub kind: GeometryKind, pub resources: ResourceEnvelope }
impl Candidate {
 pub fn new(kind: GeometryKind, resources: ResourceEnvelope) -> Result<Self, &'static str> {
  resources.validate()?;
  match kind {
   GeometryKind::FixedTilt { tilt_rad } | GeometryKind::EastWestFold { tilt_rad } => if !(0.0..=std::f64::consts::FRAC_PI_2).contains(&tilt_rad) { return Err("tilt must lie between 0 and pi/2 radians"); },
   GeometryKind::Cone { height_m } | GeometryKind::Paraboloid { height_m } => if height_m < 0.0 || height_m > resources.max_height_m { return Err("candidate height violates resource envelope"); },
   GeometryKind::FacetedCanopy { facets } if facets == 0 => return Err("faceted canopy requires at least one facet"),
   _ => {}
  }
  Ok(Self { kind, resources })
 }
}
#[cfg(test)] mod tests { use super::*;
 #[test] fn canonical_ratio_two(){ assert!((ResourceEnvelope::canonical().packing_ratio()-2.0).abs()<1e-15); }
 #[test] fn equal_envelope(){ let r=ResourceEnvelope::canonical(); let a=Candidate::new(GeometryKind::Horizontal,r).unwrap(); let b=Candidate::new(GeometryKind::Sphere,r).unwrap(); assert_eq!(a.resources.pv_area_m2,b.resources.pv_area_m2); }
 #[test] fn invalid_tilt_rejected(){ assert!(Candidate::new(GeometryKind::FixedTilt{tilt_rad:2.0},ResourceEnvelope::canonical()).is_err()); }
 #[test] fn excessive_height_rejected(){ assert!(Candidate::new(GeometryKind::Cone{height_m:2.1},ResourceEnvelope::canonical()).is_err()); }
}