# Phase II Electrical and Engineering Plan — Continues #188–#197

## Phase-II question
**How do the already-audited irradiance differences translate into realistic net electrical-energy differences, and what engineering penalties arise when the geometries become physical systems?**

Phase II consumes frozen Phase-I foundations. It does not reopen Audit-59 irradiance results, the frozen communication architecture, the narrow origami fixtures or the narrow electrical-kernel API/test semantics without genuine regression evidence.

## A. Weather -> module temperature
Select and validate a defensible module-temperature model before annual electrical promotion. Candidate families for #188 evidence review include the simple NOCT/NMOT relation, a Faiman-type wind-sensitive relation, and any other model supported by authoritative literature and by available Singapore development-weather fields.

Selection criteria:
- required inputs and units;
- primary/authoritative source quality and coefficient provenance;
- physical interpretability;
- compatibility with NASA POWER DEVELOPMENT_NOT_SERIS POA/ambient-temperature/wind inputs;
- mounting and wind dependence;
- uncertainty/model-form limitations;
- transparent Rust implementation and independent tests.

No coefficient may be calibrated merely to obtain a desired electrical result.

Required thermal validation contract:
- zero/low irradiance -> module temperature approaches ambient;
- higher irradiance at otherwise identical conditions -> higher module temperature;
- for a wind-sensitive model, higher wind at otherwise identical conditions -> lower module temperature;
- finite valid inputs -> finite output;
- invalid inputs/coefficients rejected;
- temperature and irradiance units explicit;
- independently calculated reference/limiting fixture(s);
- deterministic repeatability.

## B. Irradiance -> electrical-energy coupling
After the thermal model passes its gate, feed frozen irradiance outputs into the frozen geometry-agnostic electrical kernel. Preserve the separation between IRRADIANCE RESULT and ELECTRICAL CONVERSION RESULT. Annual AC/net energy is not promoted until thermal assumptions, module provenance, system-loss assumptions and integration tests are audited.

## C. Nonuniform irradiance / electrical mismatch
Quantify facet-to-facet irradiance nonuniformity first. Define a bounded mismatch model before introducing string/bypass-diode detail. Determine whether the mismatch penalty is material enough to justify a higher-fidelity electrical topology model.

## D. Auxiliary energy
Represent tracking, deployment and storm-stow consumption explicitly and separately. Fixed cases may use zero only where genuinely no moving auxiliary exists. Moving designs must not inherit zero auxiliary consumption in final comparisons.

## E. Deployable engineering realism
Develop bounded requirements for flexible/deployable concepts: minimum bend radius, PV/interconnect strain, hinge/rib/cable requirements, actuation, storm stowage and cycle life. Phase II defines requirements and sensitivities; it does not perform full FEA.

## F. Wind / structural requirements
Establish Singapore-relevant load cases, design requirements and evidence sources. Detailed structural optimisation is later work. Any simplified structural screening must be labelled and uncertainty-bounded.

## G. Phase-II outputs
Subject to validation/audit gates:
- annual gross DC energy;
- annual AC energy;
- annual auxiliary energy;
- annual net electricity;
- PV-area-normalised electrical yield;
- land-area-normalised electrical yield;
- sensitivity and uncertainty outputs.

## Bounded pass budget
| Continue | Bounded task |
|---|---|
| #188 | thermal-model evidence selection, comparison and implementation/test contract; no annual kWh |
| #189 | mandatory Audit 63 and thermal-model validation decision |
| #190 | implement/test selected thermal model |
| #191 | couple validated thermal model to frozen electrical kernel |
| #192 | mandatory Audit 64 / first electrical-coupling review |
| #193 | controlled annual electrical integration on frozen fixed geometries, only if authorized |
| #194 | bounded nonuniform-irradiance/mismatch model |
| #195 | mandatory Audit 65 / electrical-result review |
| #196 | auxiliary/deployability engineering penalties and sensitivities |
| #197 | Phase-II integration/closure or explicit bounded carryover |

The phase does not silently extend beyond #197. Genuine failed validation may rebalance tasks, but the failure and revised bounded allocation must be recorded.

## Exact Continue #188 contract
Do **not** generate annual electrical geometry results. Compare at minimum:
1. simple datasheet NOCT/NMOT temperature relation;
2. Faiman-type wind-sensitive temperature model;
3. another model only if authoritative evidence and available inputs make it genuinely competitive.

For each candidate record:
- equation and dimensional units;
- coefficient definitions and authoritative provenance;
- required weather/mounting inputs;
- whether NASA POWER DEVELOPMENT_NOT_SERIS fields satisfy those inputs without invented conversions;
- wind and mounting sensitivity;
- expected applicability to conventional rigid modules versus unusual 3-D mounting;
- uncertainty/model-form limitations;
- Rust API shape and validation burden.

Select the simplest scientifically defensible model, not the model producing the most favorable energy result. Prepare the Rust function/parameter contract and the thermal validation fixtures listed in Section A. Do not implement the new thermal model during #188; implementation is scheduled for #190 after the #189 Audit-63 validation decision.

## Completion target
Phase II remains bounded to #188–197; #198–207 manufacturing/cost/carbon; #208–217 integrated comparison/uncertainty/recommendations; #218–227 final synthesis/references/appendices/reproducibility/exhaustive QA. Overall target remains approximately #227.

## Scope exclusions through this contract
No Miura-ori optimisation, topology optimisation, full structural FEA, LCOE, headline CO2 reduction, or unsupported flexible-PV performance claim is authorized by this plan.

## Continue #188 thermal decision checkpoint
Provisional baseline selected: Faiman steady-state module-temperature model, pending Audit 63. Primary sensitivity/reference: selected-module simple NMOT relation. Exact evidence/contract is in `docs/PHASE_II_THERMAL_MODEL_DECISION.md`. NASA POWER development weather supplies hourly T2M and WS10M (10-m wind); no wind-height correction is silently assumed. #189 must decide coefficient transferability, wind-height treatment, API/test sufficiency and implementation authorization for #190. No annual electrical result is released.

## Mandatory Audit 63 — Continue #189
Decision B: implementation at #190 is authorized with the selected-module simple NMOT relation as the DEVELOPMENT_NOT_SERIS baseline. Nominal NMOT=42 degC with manufacturer sensitivity 39/42/45 degC; reference irradiance=800 W/m2 and ambient=20 degC. The baseline does not consume hourly wind. Faiman remains the primary wind-sensitive model-form sensitivity, but generic U0/U1 plus NASA POWER WS10M is not promoted until wind-reference compatibility is sourced. #190 implements/tests the separate thermal module only; #191 coupling remains scheduled; annual geometry kWh remains gated.

## Continue #190 / Early Audit 64
Thermal implementation PASS and FROZEN narrow foundation after all-target Rust evidence `36364613741`. #191 coupling is authorized under `docs/THERMAL_ELECTRICAL_COUPLING_CONTRACT.md`; it must remain deterministic/adapter-level and must not release annual geometry kWh. Early Audit 64 resets cadence; normal next audit is #193 unless coupling itself triggers an earlier major-result audit.

## Continue #191 / Early Audit 65
Deterministic thermal-electrical coupling PASS and FROZEN narrow foundation. #192 is rebased to annual-adapter timestamp/schema/provenance preparation and deterministic rejection evidence only. Because Early Audit 65 reset cadence, #192 is Pass 1 of Audit Cycle 66, #193 Pass 2, and normal mandatory Audit 66 is #194 unless an earlier major result triggers. No headline annual geometry kWh before authorization.


## Continue #192 annual-adapter checkpoint
Persisted Early Audit 65 at #191 governs cadence: #192 is Pass 1 of Audit Cycle 66. Strict annual adapter PASS at `2ef4ea2b` / run `36367614367`. No early audit: this is alignment/orchestration, not a new physical result. Real Audit-59 artifacts do not contain timestep-level 3-D POA; #193 is authorized to create/verify a canonical accepted timestep export whose annual component sums reproduce the frozen Audit-59 totals. It may prepare bounded annual coupling evidence for Audit 66 but must not rank/promote annual geometry kWh. Mandatory Audit 66 remains #194.

## Continue #193 dual-track correction
Technical Phase II remains on track, but final-report quality is AT RISK — RECOVERABLE. Every remaining technical pass must update/inspect its corresponding scientific section. Optional branches remain closed until the quality register demonstrates recovery. #194 Audit 66 must audit both timestep/annual pre-audit evidence and the report-quality recovery plan/current LaTeX regression.

## Mandatory Audit 66 — Continue #194
Annual electrical promotion WITHHELD pending one provenance-complete release package: monthly reconciliation against frozen Audit-59 evidence, explicit accepted configuration identifiers in timestep rows, and bounded non-ranked annual adapter evidence. The irradiance physics itself passes same-path annual aggregate-back. #195 closes this gate while report Sections 1-4 are repaired. Optional model branches remain closed because publication-quality recovery is now co-equal with technical work.

## Mandatory Audit 66 — Continue #194
Annual controlled electrical promotion is WITHHELD until the retained accepted timestep export completes and aggregate-back reproduces frozen Audit-59 evidence. Do not modify frozen aggregates. Technical programme remains on track; publication quality is AT RISK — RECOVERABLE and now consumes work in every pass. #195: close timestep artifact/reconciliation blocker if available + Sections 1-4 report repair. Optional high-complexity branches are cut unless essential to the final argument.

## Continue #195
The accepted timestep evidence now reconciles both annually and monthly to the frozen comparison within explicit tolerance, and a provenance manifest binds 4x24/sky-16/equal-resource/weather/model lineage. The annual electrical pre-audit generator is implemented for NMOT 39/42/45 with explicit P_aux=0 and ideal-zero-declared-loss scenario labels; canonical retained workflow evidence is pending and therefore not promoted. Report recovery is now continuous: Sections 1-4 repaired in #195; #196 Sections 5-8; #197 Sections 9-12 + mandatory Audit 67/Phase-II checkpoint. Technical programme ON TRACK; report quality AT RISK — RECOVERABLE; #227 AMBER.
