# Mandatory Audit 63 — Continue #189

## Scope and cadence
Mandatory three-pass audit after Early Major-Result Audit 62. Continue #189 is new-session Continue 5/12. No thermal implementation or annual electrical geometry result was executed.

## Frozen-foundation integrity
PASS. Audit-59 fixed comparison remains FROZEN DEVELOPMENT_NOT_SERIS; Audit-62 beginner-first communication architecture and narrow electrical kernel remain frozen; Audit-56 single crease and Audit-60 accordion/fan fixture remain frozen. No regression evidence was found and none is reopened.

## 1. Faiman equation and coefficient evidence
The Faiman steady-state equation is accepted as a scientifically legitimate model form:
`T_m = T_a + G_POA/(U0 + U1 v)`.
Sandia PVPMC defines T_m/T_a in degC, POA irradiance in W/m2, wind in m/s, U0 in W/m2/K and U1 in W/m3*s/K. It reports Faiman's seven-module experiment: all modules had glass fronts and Tedlar backs; U0 ranged 23.5-26.5 and U1 6.25-7.68, with combined fit U0=25 and U1=6.84. Later literature describes the test/configuration as free-standing/open-rack silicon-module work rather than universal coefficients.

The equation is accepted; the generic coefficient set is **NOT frozen as project-specific truth**.

## 2. Transferability
A common thermal model is scientifically useful as a first-order comparison assumption only if the report states that it holds thermal heat-transfer behaviour fixed while isolating irradiance/electrical conversion. It must not be interpreted as proof that flat, mushroom, hemisphere, faceted and folded geometries have identical real convection.

No geometry-specific U0/U1 is authorized. Candidate-specific thermal coefficients/local-flow treatment remain later sensitivity/engineering work.

## 3. NASA POWER wind decision
Repository evidence: NASA POWER DEVELOPMENT_NOT_SERIS provides MERRA-2 WS10M in m/s at 10 m, hourly UTC.

Authoritative Faiman implementation guidance states that wind speed must be measured at the same height for which the wind-loss factor was determined. The generic U0=25/U1=6.84 coefficients therefore cannot be paired with NASA POWER WS10M and represented as a physically matched coefficient/wind system without an explicit supported wind treatment.

No site roughness/local-flow parameters are presently frozen, and 3-D candidate airflow would add further geometry dependence. An ad-hoc wind-height transformation is therefore not authorized.

**Decision: do not use direct WS10M + generic Faiman coefficients as the Phase-II baseline.** Faiman remains the primary wind-sensitive model-form sensitivity and later engineering model. A future Faiman run must either (i) use coefficients paired to a documented compatible wind reference, or (ii) use an explicitly sourced wind-height/local-flow transformation with uncertainty.

## 4. NMOT baseline audit
Canadian Solar CS6.2-48TM-460H remains the provisional rigid-PV research design basis. Manufacturer datasheet evidence supports NMOT 42 +/- 3 degC. The declared NMOT condition is 800 W/m2 irradiance, AM1.5, ambient 20 degC and wind 1 m/s. This is NMOT, not STC. STC remains 1000 W/m2, cell temperature 25 degC and AM1.5 for electrical rating.

For the first Phase-II baseline use the transparent module-specific linear reference relation:
`T_m = T_a + (T_NMOT - T_a,NMOT) * G_POA/G_NMOT`
with nominal T_NMOT=42 degC, T_a,NMOT=20 degC, G_NMOT=800 W/m2. At G_POA=0, T_m=T_a. This is a modelling relation anchored to the manufacturer NMOT property, not measured hourly module temperature and not a claim that hourly wind/mounting equals the NMOT test condition.

The manufacturer +/-3 degC NMOT range must be retained as a thermal sensitivity. The NMOT test's 1 m/s wind condition is provenance/context, not a time-varying wind input to the simple relation.

## 5. Weather compatibility
PASS for the NMOT baseline and future sensitivity work. Canonical weather records use explicit-offset timestamps and absolute UTC seconds. NASA POWER acquisition explicitly requests UTC and documents hourly timestamps as start-of-hour for whole-hour averages. The 2024 leap-year development set contains 8,784/8,784 records with zero QC issues, duplicates, nonmonotonic timestamps, negative irradiance or implied missing samples. T2M is degC; WS10M is m/s at 10 m. Hourly irradiance energy is converted under the documented one-hour equivalence to interval-average W/m2. The frozen annual irradiance pipeline must join thermal inputs by canonical absolute timestamp, never by array index alone. No local-time shift is authorized.

## 6. Thermal API decision
Implementation at #190 must be separate from `src/electrical.rs`.

Required architecture:
- `ThermalInput { poa_w_m2, ambient_temp_c }` for the baseline;
- `NmotParameters { nmot_c, nmot_reference_ambient_c, nmot_reference_irradiance_w_m2 }`;
- explicit `module_temperature_nmot(...)`;
- a separate future `FaimanParameters { u0_w_m2_k, u1_w_m3_s_k, wind_reference }` / Faiman function may be implemented only as sensitivity infrastructure if its wind provenance is explicit and it is not promoted as baseline;
- frozen electrical kernel consumes module temperature and does not know the thermal-model source.

Validation: finite inputs; G_POA>=0; finite ambient; positive finite NMOT reference irradiance; finite NMOT/reference temperatures; finite output. No hidden clipping/tuning. Plausibility ranges are diagnostics only.

## 7. Test contract
NMOT implementation must test:
1. G=0 -> T_m=T_a;
2. manufacturer reference condition G=800, T_a=20, nominal NMOT=42 -> T_m=42 degC;
3. higher G at fixed ambient -> higher T_m for NMOT>T_a,NMOT;
4. invalid/nonfinite parameters rejected;
5. negative irradiance rejected;
6. independent hand calculation;
7. deterministic repeatability;
8. explicit units.

Faiman sensitivity implementation, when authorized for use, must additionally test zero irradiance, irradiance monotonicity, wind monotonicity for U1>0, hand calculation, invalid/nonpositive denominator, negative wind/irradiance rejection, finite output and deterministic repeatability.

Cross-model results must be reported as model-form sensitivity, never tuned to agree.

## 8. Required later sensitivity diagnostics
Before annual electrical conclusions, compare the authorized NMOT baseline with a wind-sensitive Faiman treatment after wind compatibility is resolved. Diagnostics: annual mean module temperature, irradiance-weighted mean module temperature, high-percentile module temperature, maximum modelled temperature, and annual electrical-energy sensitivity. NMOT 39/42/45 degC is the first bounded parameter sensitivity.

## 9. Report parity
Required correction from #188: Section 15 must identify NMOT as the Audit-63 baseline and Faiman as pending wind-compatible sensitivity. The thermal bridge must not imply that WS10M is a validated module-local wind measurement. DEVELOPMENT_NOT_SERIS status remains explicit.

## 10. Schedule
No schedule extension. #190 implements/tests the NMOT baseline and may scaffold Faiman only if wind-reference metadata is explicit and no annual promotion occurs. #191 thermal-electrical coupling; #192 Audit 64; #193 annual controlled electrical integration only if authorized; #194 mismatch; #195 Audit 65; #196 auxiliary/deployability penalties; #197 Phase-II closure. Overall ~#227 remains achievable.

## Audit 63 decision
**B. THERMAL IMPLEMENTATION AUTHORIZED WITH NMOT AS BASELINE AND FAIMAN AS SENSITIVITY.**

Exact #190 baseline:
- relation: `T_m = T_a + (T_NMOT - 20 degC) G_POA/(800 W/m2)`;
- nominal selected-module NMOT: 42 degC;
- bounded manufacturer sensitivity: 39/42/45 degC;
- input ambient: NASA POWER T2M [degC], aligned by canonical UTC timestamp;
- input irradiance: frozen pipeline G_POA [W/m2];
- wind: not consumed by baseline; NASA POWER WS10M remains available but is not module-local wind;
- Faiman: model form accepted, generic U0=25/U1=6.84 retained only as literature sensitivity candidates pending compatible wind-reference treatment.

Implementation is authorized for Continue #190. Annual geometry kWh remains unauthorized.
