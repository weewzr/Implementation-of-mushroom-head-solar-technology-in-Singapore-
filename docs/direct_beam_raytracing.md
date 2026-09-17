# Direct-Beam Ray-Tracing Phase

## Purpose
This phase is the first comparison in which candidate geometries are represented as actual triangular meshes and can shade themselves. It directly follows the founding mushroom-head question while removing the earlier ideal assumption that every surface element sees the sun.

## Equal-resource comparison
For each packing ratio

$$
\Pi=\frac{A_{\mathrm{PV}}}{A_{\mathrm{foot}}},
$$

the paraboloidal mushroom, cone and accordion are generated using the same footprint and approximately the same PV area. The direct-beam geometric factor is

$$
A_{\mathrm{beam}}(t)=\sum_{i=1}^{N} A_iV_i(t)\left[\mathbf n_i\cdot\mathbf s(t)\right]_+,
$$

where $V_i(t)=1$ when a ray from facet $i$ toward the sun reaches the sky without intersecting another triangle, and $V_i(t)=0$ otherwise.

The current annual-path experiment integrates this quantity over sampled solar positions and normalises it by the corresponding projected area of a $1\,\mathrm{m^2}$ horizontal reference:

$$
R_{\mathrm{beam,geom}}=
\frac{\sum_t A_{\mathrm{beam}}(t)}{\sum_t s_z(t)A_{\mathrm{ref}}}.
$$

This ratio is **not annual electricity yield**. Solar positions currently receive equal temporal weighting; measured/validated $DNI(t)$ has not yet been applied.

## Ray intersection
The implementation uses a ray/triangle intersection test. For a ray

$$
\mathbf r(\lambda)=\mathbf r_0+\lambda\mathbf s,
$$

an intersection with any other facet at $\lambda>0$ blocks the direct beam. The ray origin is displaced by a small numerical tolerance along $\mathbf s$ to prevent self-intersection.

## Required visualisations
The script `src/visualisation/direct_beam_visuals.py` creates morning, noon and afternoon 3-D comparisons at equal $\Pi$. Colour intensity encodes the illuminated cosine factor and grey denotes a back-facing or shadowed facet. The sun direction is shown explicitly.

These are simulation visualisations, not architectural renderings. They must remain visually attractive but scientifically literal.

## Interpretation rules
1. Do not call the largest geometric ratio the best solar system.
2. Do not combine these results with an annual diffuse percentage as though the two time series were independent.
3. Verify mesh-area convergence before comparing geometries.
4. Apply time-correlated Singapore irradiance before reporting annual energy.
5. Add sky-patch visibility before claiming diffuse gains.
6. Later include neighbouring-unit shading because an isolated geometry can perform differently in an array.

## Next phase
The next optical model is a discretised sky hemisphere. Each facet will cast rays toward sky patches to calculate a numerical sky-view factor and, later, anisotropic diffuse irradiance. That is necessary before a high-diffuse Singapore environment can be represented credibly.
