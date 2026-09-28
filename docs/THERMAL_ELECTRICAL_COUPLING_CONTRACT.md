# Thermal-Electrical Coupling Contract — prepared at Continue #190

Status: implementation authorized for Continue #191 by Early Major-Result Audit 64. This contract does not authorize annual geometry kWh release.

## Per-timestamp chain
For each canonical timestamp:
1. obtain frozen upstream `G_POA(t)` [W/m2];
2. obtain normalized ambient `T_ambient(t)` [degC] from the same canonical absolute UTC timestamp;
3. evaluate frozen NMOT thermal model -> `T_module(t)` [degC];
4. pass `T_module(t)`, `G_POA(t)` and the provisional rigid-PV electrical design basis to a clean electrical adapter;
5. frozen electrical equations produce ideal/delivered DC, AC, auxiliary subtraction and net power;
6. integrate only with explicit `dt_hours`.

## Timestamp rule
Join by canonical absolute UTC timestamp (`timestamp_utc_s`), never by vector index. Duplicate/missing/nonmonotonic timestamps remain upstream QC errors. No UTC-to-Singapore shift is needed for matching records already represented on the canonical absolute time axis.

## Units
- POA: W/m2 interval-average irradiance.
- ambient/module temperature: degC.
- timestep: hours only at electrical integration boundary.
- power: W.
- integrated energy: Wh.

No hidden unit conversions.

## Electrical design-basis entry
The adapter consumes the Audit-61 provisional rigid module parameters: eta_ref=0.230 for the exact 460H datasheet row, area=1.762*1.134 m2, gamma_P=-0.0029 1/degC and T_ref=25 degC. Production DC losses/inverter/system losses remain unset; ideal-zero-loss values may be used only for deterministic coupling fixtures and must not be called production yield.

## Auxiliary semantics
Auxiliary power remains an explicit separate input. Fixed deterministic fixtures may set it to zero. Moving-system auxiliary energy is not inferred from geometry and remains a later Phase-II engineering layer. Net power may be negative.

## NMOT sensitivity propagation
The same timestamp/input may be evaluated with NMOT 39, 42 and 45 degC. Later annual sensitivity must preserve identical irradiance/weather/electrical assumptions across the three cases and report the change rather than tuning parameters.

## #191 deterministic tests
- reference thermal condition flows into electrical efficiency at the resulting module temperature;
- zero irradiance -> zero gross PV generation after coupling;
- hotter NMOT sensitivity -> lower DC/AC power for negative Pmax coefficient under common nonzero input;
- 39/42/45 ordering is preserved through electrical power;
- explicit auxiliary subtraction/net import remains unchanged;
- time-step integration equals explicit sum;
- invalid thermal input fails before electrical conversion;
- deterministic repeatability;
- adapter does not modify frozen thermal/electrical core semantics.

No annual geometry comparison is authorized by these tests.

## Continue #191 implementation / Early Audit 65
PASS. Deterministic coupling is implemented in `src/thermal_electrical.rs` and frozen in narrow scope by Early Audit 65. The required additive electrical entry point accepts externally supplied module temperature while preserving legacy electrical output equivalence. Canonical all-target Rust evidence: commit `aa04579f2db3a51c55b55ca1afc4c585f47d412a`, run `36365302869` PASS, artifact `10946598195`, SHA-256 `14a8c11299ea5b135194235704790bf10154a8f1bfc0c7da073817a354909683`. Annual adapter remains a separate #192 layer; annual geometry kWh remains unauthorized.
