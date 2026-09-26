# Mandatory Audit 48 — Continue #148

**Date:** 26 September 2026  
**Trigger:** Continue #148, mandatory Pass 3 audit cycle 48.  
**Scope:** complete paraboloidal-mushroom geometric response surface from Passes #146–147 against the frozen Audit-46 conventional development baseline and Audit-47 Experiment-1 foundations. No new geometry family, thermal/electrical conversion, tracking, economics, or topology optimisation was introduced.

## Disposition

**CORRECTION REQUIRED / FREEZE WITHHELD.**

The response surface did not survive Audit 48 unchanged. Several intermediate defects had already been rejected during Passes #146–147; Audit 48 found additional material release defects in the terminal Pass-147 state. Corrective Rust source has been committed, but the corrected end-to-end evidence run was still executing at audit close and report/traceability parity remained open. Therefore the paraboloid response is not frozen by this audit.

## Frozen foundations checked

- Audit 46 NASA POWER 2024 development baseline remains the input baseline: 8,784 hourly records, UTC, DEVELOPMENT_NOT_SERIS, reconciled direct/diffuse horizontal source accounting, isotropic diffuse, albedo 0.20 development assumption, and midpoint-SPA sensitivity previously bounded.
- Audit 47 remains the accepted Experiment-1 foundation: exact equal-resource correction, canonical Rust visibility/sky-view kernels, and DEVELOPMENT_NOT_SERIS result boundary.
- Audit 48 did not reopen thermal/electrical/tracking/economic/optimisation layers.

## Audit findings

### 1. Flat-limit recovery — PASS after earlier Pass-146 correction
The k=0 row is explicitly the frozen horizontal source state: 1 m2 PV, 1 m2 land, total annual incident energy equal to source annual GHI, zero ground term, and packing/land multipliers equal to one. Earlier finite-mesh flat-limit behaviour was correctly rejected and replaced by the analytical horizontal limit.

### 2. Equal-land/equal-PV resource definitions — FAIL in terminal Pass-147 artifact; source corrected
The terminal artifact labelled `equal_land` reported discrete land area **0.988615929 m2** for every curved case rather than 1.000000000 m2. This is a material fairness defect under the master equal-resource rule. The cause is the polygonal discrete projected area of the finite azimuth mesh. Audit 48 rejects those equal-land numerical rows as release evidence.

Corrective Rust now rescales each curved discrete mesh so its **discrete projected land area** is exactly 1 m2 before the equal-land calculation. The equal-PV branch is then independently rescaled from that mesh to exactly 1 m2 discrete PV area. Assertions enforce both contracts.

### 3. Exact discrete PV/land areas and packing ratio Pi — correction pending execution evidence
The corrected source defines land as the signed upward projected triangle sum `sum(A_i n_z,i)` and PV as `sum(A_i)`; therefore Pi is computed from the same discrete mesh used by irradiance integration. Exact-resource assertions are present. Numerical release remains pending the corrected CI artifact.

### 4. Direct/diffuse/ground decomposition — source logic PASS; release numbers pending corrected resource run
Direct uses DNI, positive incidence and canonical direct visibility. Diffuse uses DHI times the canonical ray-based isotropic sky-view factor. Ground uses GHI times albedo times the standard isotropic ground-view factor. Total is explicitly direct + diffuse + ground. The Pass-147 attribution artifact showed zero direct self-shadow loss for this convex upward paraboloidal cap and small but non-zero sky obstruction, which is physically plausible; those values are not promoted because equal-land normalization changes the absolute energy scale.

### 5. Self-shadowing and sky-view treatment — foundation PASS, response release pending
The response executable calls the frozen canonical Rust `direct_visibility` and `sky_view_factor` kernels from Audit 45/47 foundations. An unobstructed comparator is retained for direct self-shadow and isotropic-sky obstruction attribution.

### 6. Mesh and sky-patch convergence — FAIL in terminal Pass-147 evidence; source corrected
The terminal `convergence.csv` changed mesh density and sky resolution simultaneously, so it could not independently attribute convergence to either discretisation. In addition, `checks.txt` claimed low/boundary probes not present in the generated CSV. Those claims are rejected.

Corrective Rust now emits separate `mesh_convergence.csv` with sky resolution held fixed and separate `sky_convergence.csv` with mesh held fixed, at low/intermediate/high curvature probes k=0.05, 0.5, 1.5 and 3.0. Mesh refinement includes 3x18, 4x24, 5x30 and 6x36; sky refinement includes n=8,16,24,32. The release criterion remains <1% fine-step relative change, with decreasing final mesh increment asserted.

### 7. Refined sampling near apparent bend/crossover — PARTIAL in Pass 147; corrected source pending evidence
The coarse response showed the land-energy multiplier below one at k=0.5 and above one at k=0.75, i.e. a crossover/bend rather than a demonstrated interior peak. Audit 48 forbids describing this as an optimum or peak. Corrective source adds k=0.60, 0.625 and 0.65 plus k=0.05, and asserts only that the unity crossover remains bracketed between k=0.5 and 0.75. No optimisation selection is made.

### 8. Temporal quadrature sensitivity — PASS for terminal Pass-147 model, to be regenerated after resource correction
Four sub-hourly solar-position evaluations at 07:30, 22:30, 37:30 and 52:30 minutes showed annual total changes below 0.1% at the sampled k values in the terminal artifact. The corrected executable retains this test at low, bend, intermediate and high curvature. This is a timing sensitivity only; hourly weather values remain held constant within each source interval.

### 9. Bounded albedo sensitivity — PASS as sensitivity, not validation
Pass-147 evaluated albedo 0.10/0.20/0.30 around the frozen 0.20 assumption. Sensitivity increases materially with curvature because ground-view area increases; the terminal artifact reached about +/-7.0% total-energy sensitivity at k=3.0. This demonstrates parameter sensitivity and does not validate albedo 0.20. Corrected source retains the bounded sweep.

### 10. Numerical stability at high curvature — insufficient explicit evidence in Pass 147; source corrected
The terminal response was finite through k=3 but did not emit an explicit geometry-stability artifact. Corrective source now writes `numerical_stability.csv` at k=0.05,0.5,1.5,3.0 using a 6x36 mesh and asserts finite positive triangle areas, finite normals and positive n_z.

### 11. Machine-readable reproducibility — PARTIAL
Pass-147 CI artifact retained `paraboloid_response.csv`, convergence, robustness, component attribution, checks and SVG plots. Audit 48 corrected the artifact contract to add independent mesh/sky convergence and numerical stability. Corrected machine-readable artifact evidence is pending the currently executing workflow.

### 12. Rust / CI evidence — PARTIAL at audit close
Pass-147 terminal acquisition run **36232019036** passed, but it exercised the defective equal-land response. Audit-48 corrected source commit `f5f2796d8f698678b79744b0d88d5e43a01a9e07` and evidence-trigger commit `6ef33e3437a19494c1e7721f7d8222c8f8d14a89` were created. Corrected acquisition run **36232970658** was still in progress at audit close. No corrected numerical response is released until this run passes and its artifact is inspected.

### 13. Report/code/result parity — FAIL
At audit time the canonical Markdown/LaTeX technical reports and traceability matrix did not yet document the Pass-146/147 paraboloid response-surface result. The master result-release gate therefore fails even if the executable itself passes. Numerical response claims must not be promoted until both reports and traceability are synchronized with the corrected machine-readable evidence and DEVELOPMENT_NOT_SERIS boundary.

### 14. DEVELOPMENT_NOT_SERIS labelling — PASS
The executable CSVs, checks and plots use `DEVELOPMENT_NOT_SERIS`. Audit 48 retains this label. Nothing here is SERIS-measured validation or electrical yield.

## Rejected response features

- Reject the terminal Pass-147 `equal_land` absolute energy rows because discrete land area was 0.988615929 m2, not 1 m2.
- Reject the terminal combined mesh/sky convergence table as proof of independent mesh and sky convergence.
- Reject the stale `checks.txt` statement claiming convergence probes that were not actually emitted.
- Reject any claim of a response **peak**, **optimum**, or preferred k from the present sweep. The observed feature is only a DEVELOPMENT_NOT_SERIS crossover/bend in land-normalised incident energy.
- Do not promote terminal Pass-147 absolute component values after the equal-land correction; regenerate them from the corrected resource mesh.

## Freeze decision

**NOT FROZEN in Audit 48.** The paraboloid geometric-response result may be frozen only after:
1. corrected run 36232970658 (or a directly equivalent later run at the same corrected source) passes;
2. corrected machine-readable artifacts are inspected for exact equal resources, independent low/intermediate/high mesh and sky convergence, refined bend sampling, timing/albedo sensitivity and high-curvature stability;
3. Markdown, LaTeX and traceability matrix are synchronized; and
4. no corrected feature violates the declared numerical robustness criteria.

## Next comparison phase

Not yet authorized. Once the Audit-48 corrective gates close and the paraboloid response is explicitly frozen, the next phase is a **fixed-geometry equal-resource comparison phase** using the same frozen DEVELOPMENT_NOT_SERIS weather/irradiance/visibility framework. It must begin with already-required conventional/fixed candidate comparisons and preserve equal land/PV/height definitions; it must not introduce thermal/electrical conversion, tracking, economics or topology optimisation merely because the paraboloid sweep exists.
