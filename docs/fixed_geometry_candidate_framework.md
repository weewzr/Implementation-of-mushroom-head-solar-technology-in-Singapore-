# Fixed-Geometry Equal-Resource Candidate Framework

**Status:** framework definition only — no comparative headline performance claims.

## Authorization basis

The paraboloid geometric-response benchmark is frozen **DEVELOPMENT_NOT_SERIS** after post-Audit-50 closure on Continue #155. This authorizes a common fixed-geometry comparison framework, but not a winner, optimum, or validated performance ranking.

## Candidate set

The initial fixed set is:
- flat horizontal reference;
- paraboloid;
- hemisphere;
- faceted canopy;
- folded surface.

No tracking, thermal/electrical conversion, economics, optimisation, or new physical model is introduced by this framework.

## One canonical mesh contract

Every candidate generator must return the same canonical Rust triangular representation, `visibility::Triangle`, in ENU coordinates with upward-oriented facets. Candidate generators may define topology only; they must not own resource scaling, projected-area accounting, packing-ratio calculation, visibility, sky integration, or annual irradiance equations.

## One canonical resource contract

All generated meshes must pass through `src/resource_geometry.rs`.

Two comparison cases are permitted and must never be conflated:

1. **Equal land:** `normalize_to_land_area(mesh, 1.0)`; postcondition: discrete projected horizontal land area = 1 m² within `RESOURCE_AREA_TOL_M2`.
2. **Equal PV:** `normalize_to_pv_area(mesh, 1.0)`; postcondition: discrete active PV area = 1 m² within `RESOURCE_AREA_TOL_M2`.

Active PV area, projected land area and packing ratio must be read back through `discrete_resources`; no candidate-specific area/scaling/packing implementation is permitted.

## One irradiance pipeline

After normalization, every candidate must use the same:
1. validated solar position/time convention;
2. `visibility::direct_visibility` direct-beam obstruction path;
3. `visibility::sky_view_factor` sky-view path at matched sky resolution;
4. same ground-reflected formulation and bounded development albedo assumption;
5. same NASA POWER 2024 DEVELOPMENT_NOT_SERIS hourly input;
6. same annual integration convention.

The common evaluator should accept a normalized triangle slice plus shared run settings. Geometry-specific branches inside the irradiance evaluator are prohibited.

## Required validation before comparative release

For every candidate and both resource cases:
- canonical resource postcondition;
- finite/upward geometry;
- independent mesh convergence at fixed sky resolution;
- independent sky convergence at fixed mesh;
- finite component closure (direct + diffuse + ground = total);
- bounded temporal/albedo sensitivity where material;
- high-curvature/fold numerical-stability check where applicable;
- machine-readable provenance containing commit SHA, dataset hash, mesh/sky settings and DEVELOPMENT_NOT_SERIS status.

Only after these gates pass may a comparison table be treated as a released development result. The framework itself does not imply that any candidate is superior.

## Rust interface direction

The existing `GeometryKind`, `Candidate` and `ResourceEnvelope` types may identify requested topology and declared constraints. The next implementation step should add a common mesh-generation interface returning `Vec<Triangle>`, followed immediately by canonical resource normalization and a shared annual irradiance evaluator. Resource accounting must not be duplicated in `candidates.rs` or individual binaries.
