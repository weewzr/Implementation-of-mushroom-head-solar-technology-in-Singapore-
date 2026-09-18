# Nomenclature

| Symbol | Meaning | Typical unit |
|---|---|---|
| $A_{PV}$ | active photovoltaic surface area | m² |
| $A_{land}$ | constrained horizontal land/site footprint | m² |
| $A_{foot}$ | projected horizontal footprint of one geometry | m² |
| $A_{proj}$ | projected area normal to a specified direction | m² |
| $R$ | mushroom/cap radius | m |
| $h$ | cap height | m |
| $k=h/R$ | dimensionless cap curvature | – |
| $\Pi$ | PV packing ratio, $A_{PV}/A_{land}$ | – |
| $\eta_{pack}$ | average packed-PV productivity relative to baseline | – |
| $M_L$ | land multiplication factor | – |
| $GHI$ | global horizontal irradiance | W m⁻² |
| $DHI$ | diffuse horizontal irradiance | W m⁻² |
| $DNI$ | direct normal irradiance | W m⁻² |
| $\mathbf{s}$ | unit vector toward sun | – |
| $\mathbf{n}$ | outward unit surface normal | – |
| $\alpha$ | solar elevation angle | rad or deg |
| $\delta$ | solar declination | rad or deg |
| $H$ | solar hour angle | rad or deg |
| $\beta$ | surface tilt from horizontal | rad or deg |
| $V_i$ | beam visibility factor for facet $i$ | – |
| $b$ | PV bifaciality coefficient | – |
| $T_c$ | PV cell temperature | °C |
| $\eta$ | PV electrical conversion efficiency | – |
| $\gamma$ | PV temperature coefficient | K⁻¹ |
| $I$ | mass moment of inertia | kg m² |
| $\omega$ | angular velocity | rad s⁻¹ |
| $\theta$ | tracker angle | rad or deg |
| $\tau$ | torque | N m |
| $\rho$ | air density | kg m⁻³ |
| $C_D$ | aerodynamic drag coefficient | – |
| $v$ | wind speed | m s⁻¹ |
| $r_{CP}$ | moment arm from pivot to centre of pressure | m |
| $E_{annual}$ | annual electrical energy | kWh yr⁻¹ |

## Notation conventions

- Subscripts `PV`, `land`, `foot`, `proj`, `front`, `rear`, and `base` identify physical role rather than mathematical operation.
- Bold lowercase symbols such as $\mathbf n$ and $\mathbf s$ denote dimensionless three-component vectors.
- Scalar angles are represented in radians inside the canonical Rust numerical implementation; degrees may be used in explanatory equations only when explicitly labelled.
- Energy quantities must state their evaluation interval; annual energy uses kWh yr$^{-1}$.
- Irradiance uses W m$^{-2}$; irradiation/energy-per-area uses kWh m$^{-2}$ over a stated interval. These terms must not be interchanged.
- A superscript $*$ denotes an optimised value when used in optimisation sections, not multiplication.

## Coordinate-system status

The final east–north–up coordinate convention, solar azimuth sign convention, facet-normal orientation and tracker-angle sign convention must be stated identically in the Markdown report, LaTeX report and Rust implementation before higher-fidelity ray tracing begins. Until that reconciliation is completed, solar-vector calculations remain preliminary.
