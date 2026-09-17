# 3-D Solar Geometry Optimisation for Land-Constrained Singapore

## Overview
This repository investigates whether three-dimensional photovoltaic geometries can improve solar-energy yield per constrained horizontal footprint in Singapore. The project began from a "mushroom-head" rotating PV concept, but the research question is deliberately broader: **what geometry, packing ratio and movement strategy maximise useful annual solar yield after optical, thermal, mechanical, structural and economic penalties are included?**

The current result is not a claim that a mushroom is superior. The emerging hypothesis is that the valuable mechanism is **3-D PV packing**: increasing active PV area per unit scarce footprint while retaining irradiation quality, sky view, bifacial rear access, ventilation and continued use of the land below.

## Motivation
EMA reports average annual solar irradiance of about 1,580 kWh/m²/year. Singapore reached 2 GWp installed solar capacity in 2025 and has raised its 2030 target to 3 GWp. SERIS identifies land scarcity as a major constraint and researches multiple-use deployments including solar canopies.

## Research question
For a constrained horizontal footprint in Singapore, what three-dimensional PV geometry and movement strategy maximises useful annual energy and lifecycle value?

## Principal model
The core dimensionless relation is

$$M_L=\Pi\eta_{pack},$$

where $\Pi=A_{PV}/A_{land}$ is the PV packing ratio and $\eta_{pack}$ is average packed-PV productivity relative to the baseline. A geometry is useful only if increased packing outweighs lost irradiation quality and added cost.

For the analytical paraboloidal cap,

$$z(r)=h\left(1-\frac{r^2}{R^2}\right),\qquad
\frac{A_{PV}}{A_{foot}}=\frac{(1+4(h/R)^2)^{3/2}-1}{6(h/R)^2}.$$

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
- `docs/` — methodology roadmap

## Reproducing the current analysis
```bash
PYTHONPATH=. python src/analysis/first_pass.py
PYTHONPATH=. python src/visualisation/make_plots.py
```

## Technical report
- [Markdown report](report/project_technical_report.md)
- [LaTeX source](report/project_technical_report.tex)

## Data
No proprietary SERIS measurements are redistributed. Model-generated outputs are explicitly distinguished from measured data.

## Current limitations
The model still requires time-correlated Singapore GHI/DHI, anisotropic diffuse transposition, 3-D ray tracing, bifacial rear irradiance, detailed thermal/electrical modelling, structural/wind constraints, array spacing and lifecycle economics.

## Next milestone
Sweep PV packing ratio $\Pi=1$ to $4$ for flat, accordion, cone, paraboloid, sparse-facet and free-form geometries under identical constraints. Then optimise a 20-facet fixed topology before considering discrete tracking.
