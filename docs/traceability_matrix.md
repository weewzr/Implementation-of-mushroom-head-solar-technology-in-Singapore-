# Equation–Code–Evidence Traceability Matrix

This file implements the master requirement that a reader can move from physical principle to equation to numerical implementation to generated result and engineering conclusion. Status labels prevent incomplete code from being mistaken for validation.

| Model element | Report / equation source | Rust implementation | Tests / validation | Data / evidence | Status |
|---|---|---|---|---|---|
| Parameter/evidence classification | `docs/parameter_provenance_register.md`; technical report §§6, 12, 14 and LaTeX parameter-evidence gate | Rust constants currently appear in geometry/solar/candidates modules; no central config API yet | audit review + source verification + future sensitivity/convergence | EMA/SERIS/NREL plus explicit assumptions/unset inputs | Foundation register implemented; dynamic synchronization required |
| Coordinate/sign convention | `docs/coordinate_conventions.md`; technical report §2 | `src/mesh.rs` documents ENU ordering; `src/solar.rs::solar_direction_enu` constructs the Sun-direction unit vector from clockwise-from-north azimuth and elevation | source tests cover north/east/zenith cardinal directions and unit norm; execution evidence from `cargo test` still required | project definition | ENU Sun-vector API implemented at source level; execution validation pending |
| Paraboloidal cap area and packing | `equations/derivations.md` §1; technical report §4 | `src/geometry.rs` | flat limit, shallow limit, scale invariance, area>footprint | exact geometry | Implemented analytical benchmark |
| Cooper declination approximation | technical report §3.1; `docs/constants_and_provenance.md` | `src/solar.rs::cooper_declination` | physical declination bound | historical approximation; production validation target is NREL SPA | Preliminary only |
| Solar hour angle | technical report §3.2 | `src/solar.rs::hour_angle` | noon=0; 1 solar hour=15° | exact solar-time conversion | Implemented, but civil-time conversion incomplete |
| Solar elevation | technical report §3.3 | `src/solar.rs::solar_elevation` | equator/equinox/noon zenith limit | spherical solar geometry | Preliminary geometry layer |
| Direct facet incidence | technical report §3.4; governing equations §2 | `src/mesh.rs::direct_beam_intercept_w` | normal, grazing, backside cases | requires time-resolved DNI and visibility | Incidence kernel only; no shadowing |
| Equal-resource candidate contract | `docs/equal_resource_comparison.md` | `src/candidates.rs` | packing=2 canonical assumption; invalid tilt/height tests | 1 m² / 2 m² / 2 m are design assumptions | Implemented contract, not a performance result |
| Isotropic diffuse benchmark | `equations/derivations.md` §2 | Not yet canonical Rust module | analytical projection identity required | DHI required | Equation benchmark only |
| Land multiplication | derivations §3; Markdown report §7; LaTeX `Land multiplication` | Not yet result pipeline | algebraic identity | candidate/baseline annual energy required | Metric defined; no validated annual result |
| Packing stationary condition | derivations §4; Markdown report §7; LaTeX `Land multiplication` | Not yet optimisation solver | derivative identity | packing sweep required | Diagnostic only |
| Rotation/mechanics | derivations §5; Markdown report §9; LaTeX `Mechanics` | Not yet implemented | scaling checks required | inertia, friction, wind and actuator inputs required | Conceptual/analytical only |
| Self-shadowing / sky visibility | report roadmap | Not implemented | ray/mesh convergence required | geometry + sun/sky directions | Missing |
| Bifacial rear irradiance | report §11 | Not implemented | benchmark/validation required | bifaciality, albedo, rear view | Missing |
| Thermal/electrical conversion | report §11 | Not implemented | datasheet/model validation required | temperature coefficients/module data | Missing |
| Wind/structure | report §9 | Not implemented | structural/aero validation required | sourced wind/load inputs | Missing |
| Free-form optimisation | report §12 | candidate variables only | solver/convergence/Pareto tests required | all validated model layers | Not ready |

## Rule for new results

A new quantitative result must not enter `Current findings` as validated until its row has a traceable implementation, tests/convergence evidence, source inputs, units and a stated uncertainty/limitation basis. Geometry-only or synthetic-input outputs must be labelled analytical or exploratory.