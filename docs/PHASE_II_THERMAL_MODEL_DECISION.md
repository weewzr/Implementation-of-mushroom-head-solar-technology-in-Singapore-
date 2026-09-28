# Phase-II Module-Temperature Model Decision — Continue #188

Status after mandatory Audit 63: **PROVISIONAL PHASE-II MODULE-TEMPERATURE BASELINE — SELECTED-MODULE SIMPLE NMOT RELATION.** Faiman model form is retained as the primary wind-sensitive model-form sensitivity, but direct NASA POWER WS10M plus generic Faiman coefficients is not an authorized physical baseline. No annual electrical geometry result is authorized.

## 1. Project requirements
The thermal layer must accept plane-of-array irradiance `G_poa_w_m2`, ambient dry-bulb temperature `T_amb_c`, and, when the model uses it, wind speed `wind_speed_m_s`; return module temperature in degC for the frozen electrical kernel; remain geometry-agnostic at its API boundary; expose mounting/ventilation assumptions; be inexpensive at hourly annual scale; use traceable coefficients; support sensitivity analysis; have transparent limiting behaviour; and reject invalid/non-finite inputs rather than invent missing values.

The current NASA POWER DEVELOPMENT_NOT_SERIS source provides 8,784 hourly 2024 records after Rust QC: irradiance pathway inputs, MERRA-2 T2M ambient temperature in degC, and MERRA-2 WS10M wind speed in m/s at 10 m. WS10M is not silently interpreted as module-height wind. Any height/local-flow correction is a separate explicit sensitivity/engineering assumption.

## 2. Candidate comparison

| Model | Equation | Inputs / coefficients | Evidence & context | Strengths | Limitations / project use |
|---|---|---|---|---|---|
| Simple datasheet NMOT relation | `T_m = T_a + (T_NMOT-T_a,NMOT) G_POA/G_NMOT` | G_POA W/m2, T_a degC; selected module NMOT 42 +/- 3 degC; G_NMOT=800 W/m2; T_a,NMOT=20 degC | Canadian Solar CS6.2-48TM-H datasheet; NMOT condition also states AM1.5 and 1 m/s wind | module-specific, minimal, exact zero-irradiance limit, trivial implementation | no time-varying wind response; implicitly carries NMOT test ventilation/mounting; use as reference/sensitivity model, not sole Phase-II baseline |
| Faiman steady-state | `T_m = T_a + G_POA/(U0 + U1 v_w)` | G_POA W/m2, T_a degC, v_w m/s; U0 W/m2/K, U1 W/m3*s/K | Faiman 2008; Sandia PVPMC reports seven glass-front/Tedlar-back modules, combined fit U0=25, U1=6.84 | simple, interpretable heat-loss denominator, uses available wind, inexpensive, strong limiting tests | coefficients are literature-generic rather than specific to CS6.2-48TM-460H; wind reference/local-flow/mounting transfer is uncertain |
| Sandia module-temperature model | `T_m = T_a + G_POA exp(a+b v_w)` | G_POA, T_a, wind; empirical a,b by module construction/mounting | Sandia Array Performance Model / PVPMC; representative open-rack glass/cell/polymer coefficients exist | wind-sensitive and mounting-class explicit | empirical coefficients are construction/configuration classes rather than selected-module-specific; no evidence advantage over Faiman for the present baseline |

The NREL/SAM NOCT cell-temperature model was also reviewed. It includes efficiency/absorptance and explicit wind-height and standoff adjustments, but therefore introduces additional parameters/assumptions not supplied by the selected module datasheet. It is not selected as the first baseline.

## 3. Selection
**PROVISIONAL PHASE-II MODULE-TEMPERATURE MODEL: Faiman steady-state model**

`T_m = T_a + G_POA / (U0 + U1 v_w)`

Provisional literature coefficients for the Audit-63 implementation decision:
- `U0 = 25 W m^-2 K^-1`
- `U1 = 6.84 W m^-3 s K^-1`

These are literature-derived generic coefficients, not Canadian Solar manufacturer values and not fitted to project output. They must remain parameterized.

Reason: it is the simplest reviewed model that uses all currently available physically relevant DEVELOPMENT_NOT_SERIS inputs without requiring absorptance/emissivity or other unavailable module parameters. It is steady-state and computationally cheap for hourly annual work. Wind/mounting transfer uncertainty is explicit rather than hidden.

**Primary alternative/sensitivity model:** selected-module simple NMOT relation using manufacturer `NMOT=42 +/- 3 degC`, `G_NMOT=800 W/m2`, `T_a,NMOT=20 degC`. The datasheet also states 1 m/s wind for the NMOT condition. Sensitivity should include the datasheet NMOT range 39-45 degC.

**Secondary model-form sensitivity if needed:** Sandia open-rack glass/cell/polymer module-temperature correlation. Do not promote it unless Audit 63 or later sensitivity evidence warrants a third model.

## 4. Mathematical contract
For Faiman:
- `T_m`: module temperature [degC].
- `T_a`: ambient dry-bulb air temperature [degC], NASA POWER T2M in the development workflow.
- `G_POA`: plane-of-array irradiance [W/m2], produced by the frozen irradiance pipeline.
- `v_w`: nonnegative wind speed [m/s]. Development source is NASA POWER WS10M at 10 m; this provenance must travel with the input.
- `U0`: constant heat-transfer coefficient [W/m2/K], positive finite.
- `U1`: wind-dependent coefficient [W/m3*s/K], nonnegative finite.

Temperature differences in degC and K are numerically identical, so the denominator yields a temperature rise in K that may be added to ambient degC.

Domains/contracts:
- `G_POA == 0` -> `T_m == T_a`.
- negative `G_POA` -> error.
- negative wind -> error.
- non-finite inputs -> error.
- `U0 <= 0`, `U1 < 0`, or non-finite coefficients -> error.
- denominator must be finite and >0.
- missing weather -> upstream missing-data/QC error; no imputation in thermal function.
- extreme but finite predicted temperature is returned with a diagnostic-range flag/check at the calling/QA layer; do not silently clamp.

Geometry dependence: `G_POA` is geometry-dependent upstream. Baseline `U0/U1` are common model parameters. Candidate-specific ventilation/local-flow coefficients are not authorized yet.

## 5. Mounting / ventilation assumption
The baseline is a **generic ventilated/free-standing rigid-module approximation**. It is not asserted to reproduce candidate-specific 3-D airflow. Faiman's published/PVPMC context includes free-exposed modules; the project therefore treats wind-height/local-flow and mounting transfer as model-form uncertainty. Do not assign different U0/U1 to flat, mushroom, hemisphere, faceted or folded candidates without evidence.

NASA POWER WS10M is 10-m wind. No module-height correction is selected in #188. Audit 63 must decide whether direct WS10M is acceptable only as DEVELOPMENT_NOT_SERIS baseline input or whether a separately sourced height adjustment is required before implementation/annual promotion.

## 6. Rust API contract
Create a new module such as `src/thermal.rs`; do not modify frozen electrical equations/API semantics.

Proposed types:
`ThermalModelParameters { u0_w_m2_k: f64, u1_w_m3_s_k: f64 }`
`ThermalInput { poa_w_m2: f64, ambient_temp_c: f64, wind_speed_m_s: f64 }`
`module_temperature_faiman(input, params) -> Result<f64, ThermalError>`

Keep a separate reference function/fixture for the simple NMOT relation rather than embedding it into the frozen electrical kernel.

## 7. Pre-implementation tests
Required before release:
1. zero irradiance -> module temperature equals ambient;
2. higher irradiance, same ambient/wind -> higher module temperature;
3. higher wind, same irradiance/ambient -> lower module temperature when U1>0;
4. finite valid inputs -> finite output;
5. negative irradiance rejected;
6. negative wind rejected;
7. invalid/non-finite U0/U1 rejected;
8. deterministic repeatability;
9. hand case: with G=800 W/m2, T_a=20 degC, v=1 m/s, U0=25, U1=6.84, expected T_m = 20 + 800/(25+6.84) degC (calculate independently in test);
10. units documented in public fields/API;
11. diagnostic plausible-range check must not alter output;
12. NMOT reference model: at G=800 W/m2 and T_a=20 degC, simple NMOT relation returns 42 degC for nominal NMOT.

## 8. Sensitivity/uncertainty plan
Bounded initial sensitivity, after authorization:
- model form: Faiman baseline versus simple module-specific NMOT relation;
- Faiman U0/U1: literature/source uncertainty or defensible ranges must be sourced before sweep;
- NMOT: 42 +/- 3 degC manufacturer range;
- wind: NASA POWER WS10M provenance and later explicit height/local-flow adjustment scenario;
- mounting/ventilation: common ventilated baseline versus later sourced configuration sensitivity;
- ambient temperature/weather uncertainty retained as DEVELOPMENT_NOT_SERIS input uncertainty.

Do not run a broad sweep in #188.

## 9. Source provenance
Authoritative sources reviewed:
- Canadian Solar, CS6.2-48TM-H product datasheet V1.0C25_F23_D1_NA, January 2025: selected 460H row, STC and NMOT electrical data, Pmax temperature coefficient -0.29%/degC, NMOT 42 +/- 3 degC; NMOT test condition 800 W/m2, AM1.5, ambient 20 degC, wind 1 m/s.
- D. Faiman (2008), simple module-temperature heat-loss model; parameters/context summarized by Sandia PV Performance Modeling Collaborative.
- D. L. King, E. E. Boyson, J. A. Kratochvil (2004), Sandia Photovoltaic Array Performance Model; module-temperature correlation summarized by PVPMC.
- NREL SAM Photovoltaic Model Technical Reference (NREL/TP-6A20-67399): NOCT cell-temperature formulation and wind-height/mounting-standoff adjustments.
- Repository `docs/development_weather_nasa_power_2024.md`: NASA POWER Hourly Point API development data, T2M and WS10M provenance, hourly UTC semantics and QC evidence.

## 10. Audit-63 questions for Continue #189
1. Is Faiman sufficiently supported as the provisional common ventilated baseline for DEVELOPMENT_NOT_SERIS annual work?
2. Are generic U0=25 and U1=6.84 acceptable for first implementation, or must module/configuration-specific coefficients be sourced first?
3. May NASA POWER WS10M be consumed directly with explicit 10-m provenance for a development baseline, or is a wind-height transformation mandatory before implementation?
4. Is simple module-specific NMOT the correct primary model-form sensitivity?
5. Does the API keep thermal physics replaceable without changing the frozen electrical kernel?
6. Are the validation fixtures sufficient to authorize implementation at #190?


## Audit 63 disposition — Continue #189
Audit 63 changes the #188 provisional ordering because wind-reference compatibility is not sufficiently closed for generic Faiman coefficients. The exact #190 baseline is:
`T_m = T_a + (T_NMOT - 20 degC) * G_POA/(800 W/m2)`,
with selected-module nominal `T_NMOT=42 degC` and manufacturer sensitivity `39/42/45 degC`.

NASA POWER T2M is the baseline ambient input and must align by canonical UTC timestamp with frozen G_POA. The baseline does not consume wind. NASA POWER WS10M remains explicitly 10-m wind and must not be called module-local wind.

Faiman remains scientifically accepted as a wind-sensitive sensitivity model. Generic `U0=25`, `U1=6.84` may be retained in provenance/tests as literature values, but an annual Faiman sensitivity cannot be promoted until its wind-reference treatment is explicitly compatible/sourced. No geometry-specific U0/U1 is authorized.

**Audit decision B: implementation authorized for #190 with NMOT as baseline and Faiman as sensitivity.**
