# Two-Technique Methodology and Verification/Validation Ladder

## Purpose

The project uses two complementary mathematical routes before optimisation:

1. **Technique A — analytical/continuous modelling**, used where geometry and physics admit exact or controlled approximate integration.
2. **Technique B — discrete numerical surface modelling**, used to reproduce the analytical benchmarks first and then extend the same physics to geometries without convenient closed forms.

The methods are not independent competing answers. Technique A supplies exact/limiting benchmarks and physical insight; Technique B supplies geometric generality. A numerical result is not promoted merely because it executes.

---

## 1. Common problem definition

For a PV surface (S), position is expressed in canonical East-North-Up coordinates and the outward unit normal is (mathbf n(mathbf x)). The Sun direction is (mathbf s(t)).

The direct electrical contribution is

[
P_{mathrm{dir}}(t)
=
int_S
eta(T),DNI(t),V(mathbf x,t)
[mathbf n(mathbf x)cdotmathbf s(t)]_+
,dA.
]

The first diffuse benchmark is

[
P_{mathrm{diff}}(t)
=
int_S
eta(T),DHI(t),F_{mathrm{sky}}(mathbf x,t)
,dA.
]

For an unobstructed isotropic sky on a planar local element,

[
F_{mathrm{sky}}=rac{1+coseta}{2}.
]

Annual energy, once the input and physics layers are validated, is

[
E=int_{t_0}^{t_1}P(t),dt.
]

Both techniques must use the same coordinate convention, resource envelope, irradiance definitions, efficiency/loss definitions and comparison metrics.

---

# 2. Technique A — analytical/continuous method

## 2.1 Role

The analytical route answers questions that can be solved exactly or with a transparent controlled approximation. It provides:

- exact geometry identities;
- dimensional and limiting checks;
- physical interpretation;
- benchmark solutions for numerical verification;
- sensitivity relations;
- reduced-order insight before high-dimensional computation.

It is not required that every final candidate have a closed-form solution.

## 2.2 Step A1 — define the continuous geometry

For the founding paraboloid,

[
z(r)=hleft(1-rac{r^2}{R^2}ight),qquad 0le rle R,
]

with aspect ratio

[
k=rac{h}{R}.
]

Before solving, declare the domain, orientation, footprint and whether the surface is single-valued, convex, monofacial/bifacial and stationary/moving.

### Gate A1
Check dimensions, coordinate convention, boundary values (z(0)=h), (z(R)=0), and the flat limit (h	o0).

## 2.3 Step A2 — derive differential geometry

For an axisymmetric graph,

[
dA=2pi rsqrt{1+left(rac{dz}{dr}ight)^2},dr.
]

For the paraboloid,

[
rac{dz}{dr}=-rac{2hr}{R^2}.
]

The exact active area follows from integration.

### Gate A2
Verify units of (dA), sign/meaning of the slope, and the (k	o0) disk limit.

## 2.4 Step A3 — solve exact geometric benchmarks

[
A_{mathrm{PV}}
=
rac{pi R^2}{6k^2}
left[(1+4k^2)^{3/2}-1ight],
]

and

[
Pi_A=
rac{A_{mathrm{PV}}}{A_{mathrm{foot}}}
=
rac{(1+4k^2)^{3/2}-1}{6k^2}.
]

Other exact benchmarks include

[
A_{mathrm{sphere}}=4pi R^2,
qquad
A_{mathrm{hemisphere}}=2pi R^2.
]

### Gate A3
Check exact identities independently, dimensional consistency and known limiting cases.

## 2.5 Step A4 — integrate simplified irradiance physics

Under the unobstructed isotropic-sky assumptions for an upward single-valued surface,

[
P_{mathrm{diff}}
=
rac{eta DHI}{2}
left(A_{mathrm{PV}}+A_{mathrm{foot}}ight).
]

This follows from

[
int_S coseta,dA=A_{mathrm{foot}}.
]

This result is a benchmark for diffuse geometry, not a complete Singapore irradiance model.

### Gate A4
State every assumption explicitly: isotropic sky, no obstruction, compatible orientation, uniform efficiency and uniform DHI over the surface.

## 2.6 Step A5 — derive dimensionless comparison quantities

Use

[
Pi=rac{A_{mathrm{PV}}}{A_{mathrm{land}}},
]

[
eta_{mathrm{pack}}=
rac{E_{3D}}{Pi E_{mathrm{base}}},
]

and

[
M_L=Pieta_{mathrm{pack}}.
]

For a differentiable energy-density-only optimum,

[
-rac{dlneta_{mathrm{pack}}}{dlnPi}=1.
]

### Gate A5
Confirm the numerator/denominator use the same weather interval, electrical definition and resource basis.

## 2.7 Step A6 — analytical sensitivity and asymptotics

Analytical work should include:

- (k	o0) shallow-cap expansion;
- monotonicity of packing with curvature;
- sensitivity (dPi_A/dk);
- ideal diffuse ratio versus (k);
- dimensional scaling with (R);
- slow-motion scaling (	au_Isim IDelta	heta/T^2).

These analyses explain trends before optimisation.

## 2.8 Analytical verification

Analytical expressions should be verified by at least two of:

1. independent algebraic derivation;
2. differentiation/integration back-check;
3. limiting case;
4. dimensional analysis;
5. numerical quadrature;
6. comparison with an exact geometric identity.

## 2.9 Analytical validation

Analytical mathematics can be correct while the physical assumptions are inadequate. Validation therefore asks whether the simplified model represents the intended physical regime. Examples:

- isotropic diffuse model versus a higher-fidelity diffuse model;
- drag equation against an appropriate aerodynamic basis;
- thermal coefficient model against selected module data;
- solar-position approximation against authoritative SPA.

A mathematically exact result under an invalid physical assumption is not a validated system result.

---

# 3. Technique B — discrete numerical surface method

## 3.1 Role

The discrete method approximates a continuous or free-form PV surface by finite planar facets. It must first reproduce Technique A benchmarks before it is trusted on arbitrary geometries.

## 3.2 Step B1 — discretise geometry

For triangular facet (i),

[
mathbf e_{i1}=mathbf v_{i2}-mathbf v_{i1},
qquad
mathbf e_{i2}=mathbf v_{i3}-mathbf v_{i1},
]

[
A_i=rac12
|mathbf e_{i1}	imesmathbf e_{i2}|,
]

[
mathbf n_i=
rac{mathbf e_{i1}	imesmathbf e_{i2}}
{|mathbf e_{i1}	imesmathbf e_{i2}|}.
]

Vertex winding must produce the intended outward/front normal.

### Gate B1
Reject degenerate facets, non-positive areas, inconsistent winding and resource-envelope violations.

## 3.3 Step B2 — discrete geometric integrals

[
A_{mathrm{PV}}^{(N)}
=
sum_{i=1}^{N}A_i.
]

For an upward triangulated graph surface, the projected-area check is

[
A_{mathrm{proj},z}^{(N)}
=
sum_i A_i n_{z,i}.
]

For the paraboloid benchmark this should converge to the circular footprint.

### Gate B2
Require area and projected-area convergence before irradiance calculations.

## 3.4 Step B3 — discrete direct irradiance

At time (t_n),

[
P_{mathrm{dir}}^{(N)}(t_n)
=
eta(T)
DNI(t_n)
sum_i
A_iV_i(t_n)
[mathbf n_icdotmathbf s(t_n)]_+.
]

Before ray tracing is enabled, (V_i=1) may be used only for a declared no-occlusion benchmark.

### Gate B3
Test normal incidence, grazing incidence, backside incidence, cardinal directions and visibility edge cases.

## 3.5 Step B4 — discrete diffuse irradiance

For the ideal isotropic unobstructed benchmark,

[
P_{mathrm{diff}}^{(N)}
=
rac{eta DHI}{2}
sum_i A_i(1+n_{z,i}).
]

Later, replace the simple view factor with explicit sky-patch integration,

[
G_{mathrm{diff},i}
approx
sum_{p=1}^{N_{mathrm{sky}}}
L_p
V_{ip}^{mathrm{sky}}
max(0,mathbf n_icdotmathbf s_p)
DeltaOmega_p.
]

### Gate B4
First reproduce the analytical isotropic benchmark. Then demonstrate sky-patch convergence before using the anisotropic/obstructed model.

## 3.6 Step B5 — time discretisation

For timestamps (t_n),

[
E
approx
sum_{n=0}^{N_t-1}
P(t_n)Delta t_n.
]

Irregular weather intervals must use their actual interval durations rather than assuming constant (Delta t).

### Gate B5
Demonstrate time-step/data-interval consistency and energy-unit correctness.

## 3.7 Step B6 — mesh convergence

For output (Q_N),

[
arepsilon_N
=
rac{|Q_N-Q_{mathrm{ref}}|}{|Q_{mathrm{ref}}|}.
]

For the paraboloid, (Q_{mathrm{ref}}) is the analytical result. For geometries without a closed form, use successive refinement and report the change between resolutions.

The project should not select a mesh because it 'looks smooth'. It must establish a quantitative convergence criterion.

## 3.8 Step B7 — numerical robustness

Test sensitivity to:

- radial/azimuthal facet count;
- triangle orientation;
- ray tolerance;
- sky-patch count;
- timestep;
- floating-point tolerances;
- geometry perturbations.

Numerical settings are model inputs and belong in the provenance/configuration record.

---

# 4. Verification ladder between the two techniques

The paraboloid is the central bridge.

### Level V1 — geometry — PASS

$$
A_{\mathrm{PV}}^{(N)}
\rightarrow
A_{\mathrm{PV}}^{(\mathrm{analytical})}.
$$

### Level V2 — projection — PASS

$$
\sum_i A_i n_{z,i}
\rightarrow
A_{\mathrm{foot}}.
$$

### Level V3 — ideal diffuse — PASS

$$
P_{\mathrm{diff}}^{(N)}
\rightarrow
P_{\mathrm{diff}}^{(\mathrm{analytical})}.
$$

### Level V4 — direct-incidence special cases — PASS

Discrete facet calculations pass exact normal-incidence, grazing-incidence and backside-clipping cases.

### Level V5 — time integration — PASS

Synthetic constant-power and piecewise-constant cases recover their exact integrated energies.

The V1–V5 numerical-method verification ladder passed in GitHub Actions run 36176587453 after correcting mesh winding, projected-area acceptance logic and the verification workflow command. This closes numerical-method verification for the present analytical/discrete paraboloid bridge only. It does not close the separate physical-validation ladder below.

---

# 5. Physical validation ladder

Verification asks **whether the equations/code are solved correctly**. Validation asks **whether the model represents the real system sufficiently well**.

1. Solar position: benchmark against authoritative SPA.
2. Weather: canonical Singapore timestamps, metadata and GHI/DHI/DNI consistency.
3. POA/direct incidence: benchmark against accepted transposition/reference cases.
4. Diffuse sky: compare isotropic benchmark with selected higher-fidelity model.
5. Visibility: test ray tracing on geometries with known blocked/unblocked rays.
6. Thermal/electrical: validate against selected module/model data.
7. Mechanics: validate load/torque assumptions before actuator conclusions.
8. Annual integration: perform energy/unit/data-gap checks.
9. Uncertainty: propagate material input/model uncertainty.
10. Candidate comparison: only then compare equal-resource annual results.

---

# 6. FTCS and transient thermal extension

FTCS is reserved for a genuine transient PDE such as

[
ho c_prac{partial T}{partial t}
=
k_T
abla^2T+dot q.
]

For a simple one-dimensional uniform grid,

[
T_j^{n+1}
=
T_j^n
+
Foleft(T_{j+1}^n-2T_j^n+T_{j-1}^night)
+
rac{Delta t}{ho c_p}dot q_j^n,
]

where

[
Fo=rac{alpha_TDelta t}{Delta x^2},
qquad
alpha_T=rac{k_T}{ho c_p}.
]

For the classical 1-D diffusion-only explicit scheme, the familiar stability requirement is

[
Folerac12.
]

This condition is not automatically transferable to a future multi-dimensional, nonlinear or coupled PV thermal model. The actual discretisation must be derived and stability-tested when that layer is implemented.

---

# 7. Method-release rule

A method layer may feed the next layer only when:

- its equations and assumptions are documented;
- units and symbols are defined;
- limiting/sanity cases pass;
- numerical implementation has tests;
- convergence is demonstrated where numerical discretisation is involved;
- provenance is recorded for external inputs;
- validation status is stated explicitly.

This ladder prevents later optimisation from amplifying an error introduced in an earlier physical or numerical layer.
