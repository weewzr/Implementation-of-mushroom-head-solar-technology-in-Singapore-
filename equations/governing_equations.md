# Governing Equations

This file is the compact mathematical reference for the project. The technical report contains derivations, assumptions and interpretation.

## Solar position
$$\delta(n)=23.45^\circ\sin\left[\frac{360^\circ}{365}(284+n)\right].$$
$$\sin\alpha=\sin\phi\sin\delta+\cos\phi\cos\delta\cos H.$$

## Direct irradiance on facet $i$
$$G_{b,i}=DNI\,V_i\,[\mathbf n_i\cdot\mathbf s]_+,$$
where $[x]_+=\max(0,x)$ and $V_i$ is direct-beam visibility.

## Isotropic diffuse first approximation
$$G_{d,i}=DHI\frac{1+\cos\beta_i}{2}.$$
This is an approximation only; the final model requires anisotropic diffuse radiance and obstruction-aware sky-view factors.

## Bifacial effective irradiance
$$G_{eff,i}=G_{front,i}+bG_{rear,i}.$$

## Temperature correction
$$\eta(T_c)=\eta_{ref}[1+\gamma(T_c-T_{ref})].$$

## Paraboloidal mushroom
$$z(r)=h\left(1-\frac{r^2}{R^2}\right),\qquad k=\frac hR,$$
$$\frac{dz}{dr}=-\frac{2hr}{R^2},$$
$$A_{PV}=2\pi\int_0^Rr\sqrt{1+\frac{4h^2r^2}{R^4}}\,dr,$$
$$\boxed{A_{PV}=\frac{\pi R^2}{6k^2}\left[(1+4k^2)^{3/2}-1\right]},$$
$$\boxed{\Pi=\frac{A_{PV}}{A_{foot}}=\frac{(1+4k^2)^{3/2}-1}{6k^2}}.$$

## Land multiplication
$$\Pi=\frac{A_{PV}}{A_{land}},\qquad \eta_{pack}=\frac{E_{3D}}{\Pi E_{base}},$$
$$\boxed{M_L=\Pi\eta_{pack}}.$$
A pure energy-density optimum satisfies
$$\eta_{pack}+\Pi\frac{d\eta_{pack}}{d\Pi}=0,$$
or
$$\boxed{-\frac{d\ln\eta_{pack}}{d\ln\Pi}=1}.$$

## Rotation and mechanics
$$L=I\omega,\qquad E_k=\frac12I\omega^2,$$
$$\boxed{\tau=I\ddot\theta+\tau_f+\tau_g+\tau_w}.$$

## Wind
$$F_D=\frac12\rho C_DA_{proj}v^2,$$
$$\tau_w\simeq F_Dr_{CP}.$$

## Free-form fixed-topology optimisation
For facet $i$,
$$\mathbf x_i=(x_i,y_i,z_i,\theta_i,\phi_i,A_i),$$
and $\mathbf X=[\mathbf x_1,\ldots,\mathbf x_N]$. The first optimisation is
$$\max_{\mathbf X}E_{annual}(\mathbf X)$$
subject to footprint, total PV area, height, non-overlap, structural, wind, thermal and manufacturability constraints.

## Tracking value
$$\Delta E_{track}=E^*_{moving}-E^*_{fixed}.$$
Movement is justified only when the lifecycle value of this increment exceeds actuator energy, capital, wear and maintenance penalties.
