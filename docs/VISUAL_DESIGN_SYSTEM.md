# Visual Design System

This document defines the visual language for the mushroom-head solar research project. Scientific accuracy takes precedence over aesthetics, but visual communication is treated as an engineering deliverable.

## Palette
Use these semantic roles consistently across reports and figures. Exact output colours may be adjusted for medium/contrast while retaining the semantic mapping.

| Role | Suggested colour | Purpose |
|---|---|---|
| Solar/direct beam | warm amber `#E9A23B` | sun vectors, DNI, incident beam |
| Diffuse sky | cyan `#4DBBD5` | DHI, sky dome, diffuse contribution |
| Mushroom / proposed 3-D PV | teal-green `#1B998B` | primary candidate geometry |
| Conventional baseline | deep blue `#3B5B92` | flat/fixed baseline |
| Alternative geometry | violet `#7A6FAC` | cone, accordion, other candidates |
| Gain / feasible | green `#3A9D5D` | positive deltas, feasible region |
| Loss / wind / constraint | vermilion `#D95F4A` | losses, overload, penalties |
| Neutral / uncertainty | slate-grey `#6B7280` | confidence bands, secondary context |
| Background | off-white `#F7F8FA` | report panels / diagrams |

Do not use colour as the sole differentiator. Pair with line style, marker, label, hatch or shape. Avoid rainbow/jet colormaps.

## Typography and layout
- Prefer a clean sans-serif for charts/diagrams and a professional report typeface for body text.
- Minimum figure text should remain comfortably readable at final report size.
- Use a strong title, short subtitle/caption and clear axis hierarchy.
- Keep whitespace around figures. Avoid dense legends when direct labelling is clearer.
- Equations inside diagrams must use the same mathematical notation as the report.

## Geometry-render conventions
- Use consistent camera angles for side-by-side candidate comparisons.
- Show footprint boundary subtly.
- Render PV active surfaces distinctly from support structure.
- Show sun vector in amber and surface normals only where explanatory.
- For irradiance heatmaps, include a labelled colour bar with physical units.
- Clearly label conceptual renders versus simulation-derived heatmaps.

## Phase storyboard

### Phase 0 — Foundation
1. Hero concept: original rotating mushroom-head solar idea over a Singapore urban use case.
2. Four-way geometry comparison: flat panel / sphere / mushroom / sparse flower.
3. Research evolution flow: original question → analytical geometry → packing → ray tracing → topology optimisation.

### Phase 1 — Analytical geometry
1. Paraboloid cross-section with $R$, $h$, $r$, local normal and incident sun vector.
2. 3-D paraboloid renders for increasing $h/R$.
3. Plot of $A_{PV}/A_{foot}$ versus $h/R$.
4. Sphere projected-area diagram showing $4\pi R^2$ surface versus $\pi R^2$ silhouette.

### Phase 2 — Singapore solar resource
1. Annual sun-path diagram at Singapore latitude.
2. Representative equinox/solstice solar trajectories.
3. GHI/DHI/DNI time series and monthly summaries once validated data are available.
4. Direct versus diffuse contribution graphic.

### Phase 3 — Equal-resource packing sweep
1. Equal-$\Pi$ side-by-side geometry renders.
2. $M_L(\Pi)$ comparison plot.
3. $\eta_{pack}(\Pi)$ comparison plot.
4. Height and characteristic tilt versus $\Pi$.
5. PV-material efficiency versus footprint efficiency trade-off.

### Phase 4 — Ray tracing
1. Morning/noon/evening shadow renders.
2. Facet-level beam visibility maps.
3. Sky-view-factor maps.
4. Annual irradiation heatmap on each geometry.
5. Difference map relative to conventional baseline.

### Phase 5 — Bifacial and thermal
1. Front/rear irradiance heatmaps.
2. Ground-reflection schematic.
3. Cell-temperature map.
4. Energy-loss waterfall: incident → optical → thermal → electrical → net.

### Phase 6 — Mechanics
1. Free-body diagram of mushroom/faceted canopy.
2. Pivot, centre of mass and aerodynamic centre diagram.
3. Torque versus wind speed.
4. Normal tracking / high-wind / storm-stow states.
5. Discrete movement schedule over solar day.

### Phase 7 — Topology optimisation
1. Generation 0 / intermediate / final geometry triptych.
2. Optimisation convergence curve.
3. Orientation-distribution sphere/rose plot.
4. Final geometry irradiance heatmap.
5. Pareto front: energy density versus cost/mass/wind load.

### Phase 8 — Techno-economics
1. LCOE/cost breakdown.
2. Sensitivity tornado chart.
3. Uncertainty bands on energy and economics.
4. Land-opportunity-value comparison.

### Phase 9 — Final synthesis
1. Polished system architecture illustration in Singapore context.
2. Benchmark matrix with conventional PV and candidate geometries.
3. Key equations + key result graphic.
4. Prototype/test roadmap.

## Caption standard
Every quantitative figure caption should identify: what is plotted; units; geometry/resource constraint; data/model source; major assumptions; and status as analytical, simulated or measured.

## Accessibility
Use sufficient luminance contrast, colour-blind-aware pairings, non-colour encodings, readable text and descriptive captions. All critical scientific information must remain understandable if printed in greyscale.
