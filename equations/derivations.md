# Analytical Derivations

## 1. Paraboloidal-cap area

Let
$$z(r)=h\left(1-\frac{r^2}{R^2}\right).$$
For a surface of revolution,
$$dA=2\pi r\sqrt{1+\left(\frac{dz}{dr}\right)^2}\,dr.$$
Since
$$\frac{dz}{dr}=-\frac{2hr}{R^2},$$
we have
$$A_{PV}=2\pi\int_0^Rr\sqrt{1+\frac{4h^2r^2}{R^4}}\,dr.$$
Set
$$u=1+\frac{4h^2r^2}{R^4},\qquad du=\frac{8h^2r}{R^4}dr.$$
Thus
$$r\,dr=\frac{R^4}{8h^2}du,$$
and
$$A_{PV}=\frac{\pi R^4}{4h^2}\int_1^{1+4h^2/R^2}u^{1/2}du.$$
Using $\int u^{1/2}du=\frac23u^{3/2}$,
$$A_{PV}=\frac{\pi R^4}{6h^2}\left[\left(1+\frac{4h^2}{R^2}\right)^{3/2}-1\right].$$
With $k=h/R$,
$$\boxed{A_{PV}=\frac{\pi R^2}{6k^2}[(1+4k^2)^{3/2}-1]}.$$
Dividing by $A_{foot}=\pi R^2$ gives the packing ratio.

### Limiting check
For $k\to0$, $(1+4k^2)^{3/2}=1+6k^2+O(k^4)$, hence $A_{PV}/A_{foot}\to1$. This recovers the flat disk.

## 2. Ideal isotropic diffuse result for a convex graph surface

For an isotropic sky,
$$dP_d=\eta DHI\frac{1+\cos\beta}{2}dA.$$
Therefore
$$P_d=\frac{\eta DHI}{2}\left(\int_SdA+\int_S\cos\beta\,dA\right).$$
The first integral is $A_{PV}$. For a single-valued upward-facing surface, $\cos\beta\,dA=dA_{horizontal}$, so
$$\int_S\cos\beta\,dA=A_{foot}.$$
Hence
$$\boxed{P_d=\frac{\eta DHI}{2}(A_{PV}+A_{foot})}.$$
This result fails to capture self-occlusion, anisotropic cloud radiance and rear-side effects; it is an analytical benchmark, not a yield model.

## 3. Land multiplication identity

By definition
$$M_L=\frac{E_{3D}}{E_{base}},$$
for equal land footprint. Insert $A_{PV}/A_{land}=\Pi$ by multiplying and dividing by $\Pi$:
$$M_L=\Pi\frac{E_{3D}}{\Pi E_{base}}.$$
Define the second factor as $\eta_{pack}$ to obtain
$$\boxed{M_L=\Pi\eta_{pack}}.$$
This separates two competing effects: added collector area and degraded average irradiation quality.

## 4. Energy-density optimum

Let
$$M_L(\Pi)=\Pi\eta_{pack}(\Pi).$$
Differentiate:
$$\frac{dM_L}{d\Pi}=\eta_{pack}+\Pi\frac{d\eta_{pack}}{d\Pi}.$$
At an interior maximum,
$$\eta_{pack}+\Pi\eta'_{pack}=0.$$
Divide by $\eta_{pack}$:
$$1+\frac{\Pi}{\eta_{pack}}\eta'_{pack}=0,$$
so
$$\boxed{-\frac{d\ln\eta_{pack}}{d\ln\Pi}=1}.$$
The interpretation is that packing should cease, for a pure land-energy objective, when a 1% increase in packing causes a 1% loss in average packed-PV productivity.

## 5. Mechanical implication of slow movement

Characteristic inertial torque scales as
$$\tau_I\sim I\frac{\Delta\theta}{T^2}.$$
If the same angular displacement is executed in twice the time,
$$\tau_I(2T)\sim\frac14\tau_I(T).$$
Because solar position changes slowly, tracking can be quasi-static. Wind, friction and gravitational imbalance are therefore expected to be more important mechanical design loads than intentional angular momentum.
