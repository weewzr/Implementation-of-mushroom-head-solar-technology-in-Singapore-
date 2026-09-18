# 3-D Solar Geometry Optimisation for Land-Constrained Singapore

## Overview
This repository investigates whether three-dimensional photovoltaic geometries can improve solar-energy yield per constrained horizontal footprint in Singapore. The project began from the user's **mushroom-head rotating solar-panel** concept—the idea that gives this repository its name—but the research question is deliberately broader: **what geometry, packing ratio and movement strategy maximise useful annual solar yield after optical, thermal, mechanical, structural and economic penalties are included?**

The current result is not a claim that a mushroom is superior. The emerging hypothesis is that the valuable mechanism is **3-D PV packing**: increasing active PV area per unit scarce footprint while retaining irradiation quality, sky view, bifacial rear access, ventilation and continued use of the land below.

> **Project governance:** continuing work must follow the **full 46-part canonical master instruction** in [`docs/MASTER_INSTRUCTIONS.md`](docs/MASTER_INSTRUCTIONS.md). That file deliberately preserves the detailed requirements for reconstruction, first-principles derivation, equation-by-equation definitions, dimensional/sanity checks, physical interpretation, engineering implications, figures, data provenance, uncertainty, optimisation, reproducible computation, Markdown/LaTeX/PDF deliverables, page-by-page PDF QA, GitHub validation and final handover. **A shortened summary is not a substitute for the master instruction.** The originating prompt and early reasoning are preserved under [`docs/foundation/`](docs/foundation/) and are mandatory context for changes in research direction.

## Foundation
The originating question was whether future solar deployment in land-constrained Singapore could use a rotating **mushroom-head** or **sphere**, whether topology optimisation could determine the best sunlight-collection geometry, and what momentum/rotation factor should govern movement.

- [Originating prompt](docs/foundation/01_originating_prompt.md)
- [Early reasoning that steered the project](docs/foundation/02_early_reasoning_record.md)
- [Canonical master instructions](docs/MASTER_INSTRUCTIONS.md)

## Motivation
EMA reports average annual solar irradiance of about 1,580 kWh/m²/year. Singapore reached 2 GWp installed solar capacity in 2025 and has raised its 2030 target to 3 GWp. SERIS identifies land scarcity as a major constraint and researches multiple-use deployments including solar canopies.

## Research question
For a constrained horizontal footprint in Singapore, what three-dimensional PV geometry and movement strategy maximises useful annual energy and lifecycle value?

## Principal model
The core dimensionless relation is

$$
M_L=\Pi\eta_{\mathrm{pack}},
$$

where

$$
\Pi=\frac{A_{\mathrm{PV}}}{A_{\mathrm{land}}}
$$

is the PV packing ratio and $\eta_{\mathrm{pack}}$ is average packed-PV productivity relative to the baseline. A geometry is useful only if increased packing outweighs lost irradiation quality and added cost.

For the analytical paraboloidal cap,

$$
z(r)=h\left(1-\frac{r^2}{R^2}\right),
$$

and

$$
\frac{A_{\mathrm{PV}}}{A_{\mathrm{foot}}}
=\frac{\left(1+4(h/R)^2\right)^{3/2}-1}{6(h/R)^2}.
$$

## Current evidence status

| Statement | Status |
|---|---|
| Curvature does not intrinsically increase photovoltaic cell conversion efficiency; its possible project value is spatial packing and orientation diversity. | Physical interpretation / project framing |
| A sphere provides a useful directionally symmetric comparison geometry but uses substantially more surface area than its projected disk. | Analytical geometry observation |
| Earlier simplified calculations suggested a trade-off between land-normalised collection and productivity per square metre of PV as curvature increased. | Exploratory historical result; not validated Singapore yield |
| Slow solar tracking should not be designed by maximising angular momentum; balance, friction, wind moment, actuator energy, locking and stow are more relevant mechanical quantities. | Mechanics interpretation requiring later quantitative design inputs |
| A sparse, faceted, bifacial canopy may outperform a smooth mushroom after shading, rear access, ventilation and wind are included. | Hypothesis to test, not a conclusion |
| No candidate geometry is currently established as optimal. | Current validated project status |

## Repository structure
- `report/` — Markdown and LaTeX technical reports
- `equations/` — governing equations, derivations and nomenclature
- `src/geometry.rs` — analytical geometry benchmarks
- `src/solar.rs` — preliminary solar-geometry relations
- `src/mesh.rs` — canonical ENU vector/facet representation and direct-incidence kernel
- `src/candidates.rs` — equal-resource candidate/resource contract
- `src/lib.rs` and `src/main.rs` — Rust crate interface and analytical demonstration
- future analysis/optimisation/visualisation modules will be added only when their mathematical and validation foundations are ready
- `plots/` — generated plots
- `data/` — model outputs and data provenance
- `references/` — BibTeX bibliography
- `docs/foundation/` — originating project prompts/reasoning
- `docs/MASTER_INSTRUCTIONS.md` — persistent quality and project requirements

## Reproducing the current analysis
The canonical implementation is Rust. The commands below are **reproduction instructions, not evidence that the current commit has already passed them**. Execution may be described as verified only when a qualifying record has been stored under the project execution-evidence protocol in `docs/rust_execution_evidence_protocol.md`.

```bash
cargo test
cargo run --release
```

Current Rust scope includes analytical geometry, preliminary solar geometry, ENU/facet incidence foundations and the equal-resource candidate contract. Higher-fidelity irradiance, ray-tracing and optimisation layers are added only after provenance, equations and validation tests are established.

## Technical report
- [Markdown report](report/project_technical_report.md)
- [LaTeX source](report/project_technical_report.tex)

## Mathematical rendering
All important mathematics must use native math notation and be checked in every target format. GitHub Markdown uses `$...$` and `$$...$$`; LaTeX must compile cleanly; PDF/DOCX equations require visual render inspection. See the master instructions for the full quality gate.

## Data
No proprietary SERIS measurements are redistributed. Model-generated outputs are explicitly distinguished from measured data.

## Current limitations
The model still requires time-correlated Singapore GHI/DHI, anisotropic diffuse transposition, full sky-view ray tracing, bifacial rear irradiance, detailed thermal/electrical modelling, structural/wind constraints, array spacing and lifecycle economics.

## Current foundation gate
Before any packing sweep or topology optimisation, reconcile Markdown/LaTeX/equations/nomenclature/Rust, eliminate legacy Python references, lock coordinate and sign conventions, complete the typed parameter/provenance register, strengthen analytical benchmarks and traceability, and pass the next master-instruction audit. Higher-fidelity modelling resumes only after these foundations are internally consistent.


## Master-instruction compliance
A dated compliance audit is maintained under [`docs/audits/`](docs/audits/). The current repository is explicitly a work in progress: the audit identifies incomplete LaTeX parity, bibliography/provenance, full Rust migration, equal-resource baselines, uncertainty/sensitivity analysis, and final compiled PDF page-by-page QA as required gates before project completion.


## Traceability and governance
- [Full master-instruction audit](docs/audits/2026-09-18_full_master_instruction_audit.md)
- [Mandatory audit ledger](docs/audits/AUDIT_LEDGER.md)
- [Cross-session project pass counter](docs/audits/PASS_COUNTER.md)
- [Equation–code–evidence traceability matrix](docs/traceability_matrix.md)
- [Typed parameter and provenance register](docs/parameter_provenance_register.md)
- [Rust execution-evidence protocol](docs/rust_execution_evidence_protocol.md)
- [LaTeX compile and PDF visual-QA protocol](docs/pdf_compile_qa_protocol.md)

The repository is not yet at final-report quality. Quantitative performance claims remain exploratory until the traceability, validation, uncertainty and PDF quality gates in the master instruction are satisfied.
