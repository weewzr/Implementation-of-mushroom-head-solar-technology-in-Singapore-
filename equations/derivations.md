# Analytical Derivations

Every displayed equation below is followed immediately by definitions of every introduced symbol, units, and—where applicable—the origin of numerical constants. These are analytical foundations and benchmarks, not validated Singapore annual-yield results.

## 1. Paraboloidal-cap area

Define the founding axisymmetric cap as

$$
z(r)=h\left(1-\frac{r^2}{R^2}\right).
$$

where $z(r)$ is surface height above the rim plane (m), $r$ is radial coordinate (m), $h$ is apex height above the rim plane (m), and $R$ is footprint radius (m). The exponent 2 is a geometry-definition choice for the paraboloidal baseline, not an empirical constant.

For a differentiable surface of revolution, the differential area is

$$
\mathrm dA=2\pi r\sqrt{1+\left(\frac{\mathrm dz}{\mathrm dr}\right)^2}\,\mathrm dr.
$$

where $\mathrm dA$ is differential surface area (m$^2$), $r$ and $\mathrm dr$ are radial coordinate and differential radial increment (m), $\mathrm dz/\mathrm dr$ is dimensionless local slope, and $2\pi r$ is the circumference of the annulus (m). The factor 2 and $\pi$ arise exactly from circle geometry.

Differentiating the surface definition gives

$$
\frac{\mathrm dz}{\mathrm dr}=-\frac{2hr}{R^2}.
$$

where $\mathrm dz/\mathrm dr$ is dimensionless and $h$, $r$, $R$ retain the definitions above. The factor 2 arises exactly from differentiating $r^2$.

Substitution gives the total active surface area

$$
A_{\mathrm{PV}}=2\pi\int_0^R r\sqrt{1+\frac{4h^2r^2}{R^4}}\,\mathrm dr.
$$

where $A_{\mathrm{PV}}$ is active curved PV area (m$^2$). The factor 4 is the exact square of the derivative coefficient 2; it is not fitted.

Use the substitution

$$
u=1+\frac{4h^2r^2}{R^4},\qquad \mathrm du=\frac{8h^2r}{R^4}\,\mathrm dr.
$$

where $u$ is a dimensionless integration variable and $\mathrm du$ its differential. The factors 4 and 8 follow algebraically from the squared slope and differentiation.

Therefore

$$
r\,\mathrm dr=\frac{R^4}{8h^2}\,\mathrm du,
$$

where both sides have units of m$^2$. Substitution into the area integral yields

$$
A_{\mathrm{PV}}=\frac{\pi R^4}{4h^2}\int_1^{1+4h^2/R^2}u^{1/2}\,\mathrm du.
$$

where $A_{\mathrm{PV}}$ is in m$^2$ and the integral is dimensionless. The prefactor therefore has units m$^2$, satisfying dimensional consistency.

Since

$$
\int u^{1/2}\,\mathrm du=\frac{2}{3}u^{3/2},
$$

where $u$ is dimensionless and $2/3$ is the exact calculus coefficient, integration gives

$$
A_{\mathrm{PV}}=\frac{\pi R^4}{6h^2}\left[\left(1+\frac{4h^2}{R^2}\right)^{3/2}-1\right].
$$

where $A_{\mathrm{PV}}$ is in m$^2$ and $h,R$ are in m. The factor 6 follows exactly from the preceding algebra and integration.

Define the dimensionless curvature/aspect ratio

$$
k=\frac{h}{R}.
$$

where $k$ is dimensionless, while $h$ and $R$ are in m. Then

$$
\boxed{A_{\mathrm{PV}}=\frac{\pi R^2}{6k^2}\left[(1+4k^2)^{3/2}-1\right]}.
$$

where all symbols are defined above. For a circular footprint $A_{\mathrm{foot}}=\pi R^2$ (m$^2$), the packing ratio is

$$
\boxed{\Pi=\frac{A_{\mathrm{PV}}}{A_{\mathrm{foot}}}=\frac{(1+4k^2)^{3/2}-1}{6k^2}}.
$$

where $\Pi$ is dimensionless and $A_{\mathrm{foot}}$ is horizontal footprint area (m$^2$).

### Limiting and dimensional check

For $k\to0$, the binomial expansion gives

$$
(1+4k^2)^{3/2}=1+6k^2+O(k^4),
$$

where $O(k^4)$ denotes terms of fourth order and higher in dimensionless $k$. Hence $\Pi\to1$, recovering a flat disk. This is a required geometry sanity check.

### Physical interpretation

Increasing $k$ increases surface area available inside a fixed projected radius. It does not create irradiance or improve cell conversion efficiency. The additional area is useful only to the extent that its orientation, visibility, temperature and electrical productivity remain adequate.

### Engineering implication

The paraboloid must be judged by land-normalised annual energy and lifecycle value, not by surface area alone. Curvature should therefore be swept together with self-shading, sky-view, bifacial access, wind/structure and cost.

## 2. Ideal isotropic diffuse benchmark

For an unobstructed plane under an isotropic sky,

$$
\mathrm dP_d=\eta\,DHI\,\frac{1+\cos\beta}{2}\,\mathrm dA.
$$

where $\mathrm dP_d$ is incremental diffuse-derived electrical power (W), $\eta$ is PV conversion efficiency (dimensionless), $DHI$ is diffuse horizontal irradiance (W m$^{-2}$), $\beta$ is local surface tilt from horizontal (rad or degrees under a consistent trigonometric convention), and $\mathrm dA$ is active area (m$^2$). The factor $1/2$ is the exact isotropic-hemisphere view-factor coefficient, not a Singapore empirical parameter.

Integrating over surface $S$ gives

$$
P_d=\frac{\eta DHI}{2}\left(\int_S\mathrm dA+\int_S\cos\beta\,\mathrm dA\right).
$$

where $P_d$ is diffuse-derived electrical power (W) and $S$ denotes the PV surface. For a single-valued upward-facing graph surface, vertical projection gives

$$
\int_S\cos\beta\,\mathrm dA=A_{\mathrm{foot}}.
$$

where $A_{\mathrm{foot}}$ is horizontal projected footprint (m$^2$). Therefore

$$
\boxed{P_d=\frac{\eta DHI}{2}\left(A_{\mathrm{PV}}+A_{\mathrm{foot}}\right)}.
$$

where $A_{\mathrm{PV}}$ and $A_{\mathrm{foot}}$ are in m$^2$. Unit check: (W m$^{-2}$)(m$^2$)=W.

### Physical interpretation

This benchmark shows why extra upward-facing surface area can collect additional isotropic diffuse irradiance in an ideal convex/no-occlusion model. Real 3-D canopies violate the unobstructed-sky assumption as facets block sky patches and one another.

### Engineering implication

Use this equation as a code benchmark only. Final Singapore calculations require anisotropic diffuse radiance and explicit sky visibility; it must not be used to claim an annual percentage gain.

## 3. Land multiplication identity

For equal land footprint, define

$$
M_L=\frac{E_{3D}}{E_{\mathrm{base}}}.
$$

where $M_L$ is land multiplication (dimensionless), $E_{3D}$ is annual electrical energy from the candidate (kWh yr$^{-1}$), and $E_{\mathrm{base}}$ is annual electrical energy from the explicitly defined baseline over the same land footprint (kWh yr$^{-1}$). Define packing ratio

$$
\Pi=\frac{A_{\mathrm{PV}}}{A_{\mathrm{land}}},
$$

where $A_{\mathrm{PV}}$ is active PV area (m$^2$), $A_{\mathrm{land}}$ is constrained horizontal site area (m$^2$), and $\Pi$ is dimensionless. Multiplying and dividing the energy ratio by $\Pi$ gives

$$
M_L=\Pi\frac{E_{3D}}{\Pi E_{\mathrm{base}}}.
$$

Define

$$
\eta_{\mathrm{pack}}=\frac{E_{3D}}{\Pi E_{\mathrm{base}}},
$$

where $\eta_{\mathrm{pack}}$ is average packed-PV productivity relative to the baseline (dimensionless). Thus

$$
\boxed{M_L=\Pi\eta_{\mathrm{pack}}}.
$$

where all terms are dimensionless.

### Physical interpretation

$\Pi$ measures how much PV is fitted into scarce footprint; $\eta_{\mathrm{pack}}$ measures how productive that PV remains. The product prevents packing alone from being mistaken for efficiency.

### Engineering implication

A 3-D design is attractive for land use only when added packing outweighs orientation, occlusion, thermal, structural, maintenance and economic penalties.

## 4. Energy-density stationary condition

Let

$$
M_L(\Pi)=\Pi\eta_{\mathrm{pack}}(\Pi).
$$

where both quantities are dimensionless functions of packing ratio. Differentiating gives

$$
\frac{\mathrm dM_L}{\mathrm d\Pi}=\eta_{\mathrm{pack}}+\Pi\frac{\mathrm d\eta_{\mathrm{pack}}}{\mathrm d\Pi}.
$$

where the derivative is dimensionless per dimensionless. At an interior stationary point,

$$
\eta_{\mathrm{pack}}+\Pi\frac{\mathrm d\eta_{\mathrm{pack}}}{\mathrm d\Pi}=0.
$$

Dividing by $\eta_{\mathrm{pack}}$ and rewriting logarithmically yields

$$
\boxed{-\frac{\mathrm d\ln\eta_{\mathrm{pack}}}{\mathrm d\ln\Pi}=1}.
$$

where the logarithmic derivative is dimensionless. The numerical value 1 follows exactly from differentiating the product; it is not an empirical threshold.

### Physical interpretation

For a pure land-energy objective, an interior stationary point occurs when a 1% increase in packing causes a 1% decrease in average packed-PV productivity.

### Engineering implication

This is not yet the project optimum because lifecycle cost, structure, reliability and land opportunity value add objectives/constraints. It is a useful diagnostic for packing sweeps.

## 5. Mechanical implication of slow movement

A characteristic inertial-torque scaling is

$$
\tau_I\sim I\frac{\Delta\theta}{T^2}.
$$

where $\tau_I$ is characteristic inertial torque (N m), $I$ is mass moment of inertia (kg m$^2$), $\Delta\theta$ is angular displacement (rad, dimensionless in SI), and $T$ is movement duration (s). If the same displacement is executed in twice the time,

$$
\tau_I(2T)\sim\frac{1}{4}\tau_I(T).
$$

where $1/4$ follows exactly from the $T^{-2}$ scaling.

### Physical interpretation

Solar position evolves slowly, so intentional tracking can be quasi-static. Increasing movement duration strongly suppresses inertial torque.

### Engineering implication

Do not optimise angular momentum. Prioritise wind moment, gravitational balance, friction, locking/stow, actuator efficiency, reliability and wear. Quantitative wind design still requires sourced aerodynamic/structural inputs.