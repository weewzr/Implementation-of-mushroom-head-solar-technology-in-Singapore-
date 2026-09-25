# Mandatory Three-Pass Audit 45 — Continue #141

**Date:** 26 September 2026  
**Trigger:** Continue #141, Pass 3 since Audit 44.

## Scope
Freeze/authorization audit of the completed V1–V5 analytical/discrete ladder, NREL SPA Appendix A.5 solar-position gate, and canonical Rust visibility/self-shadowing/isotropic sky-view foundation. No new geometry, optimisation, figures or annual performance claims were introduced.

## Executable evidence
Canonical whole-crate Rust evidence for the latest geometric-irradiance foundation is GitHub Actions run **36178674780**, commit `b4c66751f634750ab72e7656bee329102c658398`.
- canonical library suite: **56 passed, 0 failed**;
- NREL SPA A.5 regression: PASS;
- V1–V5 discrete/method verification: PASS with no regression;
- release executable: PASS, exit status 0;
- tracked working tree after execution: clean;
- evidence artifact upload: PASS.

## Gate audit

| Foundation | Audit evidence | Audit 45 |
|---|---|---|
| V1–V5 analytical/discrete verification | closed in Audit 44; current whole-crate run shows no regression | **PASS / FROZEN** |
| NREL SPA Appendix A.5 | unchanged 0.001° acceptance test passes in current whole-crate run | **PASS / FROZEN** |
| Direct analytical shadow cases | stacked lower facet blocked by upper facet; upper facet and lateral escape unblocked | **PASS** |
| Beam conservation / clipping | unblocked 0.5 m² facet at 800 W/m² normal incidence returns 400 W; blocked counterpart returns 0 | **PASS** |
| Sky-patch solid angle | equal-solid-angle hemisphere sums to 2π sr over refinements | **PASS** |
| Horizontal isotropic sky | unobstructed upward horizontal factor = 1 | **PASS** |
| Vertical isotropic sky | unobstructed vertical factor converges to 1/2; finest declared error <5e-4 | **PASS** |
| Fully blocked sky limit | large overhead plane drives upward sky factor to numerical zero | **PASS** |
| Finite obstruction convergence | finite-roof sky factor shows decreasing refinement change and remains in (0,1) | **PASS** |
| Canonical implementation language | `src/visibility.rs` is canonical Rust implementation | **PASS** |

## Material defect found and fixed
Two documentation locations still pointed to `src/models/facets.py` as the visibility/ray-tracing implementation. This contradicted the canonical Rust status. `docs/irradiance_and_visibility.md` now explicitly identifies `src/visibility.rs` as canonical and labels the Python path legacy/exploratory; `docs/code_parameters.md` now binds ray epsilon/offset parameters to the Rust module.

No numerical defect or regression was found.

## Freeze decision
The **geometric irradiance foundation is CLOSED / FROZEN** for:
1. analytical/discrete geometry verification V1–V5;
2. NREL SPA A.5 solar position;
3. monofacial direct incidence with self-shadow visibility;
4. isotropic hemispherical sky visibility with demonstrated patch convergence and limiting checks.

A frozen gate is reopened only by regression, a changed governing assumption/interface, or evidence that invalidates its acceptance basis.

This freeze does **not** claim anisotropic-sky validation, bifacial rear irradiance, thermal/electrical validation, annual Singapore yield, uncertainty closure or candidate superiority.

## Authorization
With the geometric irradiance foundation frozen, the project is **AUTHORIZED to begin the Singapore Development Weather + annual baseline phase**. Authorization means the next work may acquire/bind a development weather dataset, exercise the existing weather QC/provenance contract, combine validated SPA and geometric irradiance kernels for a baseline-only annual integration, and establish time/data-gap/unit/convergence checks.

No annual performance result is validated or claimed by this audit.
