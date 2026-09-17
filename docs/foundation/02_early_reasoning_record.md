# Foundation Record 02 — Early Reasoning That Steered the Project

**Status:** Historical reasoning record, not a validated-results document.

This record preserves the principal directions established in the first extended replies to the originating prompt. It should be read together with `01_originating_prompt.md`. Numerical values and hypotheses here are historical project context; validated statements belong in the technical report.

## 1. The objective was reframed around scarce footprint
The early reasoning proposed optimising net annual PV energy per horizontal footprint rather than module efficiency alone:

$$
\max_{\mathcal S,\theta(t)}
\frac{\displaystyle\int P_{\mathrm{PV}}(t,\mathcal S,\theta)\,dt-\displaystyle\int P_{\mathrm{motor}}(t)\,dt}
{A_{\mathrm{footprint}}}.
$$

This remains central to the project.

## 2. Arbitrary-surface solar collection
For an elemental PV area $dA$ with unit normal $\mathbf n$ and solar unit vector $\mathbf s$, the direct-beam starting point was

$$
dP_{\mathrm{dir}}=\eta\,DNI\,[\mathbf n\cdot\mathbf s]_+\,dA,
$$

with diffuse and reflected components added separately. This led naturally to triangulated/faceted geometry and later ray tracing.

## 3. Sphere versus mushroom
The sphere was identified as directionally robust but PV-material intensive: its surface area is $4\pi R^2$ while its projected disk is $\pi R^2$. This motivated treating the sphere as a baseline rather than presuming it optimal.

The mushroom was parameterised as a variable-curvature cap, initially

$$
z(r)=h\left[1-\left(\frac rR\right)^p\right],
$$

with the paraboloidal case $p=2$ becoming the main analytical baseline. The purpose was to let curvature vary rather than assuming a hemisphere.

## 4. Singapore-specific solar geometry
The near-equatorial solar path motivated explicit solar-vector modelling rather than importing a mid-latitude fixed-tilt rule. The early discussion also emphasised diffuse irradiance as potentially important in Singapore. Later project work correctly tightened this point: annual-average diffuse shares must not be used as if they were time-resolved DNI/DHI data.

## 5. Rotation and the momentum question
The original momentum question led to the mechanics relations

$$
L=I\omega,
\qquad
E_k=\frac12I\omega^2,
$$

and

$$
\tau=I\ddot\theta+\tau_{\mathrm f}+\tau_{\mathrm g}+\tau_{\mathrm w}.
$$

The early conclusion was that a solar tracker should generally not seek large angular momentum. The more useful objectives are low inertia, slow quasi-static motion, counterbalancing, low friction, locking and reduced wind moment.

## 6. Discrete movement rather than continuous chasing
The discussion proposed testing a small number of orientation states per day instead of assuming continuous tracking. This remains a project requirement: solve the best fixed geometry first, then quantify the incremental value of discrete and continuous tracking.

## 7. Wind as a governing mechanical constraint
Wind loading was recognised early as potentially more important than inertial tracking torque:

$$
F_D=\frac12\rho C_DA_{\mathrm{proj}}v^2,
\qquad
\tau_w\approx F_Dr_{CP}.
$$

This motivated porous/faceted geometries and a storm-stow mode.

## 8. Evolution from mushroom to faceted flower
The early discussion progressively considered a smooth mushroom, faceted mushroom, separated petals, bifacial petals and a sparse solar flower. The motivation was improved sky view, rear irradiance, ventilation and lower wind solidity. These are hypotheses to test, not conclusions.

## 9. Canonical constrained experiment
A particularly important early formulation was to give every candidate the same resources:

$$
A_{\mathrm{footprint}}=1\;\mathrm{m^2},
\qquad
A_{\mathrm{PV}}=2\;\mathrm{m^2},
\qquad
H_{\max}=2\;\mathrm m.
$$

Then ask how the PV should be arranged in three dimensions. This remains the canonical topology-optimisation experiment, supplemented by a packing-ratio sweep.

## 10. Free-form topology optimisation
The project direction ultimately became: do not force the optimiser to return a mushroom. Give it a constrained volume and PV budget, evaluate solar access and engineering penalties, and determine whether a mushroom-like, folded, flower-like or unexpected geometry emerges.

## 11. Core intellectual lineage
The project therefore follows this chain:

$$
\text{mushroom/sphere idea}
\rightarrow
\text{surface geometry}
\rightarrow
\text{Singapore solar field}
\rightarrow
\text{PV packing}
\rightarrow
\text{self-shadowing/bifaciality}
\rightarrow
\text{mechanics and wind}
\rightarrow
\text{free-form topology optimisation}.
$$

Future reports should reference this foundation when explaining why each modelling layer exists.
