# Irradiance and visibility layer

## Why this layer is necessary
Earlier exploratory calculations used annual-average irradiance splits to expose geometry trends. That is insufficient for a defensible yield comparison because cloud state, DNI, DHI and solar position vary together. The computational architecture now accepts irradiance at each timestep and keeps the weather input independent of geometry.

## Irradiance identity
For a horizontal surface,

$$GHI=DNI\cos\theta_z+DHI.$$

When DNI is not supplied,

$$DNI=\frac{GHI-DHI}{\cos\theta_z}$$

may be reconstructed away from the horizon. The implementation guards small $\cos\theta_z$ because the inversion becomes numerically unstable.

## First plane-of-array model
For facet normal $\mathbf n$ and solar vector $\mathbf s$,

$$G_b=DNI[\mathbf n\cdot\mathbf s]_+V_b.$$

The current diffuse approximation is

$$G_d=DHI\frac{1+\cos\beta}{2},$$

and ground-reflected irradiance is

$$G_g=\rho_gGHI\frac{1-\cos\beta}{2}.$$

Thus

$$G_{POA}=G_b+G_d+G_g.$$

This is deliberately an intermediate model. An anisotropic sky model and explicit sky-patch visibility are later milestones.

## Direct-beam self-shadowing
Each PV surface will be triangulated. For each facet centroid, a ray is cast toward the sun. If it intersects another triangle before leaving the structure, direct visibility is zero; otherwise it is one. `src/visibility.rs` is the canonical Rust Möller–Trumbore ray/triangle and sky-visibility implementation. The earlier Python `src/models/facets.py` path is retained only as legacy exploratory code and is not canonical.

The geometric direct-beam collecting factor is

$$A_{beam}(t)=\sum_i A_iV_i(t)[\mathbf n_i\cdot\mathbf s(t)]_+.$$

This quantity has units of projected area and can be multiplied by DNI and electrical efficiency.

## Bifacial extension
The current irradiance module can evaluate the same first-order model on $\mathbf n$ and $-\mathbf n$:

$$G_{eff}=G_{front}+bG_{rear}.$$

This is not yet a final bifacial model because rear sky/ground visibility and mutual obstruction are not traced separately.

## Validation tests
The tests verify:

- DNI reconstruction and horizon guarding;
- overhead incidence on a horizontal facet;
- the $1/2$ isotropic sky factor for a vertical plane;
- horizontal POA identity under a consistent synthetic irradiance state;
- a simple two-triangle shadowing case.

## Next implementation
1. Mesh generators for paraboloid, cone and accordion with equal footprint and equal $A_{PV}$.
2. Time integration of direct visibility over the annual solar path.
3. Hemispherical sky-patch sampling for diffuse visibility.
4. Replace synthetic weather with a validated Singapore time series.
5. Add thermal and electrical conversion only after irradiance conservation tests pass.
