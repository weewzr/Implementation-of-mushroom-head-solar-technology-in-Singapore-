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
| $A_i$ | active area of facet $i$ | m² |
| $N$ | number of mesh/facet elements | – |
| $N_p$ | number of petals | – |
| $\mathbf v_{ij}$ | ENU position vector of vertex $j$ of facet $i$ | m |
| $\mathbf e_{i1},\mathbf e_{i2}$ | edge vectors of triangular facet $i$ | m |
| $\mathbf n_i$ | outward unit normal of facet $i$ | – |
| $\gamma_p$ | facet/petal azimuth, clockwise from north | rad or deg |
| $\gamma_0$ | reference petal azimuth | rad or deg |
| $H_{max}$ | maximum permitted candidate height | m |
| $A_{PV,max}$ | active-PV-area resource limit | m² |
| $A_{land,max}$ | land/footprint resource limit | m² |
| $E_{net,annual}$ | net annual electrical-energy objective | kWh yr⁻¹ |

## Notation conventions

- Subscripts `PV`, `land`, `foot`, `proj`, `front`, `rear`, and `base` identify physical role rather than mathematical operation.
- Bold lowercase symbols such as $\mathbf n$ and $\mathbf s$ denote dimensionless three-component vectors.
- Scalar angles are represented in radians inside the canonical Rust numerical implementation; degrees may be used in explanatory equations only when explicitly labelled.
- Energy quantities must state their evaluation interval; annual energy uses kWh yr$^{-1}$.
- Irradiance uses W m$^{-2}$; irradiation/energy-per-area uses kWh m$^{-2}$ over a stated interval. These terms must not be interchanged.
- A superscript $*$ denotes an optimised value when used in optimisation sections, not multiplication.

## Coordinate-system status

The canonical convention is now defined in `docs/coordinate_conventions.md`: right-handed East–North–Up (ENU), solar/facet azimuth clockwise from geographic north, radians internally in Rust, and outward/front-side facet normals. Tracker rotation uses the right-hand rule about an explicitly stated ENU axis. The Markdown report, LaTeX report and Rust implementation must remain synchronized with that document. Solar-position calculations remain preliminary until NREL SPA benchmarking is completed.
