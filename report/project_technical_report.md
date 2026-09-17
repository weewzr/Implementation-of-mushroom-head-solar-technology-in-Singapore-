# Topology and Geometry Optimisation of Three-Dimensional Photovoltaic Canopies for Land-Constrained Singapore

**Status:** Research reconstruction and first-pass model  
**Date:** 17 September 2026

## Abstract
Singapore's solar resource is strong but deployment is constrained by scarce land and competing urban uses. This project investigates whether three-dimensional photovoltaic (PV) geometries—initially motivated by a rotating "mushroom-head" panel—can increase annual electricity generation per constrained horizontal footprint. The central insight is that curvature does not create solar energy: its potential value is spatial packing. A three-dimensional canopy can place $A_{PV}>A_{land}$ while attempting to retain high irradiation quality, bifacial access, ventilation and usable space beneath. The core dimensionless relationship is $M_L=\Pi\eta_{pack}$, where $\Pi=A_{PV}/A_{land}$ and $\eta_{pack}$ is average productivity of packed PV relative to a baseline. First-pass analytical work derives the area of a paraboloidal cap and exposes the fundamental trade-off: land-normalized collection can increase with curvature while electricity per square metre of PV falls. These early calculations are intentionally idealised and are not bankable yield predictions.

## 1. Problem and motivation
EMA reports average annual solar irradiance of about 1,580 kWh m⁻² yr⁻¹. Singapore reached 2 GWp installed solar capacity in 2025 and raised its 2030 target to 3 GWp. SERIS identifies land scarcity as a major PV constraint and investigates multiple-use solar configurations.

The engineering question is:

> **For a constrained horizontal footprint in Singapore, what three-dimensional PV geometry and movement strategy maximises useful annual energy and lifecycle value after optical, thermal, mechanical, structural and economic penalties are included?**

## 2. Solar incidence
For surface normal $\mathbf n$ and solar unit vector $\mathbf s(t)$,

$$dP_{dir}=\eta(T)DNI(t)V(t)[\mathbf n\cdot\mathbf s(t)]_+dA.$$

The first diffuse approximation is

$$dP_{diff}=\eta(T)DHI(t)\frac{1+\cos\beta}{2}dA.$$

The final model must replace this isotropic approximation with time-correlated Singapore irradiance data and an anisotropic diffuse model.

## 3. Paraboloidal mushroom geometry
Define

$$z(r)=h\left(1-\frac{r^2}{R^2}\right),\qquad k=\frac hR.$$

Since

$$\frac{dz}{dr}=-\frac{2hr}{R^2},$$

the axisymmetric surface area is

$$A_{PV}=2\pi\int_0^R r\sqrt{1+\frac{4h^2r^2}{R^4}}dr.$$

Direct integration gives

$$A_{PV}=\frac{\pi R^2}{6k^2}\left[(1+4k^2)^{3/2}-1\right],$$

and therefore

$$\boxed{\Pi_A=\frac{A_{PV}}{A_{foot}}=\frac{(1+4k^2)^{3/2}-1}{6k^2}}.$$

The limiting case $k\to0$ returns $\Pi_A\to1$, satisfying the flat-disk sanity check.

## 4. Diffuse-light first approximation
Under an isotropic sky,

$$P_{diff}=\eta DHI\int_S\frac{1+\cos\beta}{2}dA.$$

For a convex single-valued cap,

$$\int_S\cos\beta\,dA=A_{foot},$$

so

$$\boxed{P_{diff}=\frac{\eta DHI}{2}(A_{PV}+A_{foot}).}$$

This explains why a curved surface can collect more diffuse radiation per ground footprint in an ideal no-occlusion model. It is not a real annual-yield prediction because anisotropy, mutual visibility, electrical mismatch and time correlation are absent.

## 5. First numerical experiment
A deliberately simplified calculation used $G_{annual}=1580$ kWh m⁻² yr⁻¹, a provisional diffuse share of 57%, and $\eta=23\%$. The calculation is retained because it exposes the trade-off, not because it predicts plant yield.

| $h/R$ | $A_{PV}/A_{foot}$ | illustrative electricity (kWh m⁻²-land yr⁻¹) | gain vs flat | electricity per m² PV |
|---:|---:|---:|---:|---:|
|0.00|1.000|363.4|0.0%|363.4|
|0.50|1.219|386.1|6.2%|316.7|
|1.00|1.697|435.6|19.9%|256.7|
|1.50|2.268|494.8|36.1%|218.1|
|2.00|2.879|558.0|53.5%|193.8|
|3.00|4.149|689.6|89.8%|166.2|

The apparent land-efficiency gain is purchased with rapidly declining PV-material productivity. The unbounded rise is a warning that the model lacks occlusion, height, cost and structural constraints.

## 6. Central packing relation
Define

$$\Pi=\frac{A_{PV}}{A_{land}},\qquad \eta_{pack}=\frac{E_{3D}}{\Pi E_{flat}}.$$

Then

$$\boxed{M_L=\Pi\eta_{pack}}.$$

This is the mathematical heart of the project. A useful topology makes irradiation quality decay slowly as packing increases. The energy-density-only optimum satisfies

$$\eta_{pack}+\Pi\frac{d\eta_{pack}}{d\Pi}=0,$$

or equivalently

$$-\frac{d\ln\eta_{pack}}{d\ln\Pi}=1.$$

Economics will generally move the optimum to a lower packing ratio.

## 7. Sphere, mushroom and faceted alternatives
A sphere has surface area $4\pi R^2$ but projected area $\pi R^2$ toward a direct-beam direction. It therefore uses substantial PV material with many poorly oriented elements. It remains a baseline rather than the leading hypothesis. Candidate geometries include horizontal fixed PV, optimised fixed tilt, vertical bifacial, accordion folds, cones, paraboloidal mushrooms, hemispheres, faceted flowers and free-form topology-optimised surfaces.

## 8. Mechanics and the momentum question
Angular momentum and kinetic energy are

$$L=I\omega,\qquad E_k=\frac12I\omega^2.$$

Required torque is

$$\tau=I\ddot\theta+\tau_f+\tau_w+\tau_g.$$

Solar motion is slow, so deliberately storing rotational momentum is not advantageous. Priorities are low moment of inertia, counterbalancing, low friction, pivot placement near the centre of mass and aerodynamic centre, slow quasi-static movement, locking and storm stow.

Wind force scales as

$$F_D=\frac12\rho C_DA_{proj}v^2,$$

with approximate torque

$$\tau_w\simeq F_Dr_{CP}.$$

## 9. Tracking as optimal control
Movement should be introduced only after the best fixed topology is found:

$$\max_{\theta(t)}\int_T[P_{PV}(\theta,t)-P_{motor}(\theta,\dot\theta,\ddot\theta)]dt-C_{wear}.$$

A discrete controller may be superior to continuous tracking. A move is justified only when incremental beam gain exceeds lost diffuse collection, actuator energy and wear cost.

## 10. Bifacial and thermal extensions
For bifaciality $b$,

$$G_{eff,i}=G_{front,i}+bG_{rear,i}.$$

Temperature correction is approximated by

$$\eta(T_c)=\eta_{ref}[1+\gamma(T_c-T_{ref})].$$

Sparse elevated facets may improve rear irradiance and ventilation, but these benefits require ray-tracing and thermal validation.

## 11. Free-form optimisation
For $N$ facets, let

$$\mathbf x_i=(x_i,y_i,z_i,\theta_i,\phi_i,A_i).$$

A representative problem is

$$\max_{\mathbf X}E_{annual}(\mathbf X)$$

subject to

$$A_{foot}\le1\;\mathrm{m^2},\qquad \sum_iA_i\le2\;\mathrm{m^2},\qquad 0\le z_i\le2\;\mathrm m,$$

plus non-overlap, structural stress, wind torque, temperature and manufacturability constraints. The first optimisation should be static; discrete movement is a later extension.

## 12. Numerical method roadmap
Each timestep will compute solar position, irradiance components, facet incidence, direct-beam visibility, anisotropic sky irradiance, rear irradiance, temperature and electrical output. Ray tracing will determine self-shadowing and sky-view factors. Evolutionary optimisation followed by local refinement is appropriate because shadow boundaries make the objective non-convex and partly non-smooth.

## 13. Singapore data and validation
EMA reports approximately 1,580 kWh m⁻² yr⁻¹ average annual solar irradiance. Singapore reached 2 GWp in 2025 and targets 3 GWp by 2030. SERIS operates a 25-station irradiance network, with 10 stations additionally measuring diffuse irradiance and meteorological variables. The final model should use measured or validated time series rather than annual-average decomposition. Redistribution rights must be checked before raw third-party data are committed.

## 14. Limitations
Current percentages are exploratory. Missing effects include measured time-correlated DNI/DHI, anisotropic diffuse sky, 3-D self-occlusion, array shading, bifacial rear view, detailed temperature, electrical mismatch, inverter clipping, structural mass, wind CFD, lifecycle cost and degradation. The 57% diffuse share used in exploratory work requires source verification before promotion to a formal project input. No current percentage gain should be presented as expected real-world performance.

## 15. Engineering interpretation
The work has narrowed the hypothesis. A sphere is unlikely to be material-efficient. Continuous tracking is not automatically justified. A smooth rotating mushroom is a useful analytical starting geometry, but the stronger candidate is a sparse faceted bifacial canopy that uses three-dimensional space to increase $A_{PV}/A_{land}$ while preserving sky view and allowing continued use of the land below. The value proposition is spatial packing and multifunctional land use, not intrinsic PV efficiency from curvature.

## 16. Next steps
The immediate milestone is a packing-ratio sweep $\Pi=1$ to $4$ across common candidate geometries under identical resource constraints, followed by a 20-facet free-form optimisation. The decisive graph is $M_L(\Pi)$ together with cost and structural penalties. Only after fixed topology is validated should discrete tracking be added.

## References
[1] Energy Market Authority, “Solar,” 2026.  
[2] Energy Market Authority, “Singapore to Accelerate Solar Deployment to Meet 3 GWp Solar Target by 2030,” 2 Mar. 2026.  
[3] Solar Energy Research Institute of Singapore, “Real-Time Monitoring System of Meteorological Parameters,” accessed 17 Sep. 2026.  
[4] Solar Energy Research Institute of Singapore, *Annual Report 2025*, National University of Singapore, 2025.

## Reproducibility
`src/models/solar_geometry.py` implements the analytical geometry. `src/analysis/first_pass.py` regenerates the exploratory table. `src/visualisation/make_plots.py` generates the first plots. Model-generated data must not be confused with measurements.
