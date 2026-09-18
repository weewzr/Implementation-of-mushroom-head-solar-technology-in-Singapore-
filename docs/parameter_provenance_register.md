# Typed Parameter and Provenance Register

**Purpose:** single foundation register for numerical inputs, assumptions, exact constants and unresolved parameters. A value appearing here is not automatically validated. The `Status` and `Use gate` columns control how it may be used.

## Status vocabulary

- **Exact/definition** — mathematical or unit definition; no empirical uncertainty.
- **Sourced context** — externally sourced value useful for context, but not necessarily a simulation input.
- **Approximation coefficient** — belongs to a named analytical approximation.
- **Engineering assumption** — chosen to define an experiment; not measured or regulatory.
- **Provisional** — appeared in exploratory work but lacks sufficient verification.
- **Required input** — model requires it, but no canonical value/dataset has yet been selected.
- **Numerical setting** — discretisation/solver choice requiring convergence evidence.

## A. Geometry and equal-resource experiment

| Parameter | Symbol / code | Value or range | Unit | Type | Source / basis | Sensitivity / uncertainty requirement | Use gate |
|---|---|---:|---|---|---|---|---|
| Horizontal footprint budget | $A_{\mathrm{foot}}$ / `footprint_m2` | 1 | m² | Engineering assumption | Foundational canonical comparison cell | Sweep before design conclusions | Equal-resource analytical/numerical comparisons only |
| Active PV-area budget | $A_{\mathrm{PV}}$ / `pv_area_m2` | 2 | m² | Engineering assumption | Foundational canonical comparison cell | Sweep packing ratio; do not assume 2 is optimal | Equal-resource comparisons only |
| Maximum candidate height | $H_{\max}$ / `max_height_m` | 2 | m | Engineering assumption | Foundational canonical comparison cell | Sweep and later constrain by practical/structural requirements | Equal-resource comparisons only |
| Nominal packing limit for canonical cell | $\Pi_{\max}$ | 2 | dimensionless | Calculated from assumptions | $2\,\mathrm{m^2}/1\,\mathrm{m^2}$ | Changes directly with area budgets | Derived metric, not independent input |
| Paraboloid radius | $R$ / `radius` | variable | m | Decision/geometry variable | Candidate geometry | Sweep/constrain under footprint | Analytical geometry allowed |
| Paraboloid height | $h$ / `height` | variable, $0\le h\le H_{\max}$ | m | Decision/geometry variable | Candidate geometry | Sweep; structural constraints later | Analytical geometry allowed |
| Curvature/aspect ratio | $k=h/R$ | derived | dimensionless | Calculated | Geometry definition | Inherits $h,R$ uncertainty | Derived metric |

## B. Solar geometry

| Parameter | Symbol / code | Value or range | Unit | Type | Source / basis | Sensitivity / uncertainty requirement | Use gate |
|---|---|---:|---|---|---|---|---|
| Full angular cycle | — | 360 | deg | Exact/definition | Angular definition | None | General |
| Solar hours per cycle | — | 24 | h | Exact/definition | Solar hour-angle definition | None | General |
| Hour-angle rate | — | 15 | deg h⁻¹ | Exact/definition | $360/24$ | None | General |
| Apparent solar noon | $t_{\mathrm{solar}}$ reference | 12 | h | Exact/definition in solar-time coordinate | Hour-angle definition | Civil-time conversion uncertainty handled separately | General |
| Cooper declination amplitude | — / `23.45` | 23.45 | deg | Approximation coefficient | Cooper (1969) approximation; secondary documentation currently noted | Replace/benchmark with NREL SPA for validated production | Preliminary/educational only |
| Cooper annual period | — / `365` | 365 | day | Approximation coefficient | Non-leap-year Cooper formulation | Leap-year/timestamp effects excluded | Preliminary/educational only |
| Cooper phase offset | — / `284` | 284 | day-index | Approximation coefficient | Cooper formulation | Not tunable project parameter | Preliminary/educational only |
| Geographic latitude | $\phi$ | not yet frozen | rad internally | Required input | Site-dependent | Site choice / coordinate precision | Required before site-specific yield |
| Geographic longitude | — | not yet frozen | deg or rad | Required input | Site-dependent | Required for civil-to-solar-time conversion | Required before timestamped yield |
| Time zone | — | Singapore Standard Time, exact implementation convention not yet encoded | time offset | Required input/convention | Singapore context | Verify timestamp handling and DST assumption | Required before timestamped yield |

## C. Singapore solar/weather context and required time series

| Parameter | Symbol | Value or range | Unit | Type | Source / basis | Sensitivity / uncertainty requirement | Use gate |
|---|---|---:|---|---|---|---|---|
| Average annual solar irradiance context | — | approximately 1,580 | kWh m⁻² yr⁻¹ | Sourced context | Energy Market Authority | Do not substitute for time-resolved GHI/DHI/DNI | Context only |
| Installed solar capacity in 2025 | — | 2 | GWp | Sourced context | EMA 2 Mar 2026 release | Not a physics-model input | Motivation/context only |
| Singapore 2030 solar target | — | 3 | GWp | Sourced context | EMA 2 Mar 2026 release | Policy context may change | Motivation/context only |
| SERIS GHI monitoring stations | — | 25 | station count | Sourced context | SERIS monitoring page | Network metadata should be rechecked when dataset selected | Data-availability context |
| SERIS stations with diffuse/met variables | — | 10 | station count | Sourced context | SERIS monitoring page | Recheck dataset availability/licence | Data-availability context |
| SERIS stated monitoring resolution | — | 1 | s | Sourced context | SERIS monitoring page | Actual acquired dataset resolution may differ | Data-availability context |
| Diffuse share used in early exploration | — | 57 | % | Provisional | Historical exploratory calculation; direct source not verified | Must not be used as fixed annual decomposition | Historical only; prohibited for validated yield |
| GHI time series | $GHI(t)$ | not acquired canonically | W m⁻² | Required input | Authoritative/licensed Singapore dataset required | QC, missing data, temporal resolution | Required for validated yield |
| DHI time series | $DHI(t)$ | not acquired canonically | W m⁻² | Required input | Authoritative/licensed Singapore dataset required | QC and anisotropic diffuse modelling | Required for validated 3-D diffuse yield |
| DNI time series | $DNI(t)$ | not acquired canonically | W m⁻² | Required input | Measured or documented derivation required | Decomposition/model uncertainty if derived | Required for validated direct yield |
| Ambient temperature | $T_a(t)$ | not acquired canonically | °C | Required input | Weather dataset required | Thermal-model sensitivity | Required for validated thermal/electrical yield |
| Wind speed/direction | $v(t)$, direction | not acquired canonically | m s⁻¹, deg | Required input | Weather/design dataset required | Extreme vs operational wind must be distinguished | Required for wind/tracking design |

## D. PV, optical, thermal and electrical parameters

| Parameter | Symbol | Value or range | Unit | Type | Source / basis | Sensitivity / uncertainty requirement | Use gate |
|---|---|---:|---|---|---|---|---|
| Module efficiency used in early illustration | $\eta$ | 23 | % | Engineering assumption / historical | Earlier exploratory calculation; no selected module | Replace with sourced module/model; sensitivity required | Historical only |
| Bifaciality factor | $b$ | not selected | dimensionless | Required input | Selected bifacial module/datasheet or validated model | Sensitivity/module dependence | Required before bifacial result |
| Ground albedo | — | not selected | dimensionless | Required input | Site/surface-dependent source | Seasonal/site sensitivity | Required before reflected/rear irradiance result |
| Reference efficiency | $\eta_{\mathrm{ref}}$ | not selected | dimensionless | Required input | Selected module/model | Module uncertainty | Required before electrical yield |
| Temperature coefficient | $\gamma$ | not selected | K⁻¹ | Required input | Selected module datasheet/model | Thermal sensitivity | Required before temperature-corrected yield |
| Inverter efficiency/clipping | — | not selected | dimensionless / power | Required input | System design/model | Sensitivity to DC/AC design | Required before system-level yield |

## E. Mechanics and structure

| Parameter | Symbol | Value or range | Unit | Type | Source / basis | Sensitivity / uncertainty requirement | Use gate |
|---|---|---:|---|---|---|---|---|
| Mass moment of inertia | $I$ | geometry-dependent; not calculated canonically | kg m² | Required calculated input | Geometry + mass distribution | Material/structure uncertainty | Required before actuator sizing |
| Drag coefficient | $C_D$ | not selected | dimensionless | Required input/model output | Geometry/Reynolds/orientation dependent | High sensitivity; CFD/standards/experiments may be needed | No validated wind load yet |
| Air density | $\rho$ | not selected | kg m⁻³ | Required input | Environmental/design condition | Temperature/pressure dependence | Required before wind force calculation |
| Design/operational wind speed | $v$ | not selected | m s⁻¹ | Required input | Appropriate Singapore design/operational basis required | Extreme-value/design uncertainty | Required before structural conclusion |
| Centre-of-pressure arm | $r_{\mathrm{CP}}$ | geometry-dependent | m | Required calculated/model input | Aerodynamic load distribution | Orientation dependence | Required before wind torque conclusion |

## F. Numerical settings

| Setting | Symbol / code | Current value | Unit | Type | Evidence required before validated use |
|---|---|---:|---|---|---|
| Facet/mesh resolution | — | not frozen | facet/triangle count | Numerical setting | Mesh convergence study |
| Sky discretisation | — | not implemented | patch count | Numerical setting | Sky-patch convergence study |
| Ray tolerance | — | not implemented | m or dimensionless, method-dependent | Numerical setting | Geometric robustness + sensitivity |
| Simulation timestep | $\Delta t$ | not frozen | s/min/h | Numerical setting | Time-step convergence and weather-data compatibility |
| Optimiser population/iterations | — | not implemented | count | Numerical setting | Optimiser convergence/repeatability |

## Governance rules

1. No `Required input` may silently receive a convenient default in production calculations.
2. `Provisional` and `Historical only` values cannot support validated Singapore performance claims.
3. `Engineering assumption` values define experiments and must be sensitivity-tested before design conclusions.
4. Numerical settings require convergence evidence.
5. When a value is sourced, add or verify the corresponding BibTeX entry and cite it near the value in the report.
6. When Rust introduces a numerical constant, this register or a more specific data/config record must identify its category and basis.
7. The register must be reviewed at each mandatory audit as the model expands.