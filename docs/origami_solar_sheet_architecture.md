# Deployable Origami Solar-Sheet Model Architecture

**Status:** future model architecture only; no origami irradiance, mechanical, cost or optimisation result.

## Separation from the static folded candidate
`FixedGeometry::FoldedSurface` is a static irradiance candidate. It must not be described as a folding simulation. The deployable-sheet branch introduces configuration state and kinematic/mechanical validation before a deployed mesh is eligible for the common irradiance pipeline.

## Mesh/state definition
Let the manufactured sheet be (mathcal M=(V,F,C)), with vertices (V={v_i}), facets (F={f_j}), and crease/hinge edges (C={c_k}). A deployment coordinate (lambdain[0,1]) maps the reference sheet to positions (mathbf x_i(lambda)) and crease angles (	heta_k(lambda)). The exact meaning of (lambda=0) must be declared per mechanism (flat manufactured or compact/stowed); (lambda=1) is the fully deployed irradiance-evaluation state.

Rigid-origami variants require invariant edge lengths and facet areas within numerical tolerance. Flexible-sheet variants instead require an explicit deformation/strain model; they may not silently violate rigid invariants.

## Validation gates
Before any deployed state is admitted to irradiance comparison:
- manifold connectivity and consistent facet orientation;
- crease graph/fold compatibility;
- edge-length and facet-area preservation for rigid variants;
- collision/self-intersection rejection throughout the sampled deployment path;
- minimum bend-radius compliance;
- PV cell/interconnect strain limit;
- active-PV-area preservation or explicit inactive crease zones;
- finite/upward-or-declared two-sided facet semantics;
- canonical discrete resource accounting at the deployed state.

## Candidate mechanism families
No winner is selected. Candidate architectures include radial/umbrella folding, Miura-ori and related tessellated origami, accordion/fan folding, roll-out petals, and tensioned-membrane deployment.

## Interface to frozen irradiance physics
Deployment mechanics outputs a validated deployed `Vec<Triangle>`. That mesh then enters the existing canonical `resource_geometry` → `visibility`/sky-view → `annual_irradiance` path without origami-specific irradiance branches. Mechanics and irradiance remain separable.

## Later engineering layers
Transport volume, deployment mechanism, wind/storm stowage, actuator energy, structural/support mass, fatigue, maintenance, reliability, manufacturing and lifecycle cost are later layers. A deployable sheet may plausibly simplify some logistics or structures, but this is a hypothesis requiring mechanical and economic evidence, not a present conclusion.
