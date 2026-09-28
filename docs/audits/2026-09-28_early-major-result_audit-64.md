# Early Major-Result Audit 64 — Continue #190

## Trigger
Continue #190 implemented and obtained fresh whole-crate evidence for the Audit-63-authorized standalone module-temperature layer. This is a new validated model layer, so the master-instruction major-result rule triggers Audit 64 before freeze or thermal-electrical coupling.

## Scope integrity
PASS. No annual geometry electrical result, cost/carbon model, mismatch/string model, Miura optimisation, topology optimisation or detailed structural model was opened. Audit-59 fixed comparison, Audit-62 communication/electrical kernel, Audit-56 single crease and Audit-60 accordion fixture remain untouched.

## Implementation
Standalone `src/thermal.rs` introduced at commit `ab888bc00b78d4b985675c4baf7e261d1ee366b6`, exposed through `src/lib.rs` at `88ab9f1e109b771ed5bc38cbc7184e7d476851c5`. Deterministic evidence fixture introduced at `1dcfe63a7e0d34823ea02aa57bf7f16b09701639`; registered at `8163ef278d70f6cbec64d897a5b3ed9263389278`. CI was strengthened to the requested `cargo test --all-targets` at `80c67038999ef0eb501db9b2662acb2dfcd50007`.

The thermal core has no NASA POWER ingestion dependency. It accepts normalized physical inputs.

## NMOT baseline
Implemented equation:
`T_m = T_a + (T_NMOT-T_a,NMOT) G_POA/G_NMOT`.
Explicit parameters: NMOT degC, reference ambient degC, reference irradiance W/m2. Selected-module helper binds manufacturer-supported nominal 42 degC, 20 degC ambient, 800 W/m2. Manufacturer sensitivity helper returns 39/42/45 degC; these are design-basis sensitivity values, not statistical confidence intervals.

Validation rejects nonfinite input/parameters, negative irradiance and nonpositive reference irradiance. No physical-value clipping or calibration is performed.

## Faiman path
Faiman is implemented only as an explicit sensitivity function with caller-supplied U0, U1 and wind speed. No generic coefficient defaults and no NASA POWER WS10M adapter are supplied. Negative wind/irradiance and invalid heat-loss coefficients/denominator are rejected. This preserves Audit-63 Decision B.

## Tests and evidence
Canonical all-target Rust run: `36364613741` PASS at commit `80c67038999ef0eb501db9b2662acb2dfcd50007`.
Artifact: `10947375679`.
Artifact SHA-256: `bb1f7ff9907ede3fd4b620722ef1e691c01b72d2bbbffe58f5d9ea5b6e7ebac0`.

Tests include:
- NMOT zero irradiance -> ambient;
- manufacturer reference 800 W/m2, 20 degC, 42 degC -> 42 degC;
- irradiance monotonicity;
- 39 < 42 < 45 sensitivity ordering at common input;
- independent hand case 30 degC ambient + 400 W/m2 + NMOT 42 -> 41 degC;
- negative irradiance, zero/negative reference irradiance and nonfinite inputs rejected;
- deterministic repeatability;
- finite Singapore-like diagnostic grid;
- Faiman zero-irradiance/irradiance/wind monotonicity and invalid-input rejection.

No test acceptance threshold was weakened to obtain PASS.

## Evidence fixture
`thermal-evidence` emits machine-readable deterministic DEVELOPMENT_NOT_SERIS CSV rows for zero irradiance, manufacturer reference, independent hand case and 39/42/45 common-input sensitivity. It is not an annual result.

## Report/figure
Thermal bridge figure was updated to show NMOT baseline and Faiman as future sensitivity. Report parity must finish after this audit: mark NMOT IMPLEMENTED/VALIDATED narrow foundation, Faiman sensitivity non-baseline, and retain DEVELOPMENT_NOT_SERIS limitation. No annual yield claim is permitted.

## Decision
**FROZEN narrow thermal-model foundation.** Freeze the NMOT equation/API validation semantics, selected-module nominal/reference helper, 39/42/45 sensitivity contract and first-principles fixtures. Faiman remains non-baseline sensitivity infrastructure; its annual use is not frozen or authorized.

This freeze does not validate candidate-specific convection, module-local wind, dynamic thermal inertia, annual electrical yield or final system performance.

## #191 authorization
Thermal-electrical coupling is authorized for Continue #191 using the frozen NMOT thermal output as input to the frozen electrical kernel. #191 must first define/implement a clean adapter with exact timestamp/unit semantics and deterministic coupling tests. It must not release headline annual geometry kWh. Audit 65 cadence resets here: #191 Pass 1, #192 Pass 2, mandatory Audit 65 at #193 unless a major result triggers earlier.
