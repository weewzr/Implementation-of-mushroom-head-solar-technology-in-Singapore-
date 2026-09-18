# Mandatory Three-Pass Audit 28 — Continue #90

**Date:** 18 September 2026  
**Trigger:** mandatory Pass 3 / project Continue #90 / session Continue #6  
**Scope:** master-instruction foundation audit after civil/solar-time and SPA-validation-target work.

## Executive outcome

Cycle 28 materially improved the solar-position foundation without promoting preliminary equations to validated annual-yield physics. Continue #88 added an explicit civil-standard-time to mean-local-solar-time longitude/time-zone relation in Rust. Continue #89 selected the authoritative NREL Reda–Andreas SPA publication as the high-fidelity validation target while deliberately withholding remembered benchmark numbers.

This audit found a documentation/traceability drift: the matrix still said civil-time conversion was incomplete without representing the newly implemented mean-local-solar-time layer. That row has been corrected and a dedicated civil-to-mean-solar-time traceability row added.

No geometry ranking, annual-yield result or topology optimisation is authorised yet.

## Audit findings

| Gate | Finding | Status / corrective action |
|---|---|---|
| Governance cadence | Continue #90 is Pass 3; audit performed before further substantive expansion. | PASS |
| Session rotation | Session Continue #6 of 12. | PASS — no rotation |
| Falsifiability | Mushroom/sphere/canopy remain candidates, not presumed winners. | PASS |
| Rust-only computation | New computational implementation remains Rust. | PASS |
| Civil-time semantics | Rust now separates civil standard time from mean local solar time using longitude and UTC-offset standard meridian. | SOURCE FOUNDATION PASS |
| Apparent solar time | Equation-of-time / higher-fidelity astronomical correction is not implemented. | OPEN |
| SPA validation | Authoritative NREL Reda–Andreas SPA target is recorded; benchmark fixture has not yet been independently extracted/checked. | OPEN |
| Traceability | Matrix lagged the new civil-to-mean-solar-time implementation. | CORRECTED in audit |
| Solar azimuth/elevation | Preliminary spherical relations and analytical source tests remain clearly labelled preliminary. | PASS at source semantics; validation OPEN |
| Irradiance closure | Source diagnostic/tests exist; qualifying execution evidence remains open. | OPEN |
| Whole-crate Rust evidence | New civil-time/solar-position changes have no qualifying retained execution evidence yet. | OPEN |
| Canonical Singapore data | Measured time-correlated canonical GHI/DHI/DNI remain unbound. | OPEN |
| Site definition | Latitude/longitude and source-justified coordinate tolerance remain unfrozen. | OPEN BY DESIGN |
| Equal-resource fairness | Common resource contract remains intact; no candidate bypassed it. | PASS |
| Constants/provenance | UTC-offset/longitude relation is definitional; no Singapore site longitude was invented. | PASS |
| Optical completeness | Shadowing, anisotropic diffuse/sky view and bifacial rear irradiance remain incomplete. | OPEN |
| Thermal/electrical | Sourced module/thermal/inverter model remains incomplete. | OPEN |
| Wind/structure/mechanics | Sourced loads/materials/actuation model remains incomplete. | OPEN |
| Numerical convergence | Mesh/sky/ray/time-step/optimiser convergence evidence remains incomplete. | OPEN |
| Uncertainty/sensitivity | Required before comparative design conclusions. | OPEN |
| Report parity/PDF | Markdown/LaTeX parity, compile and page-by-page PDF QA remain incomplete. | OPEN |
| Public-repository safety | No restricted raw data or credentials introduced. | PASS |
| Result-status discipline | No preliminary solar relation or exploratory geometry output promoted to validated Singapore performance. | PASS |

## Corrective work completed

1. updated solar-hour-angle traceability to state that it consumes apparent solar time;
2. added a dedicated traceability row for civil standard time to mean local solar time;
3. preserved the explicit missing equation-of-time/apparent-time step;
4. preserved the requirement for independently checked SPA benchmark fixtures and qualifying Rust execution evidence.

## Immediate next priorities

1. independently extract and verify an authoritative NREL SPA example/benchmark fixture, recording every input and output convention;
2. map SPA azimuth/zenith conventions explicitly into project ENU conventions;
3. implement benchmark tests only after the reference fixture is checked;
4. obtain qualifying whole-crate Rust execution evidence for the solar-time/azimuth/closure changes;
5. continue canonical Singapore weather/site provenance in parallel;
6. keep annual geometry comparisons and topology optimisation paused.

## Audit decision

Audit 28 occurs on project Continue #90. Passes since audit reset to **0**. Session Continue count is **6**. Rotation is **not** required.

The next explicit Continue is project **#91 / Pass 1 of audit cycle 29**.
