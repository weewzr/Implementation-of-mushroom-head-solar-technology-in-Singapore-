# Mushroom Experiment 1 — Paraboloid vs conventional reference

**Status: DEVELOPMENT_NOT_SERIS. Incident solar energy only.**

Experiment 1 uses the frozen Audit-46 NASA POWER 2024 conventional baseline and frozen Rust SPA/visibility foundations. It compares only the conventional horizontal reference and the original upward-facing paraboloidal mushroom. No thermal/electrical conversion, tracking, economics, topology optimisation or additional geometry is included.

## Fixed experiment definition
- weather: frozen NASA POWER Singapore 2024, 8,784 hourly intervals;
- solar geometry/time: frozen Audit-46 SPA midpoint convention;
- albedo: 0.20;
- mushroom discrete packing ratio: exactly **2.0** at every reported mesh resolution;
- final mesh: 4 radial × 24 azimuthal discretisation, **168 triangular facets**;
- final sky quadrature: 16 × 64 = **1,024 equal-solid-angle sky patches**;
- direct beam: canonical Rust ray/triangle self-shadowing;
- diffuse sky: canonical Rust sky-patch visibility;
- ground-reflected diffuse: isotropic unobstructed-ground factor per upward facet, using the same 0.20 albedo assumption as the conventional baseline.

## Fair-resource experiments

### A — equal active PV surface area
Both systems receive exactly 1.000 m² active PV. The mushroom's Π=2 geometry occupies exactly 0.500 m² horizontal footprint.

| Geometry | PV area m² | land m² | annual incident Wh | Wh/m²-PV | Wh/m²-land |
|---|---:|---:|---:|---:|---:|
| conventional | 1.000 | 1.000 | 1,645,574.23 | 1,645,574.23 | 1,645,574.23 |
| paraboloidal mushroom | 1.000 | 0.500 | 1,054,679.48 | 1,054,679.48 | 2,109,358.97 |

The mushroom receives less incident energy per unit active PV area in this development model, while using half the land.

### B — equal horizontal land footprint
Both systems receive exactly 1.000 m² horizontal footprint. The Π=2 mushroom therefore contains exactly 2.000 m² active PV.

| Geometry | PV area m² | land m² | annual incident Wh | Wh/m²-PV | Wh/m²-land |
|---|---:|---:|---:|---:|---:|
| conventional | 1.000 | 1.000 | 1,645,574.23 | 1,645,574.23 | 1,645,574.23 |
| paraboloidal mushroom | 2.000 | 1.000 | 2,109,358.97 | 1,054,679.48 | 2,109,358.97 |

## Component result
For the 1 m²-PV mushroom (Experiment A):
- direct: 303,959.20 Wh;
- sky diffuse: 667,504.91 Wh;
- ground reflected: 83,215.38 Wh;
- total: 1,054,679.48 Wh.

Experiment B is the geometrically scaled counterpart at twice the PV/land scale, so its component energies are exactly doubled.

## Normalized metrics
- packing ratio: **2.0000**;
- packing efficiency = mushroom PV-area-normalized incident energy / conventional PV-area-normalized incident energy = **0.6409188**;
- land-energy multiplier = mushroom land-normalized incident energy / conventional land-normalized incident energy = **1.2818376**.

These metrics answer different questions and must not be collapsed into a single claim that one geometry “wins”.

## Numerical convergence
Mesh convergence at fixed exact Π=2 and sky_n=8:
- 36 facets: 1,043,020.06 Wh;
- 90 facets: 1,051,524.95 Wh;
- 168 facets: 1,054,510.80 Wh.
The 90→168 change is **0.283%** of the finest value and smaller than the preceding refinement change.

Sky-patch convergence at the final 168-facet mesh:
- 64 patches: 1,052,580.25 Wh;
- 256 patches: 1,054,510.80 Wh;
- 1,024 patches: 1,054,679.48 Wh.
The 256→1,024 change is **0.0160%** of the finest value and smaller than the preceding refinement change.

## Reproducibility
Canonical source: `src/bin/mushroom_experiment_1.rs`.
Whole-crate Rust evidence after exact-packing correction: Actions run **36227632046**, PASS.
End-to-end weather acquisition + Experiment 1 artifact evidence: Actions run **36227636308**, PASS.
The artifact contains `annual_results.csv`, `monthly_results.csv`, `mesh_convergence.csv`, `sky_convergence.csv`, two monthly SVG plots and `checks.txt`, alongside the frozen baseline/weather evidence.

A prior completed run exposed and retained an equal-land fairness defect (1.01615 m² footprint); that result was rejected. A subsequent correction exposed the finite-mesh Π mismatch (≈1.968 instead of declared 2); that result was also rejected. Only the exact-resource result above is accepted for Experiment 1.

No SERIS validation claim is made.
