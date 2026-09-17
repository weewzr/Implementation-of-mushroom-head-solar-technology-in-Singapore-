# Project Scope

## Central engineering question

For a constrained horizontal urban footprint in Singapore, determine the three-dimensional photovoltaic geometry, PV packing ratio, orientation distribution and optional movement strategy that maximise useful annual energy generation while accounting for solar geometry, diffuse irradiance, self-shading, bifacial collection, module temperature, wind loading, actuator energy, structure, cost and maintainability.

## Starting hypotheses

1. A sphere is unlikely to be optimal per unit PV material because much of its surface is poorly oriented at any instant.
2. A shallow/faceted mushroom or sparse solar-flower canopy may improve annual generation per horizontal footprint by packing additional PV area into vertical volume.
3. Singapore's equatorial solar path and high diffuse component may reduce the value of continuous tracking relative to spatial orientation and discrete/passive adjustment.
4. The correct objective is not module efficiency alone. Relevant metrics include kWh per square metre of site/footprint, kWh per square metre of PV, lifecycle cost, structural mass and land opportunity cost.

## Key dimensionless quantities

- PV packing ratio: `Pi = A_PV / A_footprint`.
- Packing efficiency: `eta_pack = E_3D / (Pi E_horizontal)`.
- Land multiplication factor: `M_L = Pi eta_pack`.

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

## Status

The exploratory analytical and numerical work from the originating research session is being reconstructed, checked and converted into the full reproducible project described above. Values that came from simplified exploratory calculations will be clearly marked as illustrative until validated with time-resolved Singapore irradiance data.
