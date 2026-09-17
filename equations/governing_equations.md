# Governing Equations — With Variables, Units and Constant Provenance

This is the compact mathematical reference. Every important equation is immediately followed by definitions and units. Detailed derivations are in the technical report and derivation notes.

## 1. Preliminary solar position
### Cooper declination approximation
$$
\delta(n)=23.45^\circ\sin\!\left[\frac{360^\circ}{365}(284+n)\right].
$$

where:
- $\delta$ = solar declination ($^\circ$);
- $n$ = ordinal day of a non-leap year (dimensionless day index);
- $23.45^\circ$ = approximate declination amplitude in the Cooper (1969) engineering correlation ($^\circ$);
- $360^\circ$ = one complete angular cycle, exact by definition ($^\circ$);
- $365$ = non-leap-year period assumed by this approximation (days);
- $284$ = Cooper phase-offset constant (dimensionless day index).

**Status:** astronomical approximation. Do not treat these coefficients as project-fitted constants or exact astronomy. Production calculations should use a traceable higher-accuracy solar-position method.

### Solar hour angle
$$
H=15^\circ\,\mathrm{h}^{-1}\left(t_{\mathrm{solar}}-12\,\mathrm h\right).
$$

where:
- $H$ = solar hour angle ($^\circ$);
- $t_{\mathrm{solar}}$ = local apparent solar time (h);
- $15^\circ\,\mathrm{h}^{-1}=360^\circ/(24\,\mathrm h)$ = mean hour-angle rate;
- $12\,\mathrm h$ = apparent solar noon, where $H=0^\circ$.

**Important:** Singapore civil clock time is not local apparent solar time. Longitude and equation-of-time corrections are required.

### Solar elevation
$$
\sin\alpha=\sin\phi\sin\delta+\cos\phi\cos\delta\cos H.
$$

where:
- $\alpha$ = solar elevation above local horizon ($^\circ$ or rad);
- $\phi$ = geographic latitude ($^\circ$ or rad);
- $\delta$ = solar declination (same angular convention);
- $H$ = solar hour angle (same angular convention).

All trigonometric arguments must use one consistent angular unit; NumPy implementations use radians.

## 2. Direct irradiance on facet $i$
$$
G_{b,i}=DNI\,V_i\,[\mathbf n_i\cdot\mathbf s]_+.
$$

where:
- $G_{b,i}$ = direct plane-of-array irradiance on facet $i$ (W m$^{-2}$);
- $DNI$ = direct normal irradiance (W m$^{-2}$);
- $V_i$ = direct-beam visibility factor (dimensionless; presently binary 0 or 1);
- $\mathbf n_i$ = outward unit normal of facet $i$ (dimensionless vector);
- $\mathbf s$ = unit vector from facet toward Sun (dimensionless vector);
- $[x]_+=\max(0,x)$ = positive-part operator, excluding backside direct incidence for a monofacial front face.

## 3. Isotropic diffuse first approximation
$$
G_{d,i}=DHI\frac{1+\cos\beta_i}{2}.
$$

where:
- $G_{d,i}$ = diffuse sky irradiance incident on facet $i$ (W m$^{-2}$);
- $DHI$ = diffuse horizontal irradiance (W m$^{-2}$);
- $\beta_i$ = facet tilt from horizontal ($^\circ$ or rad);
- $(1+\cos\beta_i)/2$ = unobstructed isotropic-sky view factor (dimensionless).

The factor $1/2$ follows from ideal hemispherical geometry, not empirical fitting. Final work requires anisotropic diffuse radiance plus obstruction-aware sky visibility.

## 4. Bifacial effective irradiance
$$
G_{\mathrm{eff},i}=G_{\mathrm{front},i}+bG_{\mathrm{rear},i}.
$$

where:
- $G_{\mathrm{eff},i}$ = effective irradiance used by the simplified bifacial electrical model (W m$^{-2}$);
- $G_{\mathrm{front},i}$ = front-side irradiance (W m$^{-2}$);
- $G_{\mathrm{rear},i}$ = rear-side irradiance (W m$^{-2}$);
- $b$ = bifaciality factor (dimensionless; must be sourced from the selected module/model).

## 5. Temperature correction
$$
\eta(T_c)=\eta_{\mathrm{ref}}\left[1+\gamma(T_c-T_{\mathrm{ref}})\right].
$$

where:
- $\eta(T_c)$ = PV conversion efficiency at cell temperature $T_c$ (dimensionless);
- $\eta_{\mathrm{ref}}$ = reference efficiency (dimensionless);
- $T_c$ = cell temperature ($^\circ$C or K);
- $T_{\mathrm{ref}}$ = reference cell temperature in same scale;
- $\gamma$ = relative efficiency temperature coefficient (K$^{-1}$ or $^\circ$C$^{-1}$).

No generic numerical $\gamma$ should be used in validated work; source it from the chosen PV technology/datasheet.

## 6. Paraboloidal mushroom
$$
z(r)=h\left(1-\frac{r^2}{R^2}\right),\qquad k=\frac{h}{R}.
$$

where:
- $z(r)$ = surface height above rim plane (m);
- $r$ = radial coordinate (m);
- $h$ = centre/apex height (m);
- $R$ = footprint radius (m);
- $k$ = curvature/aspect ratio (dimensionless).

$$
\frac{\mathrm dz}{\mathrm dr}=-\frac{2hr}{R^2}.
$$

where $\mathrm dz/\mathrm dr$ is local surface slope (dimensionless). The factor 2 is exact from differentiating $r^2$.

$$
A_{\mathrm{PV}}=2\pi\int_0^Rr\sqrt{1+\frac{4h^2r^2}{R^4}}\,\mathrm dr.
$$

where:
- $A_{\mathrm{PV}}$ = curved PV area (m$^2$);
- $2\pi r\,\mathrm dr$ = annular area contribution from axisymmetry (m$^2$ before slope correction);
- $\pi$ = mathematical circle constant (dimensionless);
- the square-root factor accounts for surface slope (dimensionless).

$$
\boxed{A_{\mathrm{PV}}=\frac{\pi R^2}{6k^2}\left[(1+4k^2)^{3/2}-1\right]}.
$$

where variables are defined above. The factors 4 and 6 arise from the exact derivative/integration and are not fitted constants.

For $A_{\mathrm{foot}}=\pi R^2$,

$$
\boxed{\Pi=\frac{A_{\mathrm{PV}}}{A_{\mathrm{foot}}}=\frac{(1+4k^2)^{3/2}-1}{6k^2}}.
$$

where:
- $\Pi$ = PV packing ratio (dimensionless);
- $A_{\mathrm{foot}}$ = horizontal footprint area (m$^2$).

Sanity check: $k\rightarrow0$ implies $\Pi\rightarrow1$.

## 7. Land multiplication
$$
\Pi=\frac{A_{\mathrm{PV}}}{A_{\mathrm{land}}},\qquad
\eta_{\mathrm{pack}}=\frac{E_{3D}}{\Pi E_{\mathrm{base}}}.
$$

where:
- $A_{\mathrm{land}}$ = constrained horizontal land/site area (m$^2$);
- $E_{3D}$ = annual energy of 3-D design (kWh yr$^{-1}$);
- $E_{\mathrm{base}}$ = annual energy of defined baseline (kWh yr$^{-1}$);
- $\eta_{\mathrm{pack}}$ = average productivity of packed PV relative to baseline (dimensionless).

$$
\boxed{M_L=\Pi\eta_{\mathrm{pack}}}.
$$

where $M_L$ = land multiplication factor (dimensionless).

At a differentiable energy-density-only stationary point,

$$
\eta_{\mathrm{pack}}+\Pi\frac{\mathrm d\eta_{\mathrm{pack}}}{\mathrm d\Pi}=0.
$$

where $\mathrm d\eta_{\mathrm{pack}}/\mathrm d\Pi$ is the packing-efficiency sensitivity (dimensionless).

Equivalently,

$$
\boxed{-\frac{\mathrm d\ln\eta_{\mathrm{pack}}}{\mathrm d\ln\Pi}=1}.
$$

The numerical value 1 follows exactly from the stationary-point derivative of $M_L=\Pi\eta_{\mathrm{pack}}$.

## 8. Rotation and mechanics
$$
L=I\omega,\qquad E_k=\frac12I\omega^2.
$$

where:
- $L$ = angular momentum (kg m$^2$ s$^{-1}$);
- $I$ = mass moment of inertia (kg m$^2$);
- $\omega$ = angular velocity (rad s$^{-1}$);
- $E_k$ = rotational kinetic energy (J);
- $1/2$ = exact rigid-body kinetic-energy coefficient.

$$
\boxed{\tau=I\ddot\theta+\tau_f+\tau_g+\tau_w}.
$$

where:
- $\tau$ = required actuator torque (N m);
- $\theta$ = rotation angle (rad or $^\circ$ under stated convention);
- $\ddot\theta$ = angular acceleration (rad s$^{-2}$);
- $\tau_f$, $\tau_g$, $\tau_w$ = frictional, gravitational and wind torques (N m).

## 9. Wind
$$
F_D=\frac12\rho C_DA_{\mathrm{proj}}v^2.
$$

where:
- $F_D$ = drag force (N);
- $\rho$ = air density (kg m$^{-3}$);
- $C_D$ = drag coefficient (dimensionless);
- $A_{\mathrm{proj}}$ = projected area normal to flow (m$^2$);
- $v$ = relative wind speed (m s$^{-1}$);
- $1/2$ = standard dynamic-pressure coefficient in $q=\tfrac12\rho v^2$.

$$
\tau_w\simeq F_Dr_{\mathrm{CP}}.
$$

where:
- $\tau_w$ = approximate wind torque (N m);
- $r_{\mathrm{CP}}$ = perpendicular moment arm to representative centre of pressure (m).

## 10. Free-form fixed-topology optimisation
For facet $i$,

$$
\mathbf x_i=(x_i,y_i,z_i,\theta_i,\phi_i,A_i).
$$

where $x_i,y_i,z_i$ are position coordinates (m), $\theta_i,\phi_i$ are orientation angles (rad or $^\circ$), and $A_i$ is active facet area (m$^2$).

For $N$ facets,

$$
\mathbf X=[\mathbf x_1,\ldots,\mathbf x_N].
$$

where $\mathbf X$ is the complete design vector and $N$ is number of facets (dimensionless integer).

The first optimisation is

$$
\max_{\mathbf X}E_{\mathrm{annual}}(\mathbf X).
$$

where $E_{\mathrm{annual}}$ is annual electrical energy (kWh yr$^{-1}$).

Footprint, PV area, height, overlap, structure, wind, thermal and manufacturability constraints must be stated separately with units and with numerical limits identified as sourced requirements or design assumptions.

## 11. Tracking value
$$
\Delta E_{\mathrm{track}}=E^*_{\mathrm{moving}}-E^*_{\mathrm{fixed}}.
$$

where:
- $\Delta E_{\mathrm{track}}$ = incremental annual energy attributable to permitted movement (kWh yr$^{-1}$);
- $E^*_{\mathrm{moving}}$ = optimised annual energy with movement (kWh yr$^{-1}$);
- $E^*_{\mathrm{fixed}}$ = optimised annual energy of fixed topology (kWh yr$^{-1}$);
- superscript $*$ denotes an optimised value, not multiplication.

Movement is justified only after lifecycle actuator energy, capital, wear, maintenance and reliability are included.
