# Three-Turn Master-Instruction Audit 03

**Date:** 17 September 2026  
**Trigger:** mandatory audit before expansion into new diffuse-sky physics.

## 1. Foundation alignment — PASS
The original Singapore land-constraint / mushroom-head / sphere / topology / momentum prompt remains the project lineage. The mushroom is still the founding analytical geometry, not a presumed winner. The canonical $1\,\mathrm{m^2}$ footprint / $2\,\mathrm{m^2}$ PV thought experiment remains explicitly identified as an engineering test condition rather than an optimum.

## 2. Mathematical rendering — PASS FOR CURRENT PDF / ONGOING REQUIREMENT
The current LaTeX report compiles successfully and its three rendered pages were visually inspected. Existing displayed equations render cleanly. New equations must continue to define variables and units immediately after first presentation.

## 3. Numerical-constant justification — PARTIAL PASS
A dedicated code-parameter register exists. The following remain explicitly provisional numerical settings: ray tolerance $\varepsilon=10^{-9}$, ray-origin offset multiplier 100, mesh resolution, temporal sampling, ground albedo placeholder and bifaciality placeholder. These values must not become physical conclusions. Ray tolerance and offset still require scale-aware convergence.

## 4. Source/provenance — PARTIAL PASS
Exploratory versus sourced values are separated. The final report still requires a systematic citation upgrade for all external Singapore and solar-model facts. No new external physical constant should enter code without the provenance register being updated.

## 5. Reproducibility — PASS WITH ACTIVE NUMERICAL VALIDATION
The PDF build is reproducible in GitHub Actions. The convergence analysis and visualisation are also CI-driven. The first brute-force convergence run exposed a computational bottleneck; the direct-beam blocker calculation has now been vectorised while the original scalar algorithm is retained as a correctness oracle.

## 6. Numerical correctness after acceleration — PENDING CI CONFIRMATION
The accelerated ray tracer must agree with the scalar reference tests before its convergence results are interpreted. Performance improvement alone is not sufficient. The CI run triggered by the vectorisation commit is currently active.

## 7. Visualisation coverage — PASS AT CURRENT PHASE
The convergence phase has a dedicated coloured diagnostic. Quantitative plots remain tied to generated data. The final report still requires the broader visual redesign identified in PDF QA: concept geometry, solar path, mechanics and design-volume diagrams.

## 8. Equal-resource baselines — PASS IN FORMULATION / INCOMPLETE IMPLEMENTATION
Equal packing ratio and footprint are enforced for current candidate meshes. Conventional fixed PV, sphere/hemisphere, tracking and additional baselines must enter the same numerical framework before comparative conclusions.

## 9. Public repository safety — PASS
No credentials or restricted datasets have been committed. Technical foundation records contain project reasoning rather than sensitive personal material.

## 10. Exploratory versus validated conclusions — PASS
The direct-beam geometric score is still labelled as a geometry metric rather than annual Singapore electricity yield. No geometry has been declared optimal.

## Corrections/actions before diffuse-sky expansion
1. Wait for the accelerated convergence CI run and verify reference-equivalence tests.
2. Inspect the convergence CSV and figure when produced.
3. Select a production discretisation only from demonstrated convergence, not convenience.
4. Add scale-aware ray-tolerance/offset sensitivity.
5. Then implement diffuse sky-patch visibility with all angular quadrature constants and variables documented.

## Audit conclusion
The project remains aligned with the master instructions. The correct next scientific gate is numerical equivalence and convergence of the accelerated direct-beam solver; diffuse-sky physics should not yet be interpreted until that gate passes.
