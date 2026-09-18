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

## Current findings
- A sphere is an important baseline but is unlikely to be PV-material efficient.
- Curvature does not intrinsically increase cell efficiency; its possible value is spatial packing and orientation diversity.
- First-pass models show land-normalised collection rising while energy per square metre of PV falls as curvature increases.
- The early model is intentionally idealised and omits self-shading, anisotropic diffuse light, detailed thermal/electrical losses, wind structure and lifecycle cost.
- Angular momentum is not the useful tracking design objective. Slow quasi-static movement, balance, low inertia, low friction, wind-load reduction and storm stow are more relevant.
- The leading design hypothesis is a sparse, faceted, bifacial canopy, potentially with little or discrete movement.

## Repository structure
- `report/` — Markdown and LaTeX technical reports
- `equations/` — governing equations, derivations and nomenclature
- `src/models/` — physical/geometry models
- `src/analysis/` — reproducible numerical analyses
- `src/optimisation/` — optimisation objectives and future solvers
- `src/visualisation/` — figure generation
- `plots/` — generated plots
- `data/` — model outputs and data provenance
- `references/` — BibTeX bibliography
- `docs/foundation/` — originating project prompts/reasoning
- `docs/MASTER_INSTRUCTIONS.md` — persistent quality and project requirements

## Reproducing the current analysis
```bash
# Canonical implementation is Rust. See Cargo.toml and the Rust source tree.
cargo test
cargo run --release

# Current Rust scope: analytically checked paraboloidal geometry.
# Higher-fidelity irradiance/ray-tracing/optimisation layers are added only
# after provenance, equations and validation tests are established.
```

## Technical report
- [Markdown report](report/project_technical_report.md)
- [LaTeX source](report/project_technical_report.tex)

## Mathematical rendering
All important mathematics must use native math notation and be checked in every target format. GitHub Markdown uses `$...$` and `$$...$$`; LaTeX must compile cleanly; PDF/DOCX equations require visual render inspection. See the master instructions for the full quality gate.

## Data
No proprietary SERIS measurements are redistributed. Model-generated outputs are explicitly distinguished from measured data.

## Current limitations
The model still requires time-correlated Singapore GHI/DHI, anisotropic diffuse transposition, full sky-view ray tracing, bifacial rear irradiance, detailed thermal/electrical modelling, structural/wind constraints, array spacing and lifecycle economics.

## Next milestone
Sweep PV packing ratio $\Pi=1$ to $4$ for flat, accordion, cone, paraboloid, sparse-facet and free-form geometries under identical constraints. Then optimise a 20-facet fixed topology before considering discrete tracking.


## Master-instruction compliance
A dated compliance audit is maintained under [`docs/audits/`](docs/audits/). The current repository is explicitly a work in progress: the audit identifies incomplete LaTeX parity, bibliography/provenance, full Rust migration, equal-resource baselines, uncertainty/sensitivity analysis, and final compiled PDF page-by-page QA as required gates before project completion.


## Traceability and governance
- [Full master-instruction audit](docs/audits/2026-09-18_full_master_instruction_audit.md)
- [Mandatory audit ledger](docs/audits/AUDIT_LEDGER.md)
- [Equation–code–evidence traceability matrix](docs/traceability_matrix.md)

The repository is not yet at final-report quality. Quantitative performance claims remain exploratory until the traceability, validation, uncertainty and PDF quality gates in the master instruction are satisfied.
