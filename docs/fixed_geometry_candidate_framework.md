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


## Future folded-surface implementation branch: deployable solar sheet

A future manufacturability branch of the folded-surface family may represent a **flat-manufactured flexible or segmented PV sheet** that is deployed into a three-dimensional irradiance-collecting surface. Candidate mechanisms may include folding, rolling, fan/origami deployment, or a tensioned-membrane arrangement.

This is an implementation/manufacturability branch, not a new irradiance model and not an exception to the common framework. Its deployed mesh must still enter the same `resource_geometry`, visibility, sky-view and annual-irradiance pipeline and satisfy the same matched-resource contracts.

Future engineering constraints to add only in a later implementation layer include minimum bend radius, allowable PV-cell/interconnect strain, hinge/rib/cable geometry, deployment and stow energy, wind-load and wind-stow operation, mechanism/support mass, fatigue and maintenance, deployment reliability, and manufacturing/lifecycle cost. None of these penalties is included in the frozen geometric irradiance comparison at this stage, so no deployable-sheet advantage may be claimed from irradiance geometry alone.


## Pass 159 validation status

Whole-crate Rust evidence at commit `adde4deec12cdc5e44cc340ac59ea7edf9f9080a` passes in run `36290127196`. Retained artifact `10921797900` has SHA-256 `be82d75632c40b4deea0867fc5a61f4c84f79a143e4951529bffcb359f804ae6`. The refined hemisphere test demonstrates monotone multi-resolution convergence of discrete area/footprint ratio toward the analytical value 2 and meets the declared 1% tolerance at the refined mesh. Equal-land/equal-PV contracts, matched-packing rejection, finite/upward geometry, shallow-paraboloid flat limit and folded-surface secant relation also pass.

The associated full NASA POWER end-to-end run `36290127283` was still executing when Pass 159 was recorded. Therefore the framework is **whole-crate-test-qualified but pending end-to-end qualification** and is not yet released for candidate performance execution or ranking.
