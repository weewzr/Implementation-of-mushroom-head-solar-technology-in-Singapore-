# Early Major-Result Audit 65 — Continue #191

## Trigger
The deterministic thermal-to-electrical integration layer passed whole-crate all-target evidence and establishes a newly validated model-composition result. Major-result audit is therefore required before freeze or annual-adapter work.

## Frozen-foundation integrity
PASS. Audit-59 fixed irradiance comparison, Audit-64 NMOT thermal foundation, Audit-62 electrical kernel/communication architecture, Audit-56 single crease and Audit-60 accordion/fan fixture remain scientifically unchanged.

One bounded electrical API addition was required because the frozen public `evaluate_step` owned its own NMOT calculation and could not consume externally validated module temperature. Commit `8cc83adca93bfbe54c4178c12a3a75b27283595d` adds `evaluate_step_at_module_temperature` using the identical frozen electrical equations. The legacy path now computes its existing NMOT temperature and delegates to that entry point. Regression test proves identical output for the same temperature. No electrical equation/parameter semantics changed.

## Coupling implementation
`src/thermal_electrical.rs` orchestrates frozen components only. Inputs: POA W/m2, ambient degC, dt hours, auxiliary W, explicit NMOT parameters and canonical electrical design basis. Output exposes module temperature, efficiency, ideal/delivered DC, AC, auxiliary, net power and AC/net timestep energy.

No NASA POWER provider logic or Faiman path is automatically invoked.

## Deterministic validation
First CI at `147840d0181d26d637c9ecf51ee2170aa089714a` failed only because four new test literals used Rust-invalid leading-dot syntax (.02 etc.). Frozen equations were not implicated. Commit `aa04579f2db3a51c55b55ca1afc4c585f47d412a` corrected those literals only.

Canonical evidence:
- exact commit: `aa04579f2db3a51c55b55ca1afc4c585f47d412a`
- `cargo test --all-targets` run: `36365302869` PASS
- artifact: `10946598195`
- artifact SHA-256: `14a8c11299ea5b135194235704790bf10154a8f1bfc0c7da073817a354909683`

Validated fixtures:
- zero irradiance -> T_module=T_ambient, zero gross DC/AC, explicit auxiliary/net import;
- 800 W/m2, 20 degC, NMOT 42 -> T_module 42 degC and independently computed eta/DC/AC;
- 400 W/m2, 30 degC -> T_module 41 degC and independently computed electrical chain;
- 39/42/45 NMOT -> increasing module temperature and decreasing AC power under common nonzero input;
- declared losses do not increase power; P_net=P_AC-P_aux;
- invalid thermal/electrical inputs propagate as errors;
- four-step synthetic series preserves one input -> one output, night zero gross generation, deterministic repeatability and explicit energy sum.

Machine-readable `thermal-electrical-evidence` fixture is DEVELOPMENT_NOT_SERIS and contains only deterministic cases, not annual geometry results.

## Timestamp/annual adapter contract
Annual adapter remains NOT IMPLEMENTED. Contract is `docs/ANNUAL_THERMAL_ELECTRICAL_ADAPTER_CONTRACT.md`: exact canonical UTC timestamp equality, no row-index/nearest-neighbour joins, no silent dropping, explicit hourly interval semantics, night rows retained, 39/42/45 scenario propagation, canonical electrical basis and provenance-rich output schema.

## Audit decision
**FROZEN narrow deterministic thermal-electrical coupling foundation.**
Freeze:
- orchestration API and error propagation;
- additive external-module-temperature electrical entry point with legacy-equivalence regression;
- intermediate output semantics;
- deterministic hand/reference/sensitivity/series fixtures.

Do NOT freeze or authorize:
- annual geometry kWh;
- production inverter/loss assumptions;
- moving-system auxiliary energy;
- Faiman + WS10M baseline;
- mismatch/string/bypass-diode behaviour;
- geometry-specific thermal coefficients.

## #192 authorization and cadence
Continue #192 is authorized for **annual-adapter preparation/evidence only**: implement exact timestamp/schema/provenance join infrastructure and deterministic rejection fixtures without releasing headline annual geometry electrical energy.

Early Audit 65 resets cadence. #192 = Pass 1 of Audit Cycle 66; #193 = Pass 2; normal mandatory Audit 66 = #194 unless another major result triggers earlier. The previous plan expecting a mandatory audit at #193 is superseded by this early-audit reset.

Overall ~#227 target remains achievable.
