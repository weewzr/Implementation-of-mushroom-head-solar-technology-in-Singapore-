# Canonical nomenclature and mathematical style

This file is the single notation authority for the report. Report prose, LaTeX, figures and code-facing documentation should map to these symbols rather than inventing local aliases.

## Style policy
- Scalars: italic Latin/Greek.
- Vectors: bold lowercase, e.g. $\mathbf{s}$ and $\mathbf{n}$; no arrow notation.
- Matrices/tensors: bold uppercase.
- Descriptive subscripts: upright/roman; variable indices remain italic.
- Units: upright SI typography through `siunitx` in LaTeX.
- Angles: radians in governing equations and numerical implementation; degrees only when explicitly reported.
- Dot product: $\mathbf a\cdot\mathbf b$; vector cross product: $\mathbf a\times\mathbf b$.
- Differentials: upright, e.g. $\mathrm d t$, $\mathrm d A$.
- Time-series index: subscript $t$; integration/sums show $\Delta t$ explicitly.
- Every displayed equation referenced later must have a stable `\label{}` and be cited with `\eqref{}`.
- Irradiance [W m$^{-2}$] and irradiation/energy-per-area [Wh m$^{-2}$ or kWh m$^{-2}$] are distinct quantities and must never share an ambiguous symbol.
- DEVELOPMENT_NOT_SERIS is a result-status qualifier, not part of mathematical notation.

## Canonical symbols
| Symbol | Meaning | SI unit | Notes |
|---|---|---|---|
| $\mathbf{s}$ | unit vector from surface toward Sun | 1 | ENU frame |
| $\mathbf{n}$, $\mathbf{n}_i$ | outward surface/facet unit normal | 1 | bold-vector convention |
| $\theta_i$ | solar incidence angle on facet $i$ | rad | $\cos\theta_i=\mathbf n_i\cdot\mathbf s$ |
| $\alpha$ | solar elevation | rad | report degrees only when labelled |
| $\delta$ | solar declination | rad | |
| $H$ | solar hour angle | rad | |
| $\beta$ | surface tilt from horizontal | rad | |
| $G_{\rm GHI}$ | global horizontal irradiance | W m$^{-2}$ | weather input |
| $G_{\rm DHI}$ | diffuse horizontal irradiance | W m$^{-2}$ | weather input |
| $G_{\rm DNI}$ | direct normal irradiance | W m$^{-2}$ | weather input |
| $G_{\rm POA}$ | total plane-of-array irradiance | W m$^{-2}$ | interval-average electrical input |\n| $H_{\rm POA}$ | incident plane-of-array energy over a stated interval | Wh | annual comparison uses explicit interval/year |
| $G_{{\rm dir},i}$ | direct POA irradiance on facet $i$ | W m$^{-2}$ | |
| $G_{{\rm dif},i}$ | diffuse POA irradiance on facet $i$ | W m$^{-2}$ | |
| $G_{{\rm grd},i}$ | ground-reflected POA irradiance on facet $i$ | W m$^{-2}$ | |
| $V_i$ | direct-beam visibility factor | 1 | |
| $F_{{\rm sky},i}$ | sky-view factor | 1 | |
| $A_i$ | active area of facet $i$ | m$^2$ | |
| $A_{\rm PV}$ | total active PV area | m$^2$ | |
| $A_{\rm land}$ | projected horizontal land footprint | m$^2$ | |
| $A_{\rm proj}$ | projected area normal to specified direction | m$^2$ | |
| $\Pi$ | packing ratio $A_{\rm PV}/A_{\rm land}$ | 1 | |
| $\eta_{\rm pack}$ | PV-area irradiance productivity relative to flat reference | 1 | not electrical efficiency |
| $M_L$ | land-energy multiplier $\Pi\eta_{\rm pack}$ | 1 | irradiance-level metric |
| $R$ | cap/reference radius | m | |
| $h$ | cap height | m | |
| $k=h/R$ | dimensionless cap curvature | 1 | |
| $\lambda$ | deployable interpolation/deployment parameter | 1 | reserved for origami/deployment |
| $T_a$ | ambient air temperature | $^\circ$C | canonical T2M input |
| $T_m$ | module temperature | $^\circ$C | thermal-model output |
| $T_{\rm ref}$ | electrical reference temperature | $^\circ$C | |
| $T_{\rm NMOT}$ | nominal module operating temperature | $^\circ$C | manufacturer property |
| $\eta_{\rm ref}$ | module efficiency at reference condition | 1 | |
| $\eta_m$ | temperature-adjusted module efficiency | 1 | |
| $\gamma_P$ | Pmax temperature coefficient | K$^{-1}$ | |
| $P_{\rm DC,ideal}$ | ideal DC power before declared DC losses | W | |
| $P_{\rm DC}$ | delivered DC power after declared DC losses | W | |
| $P_{\rm AC}$ | generated AC power | W | |
| $P_{\rm aux}$ | auxiliary electrical demand | W | explicit scenario |
| $P_{\rm net}$ | net power $P_{\rm AC}-P_{\rm aux}$ | W | may be negative |
| $E_{\rm net}$ | integrated net electrical energy | Wh | $\sum_tP_{{\rm net},t}\Delta t$ |
| $\Delta t$ | timestep duration | h for electrical integration | explicit conversion required |
| $\rho$ | air density | kg m$^{-3}$ | structural/wind future layer |
| $C_D$ | aerodynamic drag coefficient | 1 | future layer |
| $v$ | wind speed | m s$^{-1}$ | reference height must be stated |
| $\tau$ | torque | N m | |
| $I$ | mass moment of inertia | kg m$^2$ | |
| $\omega$ | angular velocity | rad s$^{-1}$ | |

## Reserved future symbols
Economics and carbon symbols are not frozen until those models exist. Do not reuse existing solar/thermal/electrical symbols. Candidate future entries must be added here before use in the main report.

## Coordinate convention
Right-handed East-North-Up (ENU); solar/facet azimuth clockwise from geographic north; outward/front-side facet normals; radians internally in Rust. See `docs/coordinate_conventions.md`.

## Human-readable equation rule
Each major mathematical block must follow: physical question -> diagram/physical idea -> variable definitions -> governing equation -> interpretation -> limiting/verification case.


## Audit-66 canonical mathematical style

This policy is normative for the report and supersedes legacy spellings above where they conflict.

- Scalars: italic Latin/Greek symbols.
- Vectors: bold lowercase only, e.g. $\mathbf{s}$ and $\mathbf{n}$; do not mix arrow notation.
- Matrices/tensors: bold uppercase.
- Descriptive subscripts: upright roman, e.g. $G_{\rm POA}$, $A_{\rm PV}$, $P_{\rm AC}$; variable indices remain italic.
- SI units: upright and typeset through `siunitx` in LaTeX. Irradiance is W m$^{-2}$; irradiation/energy-per-area carries an explicit interval and Wh or kWh m$^{-2}$.
- Dot product: $\mathbf n\cdot\mathbf s$; cross product: $\times$ only for vector cross products.
- Differentials: upright, e.g. $\mathrm d t$, $\mathrm d A$.
- Analytical angles use radians unless an equation/value explicitly marks degrees.
- Canonical solar/incidence symbols: $\alpha$ solar elevation, $\delta$ declination, $H$ hour angle, $\theta_i$ incidence angle, $\mathbf s$ Sun direction, $\mathbf n$ outward normal.
- Canonical irradiance symbols: $G_{\rm DNI}$, $G_{\rm DHI}$, $G_{\rm GHI}$, $G_{\rm POA}$; component POA terms must use explicit direct/diffuse/ground labels.
- Canonical resource symbols: $A_{\rm PV}$, $A_{\rm land}$, $\Pi=A_{\rm PV}/A_{\rm land}$, $\eta_{\rm pack}$, $M_L=\Pi\eta_{\rm pack}$.
- Canonical thermal symbols: $T_a$, $T_m$, $T_{\rm ref}$, $T_{\rm NMOT}$.
- Canonical electrical symbols: $\eta_{\rm ref}$, $\eta_m$, $\gamma_P$, $P_{\rm DC}$, $P_{\rm AC}$, $P_{\rm aux}$, $P_{\rm net}$, $E_{\rm net}$.
- Origami/deployment: $\lambda$ is reserved for deployment interpolation; any later wavelength/economic use must choose another symbol.
- Every displayed equation referenced later must receive a stable LaTeX `\\label{eq:...}` and be cited with `\\eqref{eq:...}`.
- Every symbol used in a displayed equation must have one canonical nomenclature entry before release.
