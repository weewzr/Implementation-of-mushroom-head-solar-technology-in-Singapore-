# Mandatory Three-Pass Audit 53 — Continue #163

**Date:** 27 September 2026  
**Status:** FAIL / release withheld pending corrected comparison evidence

## Scope
Audit of the first controlled fixed-geometry annual comparison under the Audit-52-frozen common framework. No new physical model is introduced.

## Evidence inspected
- Original end-to-end run 36306565776 at b61acbcaa808749d9a62934ce02c6b62b7f6f64e: PASS; artifact 10927064983, SHA-256 8d1b6a9ca05435820f178af143c88fc5d92cbd74c52217e241685821ccc353eb.
- Corrected end-to-end run 36306804901 at e1b74cac8cdd51b2c0aababa935427cda4936197: PASS; artifact 10927217639, SHA-256 5107a90ebd673b09d6e7d2655d3ef3ba7deee744cd9e5b8e34964943899b9dde.
- Corrected Rust evidence run 36306804979 at e1b74cac8cdd51b2c0aababa935427cda4936197: PASS; 68 library tests pass plus all-target binaries; artifact 10927337139, SHA-256 302f1861f72e975532d8fb0bbd350f610a7896b6a0d4d82332a33bbfc8135420.

## Findings
1. Shared-pipeline architecture remains intact: fixed candidate topology enters canonical resource_geometry normalization and the common annual_irradiance evaluator using the same NASA POWER 2024, SPA, hourly-midpoint temporal convention, albedo=0.20, visibility and sky-view physics.
2. Equal-land and equal-PV discrete resource postconditions pass for all five candidates at <=1e-10 m2 tolerance. Matched-packing target Pi=1 is feasible only for the flat reference; other frozen topologies are correctly rejected because uniform scaling cannot change packing ratio.
3. Finite/upward geometry and analytical/limiting unit tests pass, including hemisphere convergence toward area/footprint ratio 2, paraboloid shallow-flat limit and folded secant packing relation.
4. Pass-162 added annual candidate mesh/sky convergence and monthly-to-annual/component closure assertions. However, the retained production comparison rows still use the coarse 2x12 mesh. The hemisphere coarse annual equal-land result differs by -4.287% from the 4x24 result, so its production row fails the framework's qualifying convergence requirement. The folded sky sequence is non-monotone; sky_n=8 differs by -0.459% from sky_n=16, which is bounded but warrants use of the refined sky setting for release rows.
5. Therefore no comparative numerical row is promoted or frozen by this audit. Candidate-specific physical decomposition remains exploratory only. Existing annual artifact columns provide packing ratio and PV-/land-normalized irradiance, but explicit packing_efficiency and land_energy_multiplier columns requested for the released comparison are not yet present and must be generated consistently from canonical resources/reference before release.
6. Markdown/LaTeX/traceability still describe the Audit-52 framework and do not yet contain released comparison numbers, so there is no numerical documentation contradiction. The folded deployable/foldable solar-sheet branch remains documented as future manufacturability work without deployment/structural/cost penalties in current irradiance modelling.

## Corrective action
Commit 958dff1e5da12b248e41f6940dd7193526a5adb9 changes production comparison rows to the 4x24 candidate mesh, uses sky_n=16, and adds executable mesh-step (<2%) and sky-step (<1%) acceptance gates. Fresh CI/artifact evidence is required before any comparison release. Explicit packing-efficiency and land-energy-multiplier artifact fields and documentation synchronization remain required after corrected evidence passes.

## Decision
**Controlled fixed-geometry comparison layer NOT FROZEN.** Physical-realism expansion is withheld. DEVELOPMENT_NOT_SERIS remains mandatory. No ranking, winner, optimisation, thermal/electrical conversion, tracking, economics or topology optimisation is authorized. Next pass must inspect corrected CI/artifacts, close metric-output/documentation parity, and only then consider release.
