# Mandatory Audit 67 — Continue #197

## Governance
New-session Continue 1/12. Persisted state recovered in required order from the master instruction source, PASS_COUNTER, AUDIT_LEDGER, Audit 66, Audit-67 agenda, report roadmap/matrix/register, nomenclature and Phase-II plan. Continue #197 is the mandatory third pass after Audit 66. Frozen foundations were not reopened without regression evidence.

## Part I — annual controlled electrical evidence

### Retained canonical package inspected
Latest retained successful controlled-evidence package inspected directly:
- source commit: `b4b302f9a14baa4bc8e2a6603ed39d16d4f05b31`
- workflow run: `36383356436`
- artifact: `10953238813`
- artifact name: `controlled-comparison-evidence`
- artifact SHA-256: `f586623d81ffff8a373478d356ab7a234ae2b462de2a104af7defde197ec97cb`
- retained annual summary SHA-256: `1fa4c22645b4fb08549d06dcc908e0dfe85bcd38cf6808fe34c897fc4990050f`
- retained accepted timestep irradiance SHA-256: `172d6232612ad53e1053aa59cead9007bfa62a401b8b1d3e863891061b1e6f0e`
- retained monthly aggregate-back SHA-256: `13cfd759f122fffd7517092e38b14bbe21e1d8cda0e4591cefeb14af23663337`
- retained annual-package provenance-manifest SHA-256: `27c22979dcde0ecbd3e6e53fb501d2dd9f31dbccd9d3f8d438edeb1e132e2459`

The package contains 87,840 accepted timestep irradiance rows and 30 annual electrical scenario rows = 5 candidates x 2 resource contracts x 3 NMOT values.

### Upstream irradiance provenance
PASS. Retained rows bind flat_reference, frozen_paraboloid, hemisphere, faceted_canopy and folded_surface under equal_land/equal_pv; accepted 4x24 mesh; sky_n=16; 2024 NASA POWER DEVELOPMENT_NOT_SERIS weather; albedo 0.20; shared direct self-shadowing ray test; shared sky-view factor; canonical resource normalization; and frozen shared SPA lineage. The repository provenance manifest records annual max total aggregate-back residual 7.683e-9 Wh and monthly max absolute residual 5.059882823843509e-7 Wh across 480 checks with zero failures.

Independent inspection found exactly 8,784 accepted rows for every candidate/resource case, identical first/last UTC timestamps (1704067200 to 1735686000), zero duplicate candidate/resource/timestamp keys, and zero direct+diffuse+ground closure failures at 1e-6 Wh.

### Timestamp/weather join
SOURCE/EXECUTION PASS, RETAINED-REPLAY PARTIAL. The annual generator parses exactly 8,784 canonical NASA POWER hourly weather rows with no QC rejects, builds a map keyed by absolute UTC timestamp and requires `wm.get(&x.ts).expect("exact T2M timestamp")`. There is no nearest-neighbour join and no index join. The coupling contract explicitly forbids hidden time shifts. T2M is passed as degC.

However, the retained annual-electrical artifact does not retain the weather payload or timestep electrical rows. Therefore an auditor can verify the join contract and successful execution, but cannot independently replay every electrical timestep from the retained package alone.

### Thermal/electrical binding
PASS at source/lineage level. The generator imports and calls `thermal_electrical::evaluate_coupled_step` with `NmotParameters`; nominal/sensitivity values are 39/42/45 degC, reference ambient 20 degC and reference irradiance 800 W/m2. Faiman/WS10M is not used. The electrical design basis is `ElectricalDesignBasis::canadian_solar_cs62_48tm_460h()`, matching the Audit-62 basis: eta_ref=0.230 for the exact 460H datasheet row, module face area 1.762 x 1.134 m, gamma_P=-0.0029 1/degC, T_ref=25 degC and NMOT=42 degC. The 0.230 value is the selected-module datasheet basis, not the superseded generic placeholder.

### Accounting and sensitivity
PASS for all retained annual rows:
- finite/nonnegative gross DC and AC;
- ideal-zero-declared-loss scenario gives AC = delivered gross DC;
- auxiliary energy = 0 for the fixed baseline;
- net = AC - auxiliary;
- zero missing/rejected rows;
- PV-normalized and land-normalized yields are present;
- every candidate/resource case has 39, 42 and 45 degC rows;
- negative gamma_P ordering is correct: E39 >= E42 >= E45;
- module-temperature diagnostics order oppositely and remain finite.

Thermal diagnostics are simplified model outputs, not measurements. Across retained cases, maxima are finite and plausible for this DEVELOPMENT_NOT_SERIS NMOT model; they are not SERIS/measured temperatures.

### Release decision
**B — RELEASE WITHHELD.**

One narrow blocker remains: the retained package must include enough timestep electrical evidence to independently recompute annual electrical integration and audit the exact T2M-to-POA join from retained rows. The current artifact retains accepted irradiance rows and annual electrical summaries, but not timestep electrical rows (nor the weather payload). Generator assertions and CI success are strong execution evidence but do not satisfy the stricter independent retained-row replay requirement in Audit 67.

No candidate ranking or universal winner is authorized.

## Part II — report-quality audit

### Current report regression
Audit 67 found a real P0 reproducibility regression on current main: run `36383356491` at commit `b4b302f9...` failed because `fixed_equal_land.svg` was absent in a clean checkout. The current `.github/workflows/latex-report.yml` had lost the reproducible Rust publication-figure generation step.

Corrective commits:
- `4db50f98226a5eecd4896463660be4d3f0f1b426`: restore publication-figure generation;
- `4b16903399f6f9ae153b1f16720e38eb07ecaffd`: correct the canonical Cargo bin target name;
- `f8756c3e92467cd7f2a46237f9b014c7de7fc0be`: repair a literal table-newline defect in `equations/nomenclature.md`.

A fresh current-descendant PDF must be retained green before #198 substantive work. Until that evidence exists, CURRENT PDF is FAIL and P0=1 (reproducible clean build). This is an evidence/build P0, not a new scientific-model failure.

### Sections 1–8 visual audit
The latest retained green #196 descendant PDF (commit `d4d691351a5a76b2778e213c03f9864e8deca1ad`, run `36382919719`, artifact `10953259094`, artifact SHA-256 `94e5deaa6e4ce7a15fc61b9cb5957a327cce720dd05c380f2ad59b7beff53e72`) was rendered and Sections 1–8 inspected page by page.

Result: **READABLE with remaining visual P1 defects.** The prose/equation sequence now follows fair comparison -> mathematical formulation -> analytical verification -> numerical method. Equations are legible and the reader can generally see why each block exists, what variables mean and how analytical checks connect to numerical work. No raw LaTeX, broken Unicode or equation clipping was observed in these sections.

Remaining defects are real: the legacy solar-ray/direct-incidence figures still use source-like textual mathematics, and the validation/numerical-workflow figure is text-dense. These are not closed merely because the surrounding equations render.

### Nomenclature
The canonical symbol policy is materially active for Sun vector, surface normal, irradiance components, active/land area, packing ratio, packing efficiency, land-energy multiplier, time, temperature, power and energy. Audit 67 found and corrected one literal `\n` table defect in `equations/nomenclature.md`. No new symbol collision is authorized. A whole-report notation sweep remains a later P1/P2 closure task.

### P1 count
The reduction from Audit-66's eight localized rendered-PDF P1 classes to six after #195–196 is substantive, not category merging. The six genuine remaining classes are:
1. legacy mathematical-figure readability;
2. origami/deployable exposition;
3. controlled-comparison table readability;
4. electrical/thermal scientific-flow duplication;
5. citation completeness;
6. final report-spine/final-section coherence.

Sections 1–8 are substantially readable; their remaining major issue is primarily class 1 plus later citation/whole-paper consistency work.

## Phase-II closure
**NARROW CARRYOVER.** The validated/frozen foundations are sufficient: NMOT thermal model, electrical kernel, thermal-electrical coupling, strict annual adapter, accepted timestep irradiance exporter, annual/monthly aggregate-back, and a retained annual summary package. Broad Phase-II research is closed.

Carry over only:
- retained timestep electrical replay evidence sufficient to close the annual release gate;
- bounded mismatch/nonuniform-irradiance sensitivity, not a high-complexity string/bypass model unless evidence later requires it;
- explicit evidence-supported auxiliary/deployability penalties for moving concepts;
- report integration.

Fixed geometries retain P_aux=0 only under the declared fixed baseline. Moving/deployable concepts require explicit deployment/tracking/stow/control scenarios; no actuator-energy values may be invented.

## Report recovery and #227
Sections 9–12 are the next report-quality block. Optional Miura optimisation, topology optimisation, new geometry families, full FEA and high-complexity bypass/string modelling remain cut.

**#227 feasibility: AMBER.** Technical foundations are strong, but GREEN is not justified while the current clean report build has just regressed and six genuine P1 classes remain.

## Exact Continue #198 task
1. Before any manufacturing expansion, retain a green current-descendant report PDF after the Audit-67 CI repair and record commit/run/artifact/PDF SHA-256.
2. Close the one annual-electrical release blocker by retaining timestep electrical rows (or an equivalently replayable machine-readable package) with timestamp, candidate/resource, NMOT, canonical T2M, POA, module temperature, DC, AC, auxiliary, net and timestep energy; independently aggregate these rows back to the annual summary and verify hashes/provenance.
3. If and only if that replay passes, promote the annual controlled electrical result as DEVELOPMENT_NOT_SERIS and integrate it carefully into Section 15 without universal ranking.
4. In the same pass, begin the Sections 9–12 repair block, prioritizing V&V consolidation, fixed-candidate/result presentation, controlled-comparison table readability and origami/deployable exposition.
5. Do not open manufacturing until items 1–2 are closed; if both close early in #198, manufacturing may begin only as the remaining bounded portion of the pass.

## End state at audit recording
ANNUAL ELECTRICAL RESULT: **WITHHELD**.
PHASE II: **NARROW CARRYOVER**.
CURRENT PDF: **FAIL pending fresh corrected descendant**.
P0 COUNT: **1**.
P1 COUNT: **6**.
SECTIONS 1–8: **READABLE**.
#227: **AMBER**.
