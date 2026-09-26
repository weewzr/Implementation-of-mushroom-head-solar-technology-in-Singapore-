# Early Major-Result Audit 47 — Continue #145

**Trigger:** first annual mushroom-vs-conventional numerical result is a major project result, requiring an audit before promotion even though only one pass elapsed since Audit 46.

## Scope
Mushroom Experiment 1 only: frozen weather/baseline parity, equal-resource definitions, exact discrete packing, canonical Rust self-shadowing/sky visibility, convergence, output labelling and executable evidence.

## Defects caught before release
1. Initial Rust literal compile defect: rejected before execution.
2. First completed Experiment B incorrectly scaled the mushroom to 2 m² PV, yielding 1.01615 m² footprint rather than the required 1.00000 m². Result rejected and equal-land constraint corrected.
3. Corrected resource run revealed that using the analytical Π=2 height with a finite mesh produced discrete Π≈1.9682. Although energy convergence passed, this contradicted the declared resource ratio. Result rejected.
4. Final code solves paraboloid height independently at each mesh resolution so discrete triangle area / horizontal footprint = **2.000000** to numerical tolerance. Experiment A is exactly 1 m² PV / 0.5 m² land; Experiment B is exactly 2 m² PV / 1 m² land.

## Accepted evidence
- Whole-crate Rust: Actions run **36227632046**, PASS.
- End-to-end frozen POWER acquisition + baseline + Experiment 1: Actions run **36227636308**, PASS.
- Final mesh: 168 facets. 90→168 annual-energy change = ~0.283%, decreasing under refinement.
- Final sky: 1,024 patches. 256→1,024 annual-energy change = ~0.0160%, decreasing under refinement.
- All output rows carry `DEVELOPMENT_NOT_SERIS`.

## Result boundary
Accepted Experiment 1 result is incident irradiance/energy only. No electrical yield, temperature, tracking, economics, optimisation or other candidate geometry is present. Equal-PV and equal-land experiments are reported separately and do not support a universal winner claim.

## Audit disposition
**PASS for release as a DEVELOPMENT_NOT_SERIS Experiment 1 result.** Frozen Audit-46 conventional baseline and earlier geometric foundations show no regression. Because this early audit was triggered by a major numerical result, the passes-since-audit counter resets to zero.
