# Early Major-Result Audit 59 — Continue #179

## Trigger
Direct inspection of the first correctly retained controlled-comparison bundle from run 36323622005 triggered the mandatory early audit before result promotion.

## Exact evidence
- Commit: `137327c8e54f6001bbac8a72e85a90c7aa58d0a8`
- Run: 36323622005 — PASS
- Artifact: 10933222710, `controlled-comparison-evidence`
- Artifact SHA-256: `4c3ef238d1dd673b0389265adf9e320c1b2c63f5910a02c37f3cd1b8ddc5e51b`
- Weather dataset hash retained in bundle: `f3442ca0c336011c5c61fe434e00e1f7984f9d34afa27d1431fa939f5eac204c`
- NASA POWER Singapore 2024 DEVELOPMENT dataset QC: 8784/8784 samples, zero reported QC issues.

## Direct artifact inspection
The retained bundle contains:
`annual_results.csv`, `monthly_results.csv`, `resource_invariants.csv`, `geometry_mesh_checks.csv`, `matched_packing_feasibility.csv`, `irradiance_convergence.csv`, comparison README/provenance, run status, weather hash and QC summary.

The accepted annual result rows explicitly contain status, resource contract, geometry, 4x24 refinement identity, facet count, PV area, land area, packing ratio, direct/diffuse/ground/total irradiance, PV-normalised irradiance, land-normalised irradiance, packing_efficiency and land_energy_multiplier.

## Recomputed release checks
- All 11 retained accepted annual rows satisfy `land_norm = pv_norm * packing_ratio` within the precision of the retained CSV.
- All annual rows satisfy `total = direct + diffuse + ground` within retained numerical precision.
- All ten equal-land/equal-PV resource postconditions are accepted with absolute errors <= 8.882e-16 against tolerance 1e-10.
- Monthly rows provide 12 months for every equal-land/equal-PV geometry/contract pair and close to the retained annual direct/diffuse/ground/total values within output rounding.
- Matched-packing target Pi=1 is explicitly accepted only for flat_reference; frozen_paraboloid, hemisphere, faceted_canopy and folded_surface are explicitly rejected because uniform scaling cannot alter their intrinsic packing ratio.
- Mesh convergence uses 2x12, 3x18 and accepted 4x24 rows. The declared final-step <2% criterion passes: largest retained penultimate-to-finest magnitude is hemisphere 1.4200044%.
- Sky convergence uses sky_n 4, 8 and accepted 16. The declared final-step <1% criterion passes: largest retained penultimate-to-finest magnitude is folded_surface 0.4587311%.
- Coarse rows remain convergence evidence only; the controlled annual comparison rows use 4x24 and sky_n=16.

## Frozen model scope
The comparison is frozen only as **DEVELOPMENT_NOT_SERIS controlled fixed-geometry irradiance evidence** under the shared frozen model: NASA POWER Singapore 2024 development weather, shared SPA/annual evaluator, hourly midpoint integration, albedo 0.20, explicit visibility/self-shadowing and sky-view treatment, equal-land/equal-PV canonical resource normalization, and candidate definitions present at the audited commit.

It is NOT an electrical-energy result, structural result, economic result, carbon result, tracking result, deployable-origami performance result, or universal geometry recommendation. No geometry is declared an overall winner.

## Decision
**PASS — freeze the controlled fixed-geometry comparison layer as DEVELOPMENT_NOT_SERIS.**

The result is sufficiently traceable and retained for scientific reporting in Sections 5, 9, 10 and 11, subject to the explicit limitations above.

## Communication gate
Separate from the numerical freeze, artifact 10933596872 was re-rendered in full. The former black rectangles are absent, but pages 2 and 3 still show material text overlap in the fair-comparison and origami architecture SVGs. Therefore beginner-first communication architecture remains HOLD; compilation success is not treated as visual acceptance.

## Cadence
This early major-result audit resets passes-since-audit to zero at Continue #179.
