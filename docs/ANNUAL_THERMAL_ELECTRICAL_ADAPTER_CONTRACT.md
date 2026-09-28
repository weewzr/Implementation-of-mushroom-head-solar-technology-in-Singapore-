# Annual Thermal-Electrical Adapter Contract — prepared at Continue #191

Status: preparation contract only. No annual geometry electrical result is authorized.

## Canonical join
Future annual coupling must join frozen accepted irradiance rows to canonical weather records using the absolute UTC key `timestamp_utc_s`. Vector position, nearest-neighbour matching and implicit UTC/local-time shifts are prohibited.

NASA POWER DEVELOPMENT_NOT_SERIS hourly timestamps are start-of-hour timestamps for whole-hour averages. The canonical weather layer stores the explicit-offset source timestamp plus absolute UTC seconds. The annual irradiance output used for electrical coupling must carry the same canonical timestamp key or a provably identical derived key.

## Join rejection rules
- duplicate timestamp on either side -> error;
- missing irradiance timestamp -> error;
- missing ambient-temperature timestamp -> error;
- nonfinite ambient temperature -> error;
- negative/nonfinite POA -> error under frozen thermal contract;
- no silent row dropping or imputation;
- no nearest-neighbour match.

## Units and interval
- POA: interval-average W/m2;
- ambient/module temperature: degC;
- timestep: hours at electrical integration boundary;
- power: W;
- energy: Wh, converted to kWh only in explicitly labelled output columns.
For the current NASA POWER one-hour development source, hourly Wh/m2 is numerically equivalent to interval-average W/m2 only because dt=1 h. This equivalence must not be generalized to other intervals.

## Required row identity
Every annual output row must retain:
- canonical UTC timestamp;
- geometry identifier;
- resource contract identifier (equal_land/equal_pv or later audited contract);
- irradiance evidence/source lineage;
- weather source lineage;
- NMOT scenario (39/42/45 degC);
- electrical design-basis identifier;
- declared electrical-loss/inverter scenario;
- auxiliary scenario.

## Output schema
At minimum:
`timestamp_utc_s,geometry,resource_contract,nmot_c,poa_w_m2,ambient_temp_c,module_temp_c,efficiency,ideal_dc_w,delivered_dc_w,ac_w,auxiliary_power_w,net_w,dt_hours,net_energy_wh,status`.

## Night handling
A valid zero-POA row remains a row: thermal output equals ambient; gross DC/AC are zero; auxiliary semantics remain explicit and may produce negative net power. Do not drop night rows.

## NMOT sensitivity
39/42/45 degC scenarios must use identical timestamp/weather/irradiance/electrical inputs. Differences are reported as model/design-basis sensitivity, not tuned away.

## Production-parameter gate
The provisional rigid-PV module parameters may be bound from the canonical electrical design basis. Production DC losses, inverter curve/efficiency and moving-system auxiliary consumption remain separate evidence gates. Ideal-zero-loss fixtures must not be presented as annual production yield.

## #192 bounded preparation task
Implement/validate the annual adapter schema and exact timestamp join on a small non-headline fixture or metadata path. Verify accepted frozen irradiance rows can carry the required timestamp/resource/geometry identity and weather can be joined without index assumptions. Define the production-loss/inverter evidence gap precisely. Do not emit headline annual geometry kWh. Prepare Audit 65 evidence for the annual-integration authorization decision.

## Continue #192 implementation evidence
Annual adapter PASS as a strict data-alignment/orchestration layer. Canonical Rust evidence: commit `2ef4ea2b567a0348b43631e20afb6b456fe51120`, run `36367614367` PASS, artifact `10947916945`, SHA-256 `98e052470e41a680347716e30293b30a3ce5ac97f390e2d931338e7e6e223a4c`. Synthetic tests prove exact timestamp joining and rejection of missing, duplicate, one-hour-shifted and nonmonotonic records; accepted-status enforcement; NMOT sensitivity propagation; identity/provenance preservation; explicit Wh/kWh conversion; and frozen coupling invocation.

**Real-schema blocker:** Audit-59 fixed-comparison artifacts contain annual/monthly aggregates, resource invariants and convergence evidence, but no accepted timestep-level candidate POA rows. The shared annual evaluator accumulates hourly contributions internally and returns totals only. Annual electrical coupling MUST NOT reconstruct hourly POA from annual totals. #193 must add a canonical timestep-level irradiance export from the already-frozen accepted 4x24 / sky-16 comparison configurations, preserving timestamp, geometry, resource contract, direct/diffuse/ground/total POA contribution, mesh/sky acceptance lineage and DEVELOPMENT_NOT_SERIS provenance. Its annual sums must reproduce the frozen Audit-59 totals within declared numerical tolerance before Audit 66 can authorize electrical promotion.
