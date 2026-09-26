# Mandatory Audit 46 — Continue #144

**Date:** 26 September 2026  
**Trigger:** Continue #144, mandatory Pass 3 audit cycle 46.

## Scope
Audit of the NASA POWER 2024 annual conventional-PV development baseline only. Frozen V1–V5, SPA A.5 and Rust visibility foundations were not reopened absent regression. No mushroom/3-D result, thermal/electrical conversion, tracking, optimisation or economics was introduced.

## Input/provenance
- NASA POWER hourly point query: 1.3521 N, 103.8198 E, calendar 2024, UTC.
- Raw SHA-256 repeatedly reproduced: `f3442ca0c336011c5c61fe434e00e1f7984f9d34afa27d1431fa939f5eac204c`.
- 8,784/8,784 leap-year hourly records; no gaps/duplicates/non-monotonic timestamps in Rust QC.
- POWER timestamps are hourly-start labels; baseline evaluates SPA at +30 min (interval midpoint).
- Dataset and every result row are labelled `DEVELOPMENT_NOT_SERIS`.

## Dimensional and reconciliation audit
POWER solar fields are hourly Wh/m². For the fixed one-hour interval their numerical value equals interval-average W/m², which is the value supplied to the POA rate equations; integration over exactly 1 h returns Wh/m². The code records this restriction explicitly.

Horizontal source-energy conservation is exact by construction. POWER has **453** intervals in which DHI > GHI, with total excess **1,258.69 Wh/m²**. For the horizontal component decomposition only, direct is set to max(GHI-DHI,0) and diffuse is the remainder GHI-direct. Thus horizontal direct+diffuse=GHI exactly while the source inconsistency is counted and reported rather than silently changing total energy.

## Solar geometry and horizon
Validated Rust SPA is evaluated at each interval midpoint in UTC. DNI is set to zero whenever SPA zenith >=90 degrees before tilted-plane transposition. The executable asserts zero night-time tilted direct beam.

Audit 46 added an explicit midpoint sensitivity experiment: every non-horizontal fixed plane is recomputed with four equal 15-minute subintervals using SPA at 07:30, 22:30, 37:30 and 52:30 minutes of each POWER hour while holding the source hourly irradiance state constant. A machine-readable `midpoint_sensitivity.csv` is generated. Acceptance requires maximum annual total-POA relative difference across the declared sweep <0.5%. The Audit-46 acquisition/baseline run **36226973850** passes this assertion, so hourly-midpoint solar geometry is adequate for Mushroom Experiment 1 at the current development-data fidelity.

## Conservation/convention audit
- horizontal annual total = source annual GHI = **1,645,574.23 Wh/m²**: PASS;
- horizontal ground component = 0: PASS;
- direct/diffuse/ground components sum to total by the POA implementation: PASS;
- monthly component sums reproduce annual components: PASS;
- leap-year completeness: PASS;
- fixed tilt is degrees from horizontal;
- azimuth convention is clockwise from north, matching validated SPA/project convention;
- ground albedo = **0.20**, explicitly a development assumption, not a measured site value.

## Artifacts and reproducibility
Rust-only baseline executable: `src/bin/annual_development_baseline.rs`.
Generated machine-readable/visual artifacts:
- `annual_poa_sweep.csv`;
- `monthly_poa_sweep.csv`;
- `midpoint_sensitivity.csv`;
- `checks.txt`;
- `horizontal_monthly_components.svg`;
- `fixed_tilt_azimuth_sweep.svg`.

Pre-audit whole-crate Rust evidence run **36209058767** passed. End-to-end acquisition/baseline run **36209063066** passed. Audit-46 corrected source plus midpoint sensitivity is executed by run **36226973850**, PASS, and retains the raw file, checksum, QC, CSV/SVG outputs and sensitivity table as an Actions artifact.

## Material defects
No material baseline defect remained after the Pass-143 DHI>GHI reconciliation. Audit 46 identified one missing validation item: midpoint SPA timing had been described as an approximation but not quantitatively bounded. This was fixed in code by four-point sub-hourly quadrature, a <0.5% annual sensitivity acceptance assertion, and machine-readable sensitivity output. The corrected chain passes.

## Freeze / authorization
**NASA POWER 2024 annual conventional-PV DEVELOPMENT baseline: PASS / FROZEN.**

Frozen scope is incident-energy geometry/transposition only, using NASA POWER development weather and isotropic diffuse + albedo 0.20. It is not SERIS-measured validation and is not electrical yield.

**Mushroom Experiment 1 is AUTHORIZED** to compare the already-verified mushroom geometry against this frozen development baseline under explicitly equal resource/model assumptions. Authorization does not pre-approve any mushroom performance result; Experiment 1 must retain development labelling and its own mesh/visibility/convergence/conservation evidence.
