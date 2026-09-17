# Three-Turn Master-Instruction Audit 04

**Date:** 18 September 2026  
**Trigger:** user explicitly requested restoration of the mandatory three-turn audit and challenged missing report diagrams.

## Audit finding
The project has drifted from two explicit master requirements: (1) substantive master-instruction checking at least every three project turns, and (2) visualisation coverage at every major phase. Numerical work remained careful, but these two process requirements were not enforced visibly enough.

## 1. Foundation alignment — PASS
The originating Singapore land-constraint, mushroom/sphere/topology and rotation-mechanics questions remain the project basis. The mushroom is a candidate, not a presumed winner.

## 2. Three-turn audit cadence — DEFICIENCY; CORRECTED
The master file requires an audit at least once every three user/assistant project turns. Recent work exceeded that cadence without a recorded substantive audit. This audit resets the cadence. Future project turns must count from this audit and trigger another audit no later than the third subsequent project turn.

## 3. Mathematical rendering — PASS FOR LAST QA'D PDF; RECHECK REQUIRED AFTER VISUAL REDESIGN
The last compiled PDF passed mathematical glyph/layout QA. Any new visual-report build must be rendered and visually inspected again.

## 4. Numerical constants / variables / units — PARTIAL PASS
Equation-by-equation definitions remain mandatory. Rust numerical settings are labelled as computational settings. The preliminary Cooper solar coefficients still require authoritative provenance before validated Singapore-energy conclusions.

## 5. Reproducibility — PASS WITH ONE RECENT CI DEFECT CORRECTED
Rust tests and combined convergence pass. A pipe/tee false-positive issue was discovered and corrected with pipefail/output verification. The orthogonal convergence executable has now been added and must complete successfully before its results are interpreted.

## 6. Visualisation coverage — FAIL; IMMEDIATE CORRECTIVE ACTION REQUIRED
The report does not yet contain the promised professional engineering diagrams. Earlier PDF QA explicitly identified this gap, but numerical work continued too long without closing it. This violates Section 7 of the master instructions.

### Required visual package for the next report iteration
1. **Founding concept comparison** — fixed horizontal panel, mushroom/paraboloid, sphere/hemisphere and tracking concept under the same footprint; conceptual and clearly labelled.
2. **Mushroom geometry derivation diagram** — radius R, height h, footprint area, PV surface area, local normal and paraboloid profile.
3. **Singapore solar-geometry diagram** — ENU axes, solar vector, altitude, azimuth, hour angle and representative near-equatorial path; distinguish conceptual geometry from weather data.
4. **Direct-beam ray-tracing diagram** — receiving facet, blocker facet, centroid ray, visibility V_i, normal n_i and incidence factor.
5. **Packing-ratio visual** — explain Pi=A_PV/A_land and the canonical 1 m2 footprint / 2 m2 PV example without presenting Pi=2 as optimal.
6. **Convergence figure** — spatial and temporal convergence separately, generated from validated CI data.
7. **Mechanical tracking/free-body concept** — rotation axis, centre of mass, actuator torque, gravity, wind force/torque, bearing friction and storm-stow direction.
8. **Project method/validation flowchart** — foundation -> analytical geometry -> Rust ray tracing -> convergence -> irradiance/weather -> electrical/thermal -> mechanics -> topology optimisation -> techno-economics.

Use the semantic colour system already specified in `docs/VISUAL_DESIGN_SYSTEM.md`. Quantitative figures must be code/data-generated; conceptual figures must say `Conceptual — not to scale`.

## 7. Equal-resource baselines — PARTIAL
The framework specifies required baselines, but the current numerical implementation is still concentrated on the canonical paraboloidal mushroom. Baseline expansion remains required before comparative conclusions.

## 8. Evidence/provenance — PARTIAL
No annual Singapore electricity claim is validated yet. Actual Singapore irradiance/weather data and higher-accuracy solar position remain future gates.

## 9. Public repository safety — PASS
No credentials or restricted data identified in current project work.

## 10. Exploratory versus validated conclusions — PASS
The approximately 1.299 direct-beam geometry ratio remains explicitly preliminary and geometry-only, not an annual energy gain.

## Corrective priority
The next major work package is no longer allowed to be purely numerical. It must build the report visual package in parallel with completing orthogonal convergence. The report should not be called professionally redesigned until the diagrams are embedded, captions/provenance are present, LaTeX compiles, and the resulting PDF is visually inspected page by page.
