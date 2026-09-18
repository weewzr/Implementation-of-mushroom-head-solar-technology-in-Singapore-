# Equal-Resource Geometry Comparison Protocol

## Purpose

A geometry must not appear superior merely because it receives more photovoltaic material, land, or height. Every comparison therefore declares a common resource envelope.

The canonical experiment inherited from the foundation record is

$$
A_{\mathrm{foot}}\le 1\;\mathrm{m^2},\qquad A_{\mathrm{PV}}\le 2\;\mathrm{m^2},\qquad H\le 2\;\mathrm m.
$$

where $A_{\mathrm{foot}}$ is allowed horizontal footprint (m$^2$), $A_{\mathrm{PV}}$ is active PV area (m$^2$), and $H$ is maximum candidate height (m). The numerical values are engineering comparison assumptions, not Singapore regulatory limits or discovered optima, and must later be swept in sensitivity analysis.

The corresponding nominal packing limit is $\Pi_{\max}=A_{\mathrm{PV}}/A_{\mathrm{foot}}=2$.

## Candidate family

Horizontal, optimised fixed tilt, east-west/folded, vertical bifacial where relevant, cone, paraboloid, hemisphere, sphere, sparse/faceted canopy, then tracking and finally free-form topology optimisation are compared progressively.

## Fairness rules

Candidates use the same footprint definition, PV-area budget, weather series, electrical and temperature models, albedo/environment, timestep and evaluation period. Invalid overlap or self-intersection cannot be credited as usable PV area.

$$
M_L=\frac{E_{3D}}{E_{\mathrm{base}}}=\Pi\eta_{\mathrm{pack}}.
$$

where $M_L$ is land multiplication (dimensionless), $E_{3D}$ and $E_{\mathrm{base}}$ are annual candidate and baseline energies (kWh yr$^{-1}$), $\Pi$ is active-area packing ratio (dimensionless), and $\eta_{\mathrm{pack}}$ is average packed-PV productivity relative to baseline (dimensionless).

## Interpretation

This prevents confusing more PV area fitted into a footprint with better cell conversion. Curvature or folding may raise land-normalised energy while lowering energy per square metre of PV; the engineering value then depends on land scarcity, material and structure, shading, maintenance and lifecycle economics.

## Rust implementation

The Rust module `src/candidates.rs` encodes the common resource envelope and candidate family. Subsequent mesh generators and irradiance solvers must consume this contract rather than inventing independent resource assumptions.