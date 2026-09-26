# NASA POWER 2024 Conventional-PV Annual Development Baseline

**Status: DEVELOPMENT RESULT — NOT SERIS MEASURED VALIDATION.**

Pass #143 establishes the first end-to-end annual conventional-plane incident-energy baseline. It does not rank or evaluate mushroom/3-D candidates.

## Executable chain
1. Fixed NASA POWER 2024 Singapore query, SHA-256 recorded in the development-weather manifest.
2. Canonical Rust POWER ingestion/QC: 8,784/8,784 leap-year hourly records, zero QC issues.
3. Validated Rust NREL SPA evaluated at each hourly interval midpoint in UTC.
4. Conventional fixed-plane isotropic POA transposition with direct, sky-diffuse and ground-reflected components retained separately.
5. One-hour interval integration to Wh/m².
6. Monthly and annual conservation assertions plus machine-readable CSV/SVG outputs.

Canonical whole-crate Rust evidence for the final baseline source is Actions run **36209058767** (commit `81d5bf02d0a1383168f09ded0aaf89bb205b7bb3`), PASS. End-to-end weather acquisition + annual baseline artifact evidence is Actions run **36209063066**, PASS.

## Dimensional treatment
NASA POWER hourly solar fields are Wh/m² over each hour. Because the interval is exactly one hour, their numerical values equal the corresponding one-hour mean W/m² values, but the stored physical quantity remains interval energy. SPA is evaluated at the interval midpoint for tilted-plane transposition. This midpoint treatment is a development approximation and must not be confused with sub-hourly integration.

The horizontal reference is energy-conserving by construction: annual POA equals source annual GHI. In 453 hours POWER DHI exceeded GHI by a combined 1,258.69 Wh/m²; for the horizontal decomposition only, diffuse is capped at GHI and direct set to zero for those inconsistent source intervals. This reconciliation is reported rather than hidden.

## Verification
- leap-year completeness: **PASS**, 8,784 hours;
- tilted-plane night direct beam after SPA horizon clipping: **0 W/m² maximum**, PASS;
- monthly component sums equal annual component sums: **PASS**;
- horizontal ground-reflected term: **0**, PASS;
- horizontal total equals source GHI annual energy: **PASS**;
- reproducible Rust-only generation: **PASS**.

## Horizontal reference development result
Annual incident energy:
- direct-horizontal reconciled component: **753,273.84 Wh/m²**;
- diffuse-horizontal reconciled component: **892,300.39 Wh/m²**;
- ground-reflected: **0 Wh/m²**;
- total/source GHI: **1,645,574.23 Wh/m² = 1,645.574 kWh/m²**.

These are incident-energy development quantities, not electrical yield.

## Transparent fixed-plane sweep
The development sweep contains one horizontal reference plus tilts 5°, 10°, 15°, 20°, 25°, 30° and 35° at azimuths 0°, 90°, 180° and 270° (clockwise from north). Ground albedo is the explicit development assumption 0.20. The sweep is diagnostic and is **not an optimisation or ranking**.

## Generated artifact files
The successful Actions artifact contains:
- `baseline-results/annual_poa_sweep.csv`;
- `baseline-results/monthly_poa_sweep.csv`;
- `baseline-results/horizontal_monthly_components.svg`;
- `baseline-results/fixed_tilt_azimuth_sweep.svg`;
- `baseline-results/checks.txt`;
- raw POWER CSV, SHA256SUMS and QC summary.

No thermal model, module efficiency, tracking, economics, 3-D geometry comparison or SERIS validation is included.
