# Mandatory Three-Pass Audit 44 — Continue #138

**Date:** 26 September 2026  
**Trigger:** Continue #138, Pass 3 since Audit 43. This is the first explicit Continue in the current ChatGPT session after cross-session state recovery.

## Scope
Complete V1–V5 analytical/discrete verification ladder; current Rust execution failures; reduced SPA implementation against NREL Appendix A.5; paraboloid mesh orientation; area/projected-area/diffuse/direct/time-integration convergence; Markdown/LaTeX/code/traceability parity. No new conceptual or performance material was introduced.

## Material defects found and corrected
1. **Verification workflow command defect.** The exploratory workflow concatenated two `cargo test` invocations on one shell line, causing Cargo to fail before tests executed. Corrected to a multiline command and added the method-verification binary.
2. **Paraboloid mesh winding defect.** Executable evidence showed the Audit-43 winding correction was still wrong: the centre fan and annular triangles produced downward normals. Both windings were corrected. Upward-orientation tests now pass.
3. **Projected-area acceptance-test defect.** The test incorrectly required the coarsest 16-sided projected polygon to be within 1% of the circular footprint. The physically correct requirement is convergence under azimuthal refinement plus a fine-grid tolerance. The test now checks monotonically decreasing error and <1% at the finest tested grid.
4. **SPA orbital-radius defect.** The eccentric-orbit denominator used the true anomaly angle itself rather than its cosine. Corrected.
5. **SPA longitude/status defects.** The reduced solar series was treated as if it were Earth heliocentric longitude and received an erroneous 180-degree conversion. This was corrected, and apparent sidereal-time handling was improved. Source comments that called the reduced implementation “reference-grade”/SPA-equivalent were removed because the A.5 gate still fails.
6. **Rust evidence clean-tree defect.** The evidence artifact itself made the post-run working tree appear dirty. The check now ignores untracked evidence output and tests tracked-file cleanliness.
7. **Report/traceability drift.** Markdown, LaTeX, methodology and traceability were synchronized to distinguish closed V1–V5 numerical-method verification from still-open SPA/physical validation.

## V1–V5 gate determination

| Gate | Requirement | Evidence | Audit 44 |
|---|---|---|---|
| V1 | Discrete paraboloid area converges to analytical area | `discrete_area_converges_toward_exact` passes; refinement output generated | **PASS** |
| V2 | `sum(A_i n_z,i)` recovers footprint under refinement and normals are consistently upward | `projected_area_recovers_footprint` and `all_paraboloid_facets_face_upward` pass after winding/test repair | **PASS** |
| V3 | Ideal isotropic diffuse discrete integral converges to analytical benchmark | both diffuse convergence tests pass | **PASS** |
| V4 | Direct-incidence normal/grazing/backside special cases recover exact geometry | `direct_special_cases_match_geometry` passes | **PASS** |
| V5 | Synthetic time integration recovers known energy | constant-power and piecewise-constant exact-integration tests pass | **PASS** |

Canonical executable evidence: GitHub Actions exploratory run **36176587453**, commit `8f652384e60873bf0ab7f817bc82a17bf3c92b5c`: discrete verification **6/6 PASS**, method verification **2/2 PASS**, CSV generation/upload PASS.

## SPA / whole-crate status
The SPA fixture is **not part of V1–V5**; it is the first physical-validation gate.

The reduced `src/spa.rs` implementation is **FAIL** against the declared NREL Appendix A.5 tolerance. After correcting the 180-degree longitude error and orbital-radius defect, the latest observed A.5 azimuth was **194.333758°** versus the pinned target **194.34024°**, an absolute error of approximately **0.00648°**, still above the 0.001° test tolerance. The whole-crate Rust evidence therefore remains **FAIL** because `spa::tests::nrel_a5_reference` fails, even though the other observed library tests pass.

This failure must not be hidden by loosening the acceptance tolerance. The reduced series is not the full NREL SPA/VSOP87 implementation and remains preliminary.

## Report/code parity
- `docs/methodology.md`: V1–V5 now explicitly marked PASS and numerical-method closure boundary stated.
- `report/project_technical_report.md`: V1–V5 closure and SPA-open boundary synchronized.
- `report/project_technical_report.tex`: same numerical-method status synchronized; stale “future mesh calculation” wording corrected.
- `docs/traceability_matrix.md`: isotropic-diffuse and facet verification rows updated; SPA row records the failed A.5 status.
- No annual-yield, candidate-ranking or optimisation result was promoted.

## Closure decision
**Numerical-method verification phase for the present analytical/discrete paraboloid bridge: CLOSED / PASS.**

This closure is deliberately narrow. It establishes that the current triangular discretisation reproduces the analytical area/projection/diffuse benchmarks and exact direct/time special cases. It does **not** validate Singapore annual performance, arbitrary-candidate visibility, sky-view diffuse physics, thermal/electrical conversion, or solar position.

**Physical validation phase: OPEN.** The immediate blocking gate remains authoritative SPA reproduction/benchmarking. Canonical Singapore weather, visibility/sky-view, downstream convergence and uncertainty remain separate later gates.

## Audit disposition
**PASS for V1–V5 numerical-method verification; FAIL for SPA A.5 / whole-crate validation.** Numerical-method verification may close without promoting any physical-performance conclusion.
