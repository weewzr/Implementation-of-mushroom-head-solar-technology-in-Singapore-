# Project Scope

> **Governance:** All work in this repository is governed by [`MASTER_INSTRUCTIONS.md`](MASTER_INSTRUCTIONS.md). The originating prompt and early reasoning in [`foundation/`](foundation/) are mandatory project context because they explain the repository name and the research lineage.

## Central engineering question

For a constrained horizontal urban footprint in Singapore, determine the three-dimensional photovoltaic geometry, PV packing ratio, orientation distribution and optional movement strategy that maximise useful annual energy generation while accounting for solar geometry, diffuse irradiance, self-shading, bifacial collection, module temperature, wind loading, actuator energy, structure, cost and maintainability.

## Origin
The project began with the question of whether a **mushroom-head rotating solar panel or sphere** could improve solar utilisation in land-constrained Singapore, whether topology optimisation could determine a superior collection geometry, and what momentum/rotation behaviour would be appropriate. That origin remains part of the project even when later analysis challenges the initial concept.

## Starting hypotheses

1. A sphere is unlikely to be optimal per unit PV material because much of its surface is poorly oriented at any instant.
2. A shallow/faceted mushroom or sparse solar-flower canopy may improve annual generation per horizontal footprint by packing additional PV area into vertical volume.
3. Singapore's equatorial solar path and substantial diffuse irradiance may reduce the value of continuous tracking relative to spatial orientation and discrete/passive adjustment; this must be tested with time-resolved data rather than annual-average decomposition.
4. The correct objective is not module efficiency alone. Relevant metrics include kWh per square metre of site/footprint, kWh per square metre of PV, lifecycle cost, structural mass and land opportunity cost.

## Key dimensionless quantities

PV packing ratio:

$$
\Pi=\frac{A_{\mathrm{PV}}}{A_{\mathrm{footprint}}}.
$$

Packing efficiency:

$$
\eta_{\mathrm{pack}}=\frac{E_{3D}}{\Pi E_{\mathrm{horizontal}}}.
$$

Land multiplication factor:

$$
\boxed{M_L=\Pi\eta_{\mathrm{pack}}}.
$$

These definitions separate the benefit of fitting more PV into a footprint from the irradiation penalty caused by tilt, self-shading and occlusion.

## Modelling progression

1. Analytical geometry and limiting cases.
2. Singapore solar-position model.
3. Direct + diffuse plane-of-array irradiance.
4. 3-D visibility / ray tracing and sky-view factors.
5. Bifacial rear irradiance.
6. Thermal/electrical conversion.
7. Wind and tracker mechanics.
8. Geometry/topology optimisation.
9. Techno-economic and sensitivity analysis.
10. Comparison with conventional fixed and tracking PV baselines.

## Mathematical quality requirement
Every report format must render mathematical notation correctly. GitHub Markdown uses `$...$` and `$$...$$`; LaTeX must compile cleanly; final PDF/DOCX equations must be visually inspected for glyph, alignment, overflow and numbering problems. Important derivations must define symbols, state assumptions, check units and limiting cases, and explain physical meaning.

## Status
The exploratory analytical and numerical work from the originating research session is being reconstructed, checked and converted into the full reproducible project described above. Values from simplified exploratory calculations remain explicitly labelled illustrative until validated with time-resolved Singapore irradiance data.
