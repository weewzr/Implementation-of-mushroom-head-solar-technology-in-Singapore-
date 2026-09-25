# Code Parameters, Numerical Settings and Justification Register

This register exists because the master instructions require **every non-trivial numerical value** to be classified and justified. A number appearing in code is still an assumption even when it never appears in the report.

## Classification
- **D — definition/conversion:** exact or conventional mathematical definition.
- **A — astronomical/geometrical approximation:** source-required approximation coefficient.
- **S — sourced physical/empirical value:** requires citation/data provenance.
- **E — engineering design assumption:** requires rationale and sensitivity analysis.
- **N — numerical/computational setting:** requires convergence or numerical-stability justification where material.

## Current register

| Parameter/default | Current value | Units | Class | Location | Rationale/status |
|---|---:|---|---|---|---|
| Singapore preliminary latitude | 1.3521 | degrees north | S | solar-position code | Approximate representative Singapore latitude. Must be tied to an explicit study location or replaced by site coordinates for validated work. |
| Cooper declination amplitude | 23.45 | degrees | A | solar-position code/report | Cooper (1969) approximation coefficient; retain only for transparent preliminary calculations. |
| Cooper annual angular cycle | 360 | degrees | D | solar-position code/report | One complete angular cycle. |
| Cooper year length | 365 | days | A | solar-position code/report | Non-leap-year assumption in correlation. |
| Cooper phase offset | 284 | day index | A | solar-position code/report | Cooper correlation phase shift. |
| Hour-angle rate | 15 | degrees/hour | D | report | $360^\circ/24\,\mathrm h$. |
| Solar-noon apparent time | 12 | hours | D | report | Definition of local apparent solar noon in solar-time coordinates. |
| DNI reconstruction horizon guard | 0.1 | dimensionless $\cos\theta_z$ | N | `src/models/irradiance.py` | Prevents numerical blow-up near horizon. **Not physically validated.** Must be sensitivity-tested/replaced by a robust irradiance decomposition procedure. |
| Ground albedo default | 0.2 | dimensionless | E | `src/models/irradiance.py` | Placeholder engineering assumption. Must not be used for validated Singapore results without site/material justification and sensitivity analysis. |
| Bifaciality default | 0.8 | dimensionless | E | `src/models/irradiance.py` | Placeholder representative assumption only. Final value must come from selected module data/model. |
| Ray intersection epsilon | $10^{-9}$ | geometry coordinate units | N | `src/visibility.rs` | Floating-point tolerance. Must be tested against geometry scale and convergence; should eventually scale with characteristic length. |
| Ray-origin offset multiplier | 100 | dimensionless | N | `src/visibility.rs` | Numerical self-intersection avoidance. Must be replaced/tested with a scale-aware offset. |
| Paraboloid mesh radial divisions | 8 default; 5 in coarse sweep | count | N | `src/models/meshes.py`, direct sweep | Discretisation setting. Requires mesh-convergence study. |
| Paraboloid/cone azimuth divisions | 32 default; 20 in coarse sweep | count | N | mesh/sweep code | Discretisation setting. Requires convergence study. |
| Accordion folds | 4 | count | E/N | mesh/sweep code | Canonical comparison geometry, not an optimum. Must be swept as a design variable and checked for numerical/geometric effects. |
| Coarse annual day step | 10 | days | N | `direct_beam_mesh_sweep.py` | Exploratory computational sampling only. Requires temporal convergence. |
| Coarse daylight time step | 0.5 | hours | N | direct sweep | Exploratory computational sampling only. Requires temporal convergence. |
| Coarse hour range | 06:25–17:45 solar-time equivalent sampling | hours | N | direct sweep | Preliminary geometry sampling. Must be replaced by sunrise/sunset-aware timestamps in validated calculations. |
| Canonical footprint | 1 | m$^2$ | E | topology experiment | Chosen normalization/design experiment, not regulation or optimum. |
| Canonical PV area | 2 | m$^2$ | E | topology experiment | Chosen to test $\Pi=2$ packing. Must be swept. |
| Canonical maximum height | 2 | m | E | topology experiment | Chosen design-domain limit. Must be swept and later tied to use-case constraints. |

## Required convergence programme
Before geometry rankings from the ray tracer are treated as numerical findings, vary at minimum:

$$
N_r,\quad N_\phi,\quad \Delta n,\quad \Delta t,\quad \varepsilon_{\mathrm{ray}}.
$$

where:
- $N_r$ is radial mesh resolution (count, dimensionless integer);
- $N_\phi$ is azimuthal mesh resolution (count, dimensionless integer);
- $\Delta n$ is day-of-year sampling interval (days);
- $\Delta t$ is intraday sampling interval (h or s);
- $\varepsilon_{\mathrm{ray}}$ is ray-intersection numerical tolerance (m when geometry coordinates are in metres).

A numerical setting is acceptable only when further refinement changes the reported comparison metric by less than a declared convergence tolerance. That tolerance itself must be justified before final results.
