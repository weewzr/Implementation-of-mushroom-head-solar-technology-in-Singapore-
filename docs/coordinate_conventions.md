# Coordinate and Sign Conventions

**Status:** canonical foundation convention. This document must remain consistent with the Markdown report, LaTeX report and Rust implementation before higher-fidelity ray tracing or tracking is introduced.

## 1. Local Cartesian frame

Use a right-handed local **East–North–Up (ENU)** Cartesian frame:

- $+x$: east;
- $+y$: north;
- $+z$: vertically upward.

A position vector is

$$
\mathbf r=(x,y,z).
$$

where $\mathbf r$ is position (m) and $x$, $y$, $z$ are east, north and upward coordinates respectively (m). The coordinate axes are definitions, not measured parameters.

### Physical interpretation

The ENU convention matches the physical horizontal plane and makes the vertical component of a surface normal directly interpretable as upward-facing projection.

### Engineering implication

All geometry generators, sun vectors, facet normals, visibility rays and mechanical axes must use this same frame. A geometry must not silently change axes between analytical equations and Rust.

## 2. Surface normals

Each active facet has an outward/front-side unit normal

$$
\mathbf n=(n_E,n_N,n_U),\qquad \|\mathbf n\|=1.
$$

where $n_E$, $n_N$, and $n_U$ are dimensionless east, north and upward components, and $\|\mathbf n\|$ is the Euclidean norm. For a horizontal upward-facing monofacial facet, $\mathbf n=(0,0,1)$.

Front-side direct incidence requires

$$
\mathbf n\cdot\mathbf s>0.
$$

where $\mathbf s$ is the unit vector from the receiving point toward the Sun (dimensionless). The dot product is dimensionless. The positive-part operator $[\mathbf n\cdot\mathbf s]_+$ sets grazing/backside beam contribution to zero for the monofacial front-side benchmark.

## 3. Solar vector

Represent the Sun direction by solar elevation $\alpha$ and solar azimuth $\gamma_s$. Adopt **azimuth measured clockwise from geographic north**: north $=0^\circ$, east $=90^\circ$, south $=180^\circ$, west $=270^\circ$. This convention must be stated whenever azimuth data from an external source are imported because other libraries may use different definitions.

In ENU coordinates,

$$
\mathbf s=\left(\cos\alpha\sin\gamma_s,\;\cos\alpha\cos\gamma_s,\;\sin\alpha\right).
$$

where $\mathbf s$ is the dimensionless unit vector toward the Sun, $\alpha$ is solar elevation above the horizon (rad in Rust; degrees may be shown in explanatory text), and $\gamma_s$ is clockwise-from-north solar azimuth (rad in Rust).

### Sanity checks

- At zenith, $\alpha=90^\circ$ and $\mathbf s=(0,0,1)$.
- At the eastern horizon, $\alpha=0^\circ$, $\gamma_s=90^\circ$, and $\mathbf s=(1,0,0)$.
- At the northern horizon, $\alpha=0^\circ$, $\gamma_s=0^\circ$, and $\mathbf s=(0,1,0)$.
- The vector norm is unity for all $\alpha$ and $\gamma_s$.

## 4. Solar hour angle

Retain the existing convention

$$
H=15^\circ\,\mathrm{h}^{-1}(t_{\mathrm{solar}}-12\,\mathrm h),
$$

where $H$ is solar hour angle, negative before apparent solar noon and positive after apparent solar noon; $t_{\mathrm{solar}}$ is local apparent solar time (h); $15^\circ$ h$^{-1}$ follows exactly from $360^\circ/24$ h; and 12 h is apparent solar noon.

Do not substitute Singapore civil clock time directly for apparent solar time.

## 5. Tilt and azimuth of a facet

Define facet tilt $\beta$ from horizontal: $\beta=0$ for upward horizontal and $\beta=90^\circ$ for vertical. Define facet azimuth $\gamma_p$ using the same clockwise-from-north convention as solar azimuth.

The corresponding front normal is

$$
\mathbf n=\left(\sin\beta\sin\gamma_p,\;\sin\beta\cos\gamma_p,\;\cos\beta\right).
$$

where $\beta$ and $\gamma_p$ are angles (rad in Rust), and all vector components are dimensionless.

## 6. Tracker angles

No universal tracker-angle sign convention is yet validated because the mechanical axis depends on candidate architecture. Each tracking model must define its rotation axis as an ENU unit vector and use the right-hand rule for positive rotation. Do not introduce a generic positive/negative tracker angle before the axis is specified.

## 7. Rust requirements

The canonical Rust vector type must document ENU ordering explicitly. Future solar-vector and facet-orientation constructors must accept radians internally and include the sanity tests above. External azimuth conventions must be converted at the data boundary rather than spread through model code.

## 8. Status

This convention is a project definition, not an empirical result. It closes a foundation ambiguity identified by the mandatory three-pass audit. It does not validate the current preliminary solar-position approximation or annual Singapore yield model.