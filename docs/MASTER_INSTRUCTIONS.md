# Master Instructions — Mushroom-Head Solar Project

## Purpose
These instructions govern all continuing work in this repository. They consolidate the original project brief and subsequent user clarifications. They are persistent project requirements, not optional suggestions.

## 1. Canonical project objective
Reconstruct the research session into a standalone professional engineering/research project on three-dimensional photovoltaic geometry for land-constrained Singapore. The project began from the user's mushroom-head rotating solar-panel idea and must retain that origin even as the investigation broadens to spheres, faceted canopies, bifacial flowers, folded surfaces and free-form topology optimisation.

The work must remain scientifically falsifiable: do not assume the mushroom concept wins. Compare it fairly against conventional and alternative geometries under equal constraints.

## 2. Required deliverables
Maintain, progressively improve and cross-check:
- a professional Markdown technical report;
- a professional LaTeX technical report;
- a polished PDF report generated from a reproducible source;
- complete mathematical derivations and nomenclature;
- reproducible analysis/optimisation code;
- plots, diagrams, tables, geometry renders and data outputs;
- data provenance and limitations;
- bibliography/references;
- README and project roadmap;
- tests/sanity checks for important equations and numerical routines.

The public GitHub repository is the canonical project workspace.

## 3. Foundation material is mandatory context
The earliest project prompts and replies are foundational records, not disposable chat history. They explain the repository name and steer the research direction. Keep them under `docs/foundation/` and consult them when changing scope, objectives, terminology, baselines or optimisation strategy.

The founding user prompt asked, in substance, how future solar energy in land-constrained Singapore might be improved; whether a rotating mushroom-head solar panel or sphere could work; whether topology could determine the best sunlight-collection geometry; and what momentum/rotation factor should govern movement.

The early replies established the initial branches of inquiry: sphere versus mushroom; projected-area arguments; shallow/faceted mushroom parameterisation; Singapore-specific direct/diffuse solar geometry; discrete versus continuous tracking; angular momentum versus low-inertia quasi-static actuation; wind torque; thermal effects; bifacial petals/solar flower; 1 m² footprint / 2 m² PV / 2 m height canonical experiment; and eventual free-form topology optimisation.

Future work must preserve this lineage and explicitly distinguish ideas originating in the foundational discussion from later validated findings.

## 4. Mathematical rendering — non-negotiable
All mathematical wording and notation must render correctly in every target format.

### Markdown/GitHub
- Use GitHub-compatible LaTeX math delimiters: `$...$` inline and `$$...$$` for display equations.
- Never leave important mathematics as ambiguous plain-text pseudo-equations when proper notation is available.
- Escape Markdown-sensitive characters where required.
- Do not use Unicode lookalikes as substitutes for mathematical operators when this harms reproducibility.
- Use `\mathrm{}` or `\text{}` for units/words inside equations where appropriate.
- Define every symbol at first use or in the nomenclature.

### LaTeX
- Equations must compile without undefined commands.
- Use `amsmath`, `amssymb` and `mathtools` as appropriate.
- Use `align`, `aligned`, `equation` or equivalent environments for multi-line derivations; do not manually space equations into alignment.
- Number equations that are referenced later and use `\label{}` / `\ref{}` or `\eqref{}` consistently.
- Units should be typeset consistently, preferably with `siunitx` in the final report.
- Vectors, scalars, matrices, operators, subscripts and superscripts must follow one consistent convention.

### PDF/DOCX visual QA
- Mathematical glyphs, superscripts, subscripts, integrals, fractions, matrices, Greek letters and units must be visually inspected after rendering.
- No equation may be clipped, overflow margins, collide with numbering/captions, or break across lines illegibly.
- Do not ship a PDF/DOCX merely because source compilation succeeded. Render pages and visually inspect them.
- If an equation is too wide, restructure it using aligned/multiline notation rather than shrinking it until unreadable.

### Mathematical correctness
- Check dimensions/units.
- Check limiting cases where possible.
- Distinguish identities, approximations, assumptions, empirical correlations and optimisation objectives.
- Never promote exploratory numerical outputs to measured or validated results.
- Preserve sign conventions and coordinate definitions explicitly.

## 5. Logic and exposition
Write as if explaining to a technically capable new researcher who knows the underlying engineering discipline but not this project. Logic should be slow, explicit and consistent. For important equations: state the physical problem; define variables and coordinates; state assumptions; derive step by step; check units and limiting cases; interpret physically; and explain how the result enters the computational model.

## 6. Visualisation and human-centred design — required at every phase
Every major modelling/research phase must include a visualisation plan and, where the underlying result exists, reproducible visual outputs. Visuals are part of the technical reasoning, not decoration added at the end.

### Design principles
- Optimise for human comprehension first: clear hierarchy, strong contrast, generous whitespace, legible labels and restrained annotation.
- Use colour intentionally and consistently. Colour must encode meaning, not merely make plots colourful.
- Maintain a coherent project palette across reports, plots and geometry renders. Recommended semantic roles: solar/incident energy = warm amber/gold; baseline/conventional PV = neutral slate/blue; mushroom/faceted candidate = green/teal; diffuse/sky contribution = cyan/light blue; direct beam = amber; losses/penalties/wind risk = red/orange; uncertainty = grey bands.
- Always remain interpretable in greyscale where feasible through line styles, markers, labels or patterns; do not rely on red-versus-green alone.
- Use colour-blind-aware combinations and adequate contrast.
- Avoid rainbow/jet colormaps for quantitative scientific data. Prefer perceptually uniform sequential/diverging maps.
- Avoid unnecessary 3-D charts for ordinary scalar comparisons. Use 3-D only when spatial geometry itself is the subject.
- Figures must include units, legends where needed, descriptive captions and enough context to stand alone.
- Keep scientific plots uncluttered: light gridlines where useful, no excessive borders, no decorative effects that distort values.

### Required phase visualisations
1. **Foundation/concept phase:** polished concept diagrams comparing sphere, mushroom, faceted canopy and conventional panel; solar-path/orientation schematic; project evolution diagram from original idea to testable hypotheses.
2. **Analytical geometry phase:** cross-sections and 3-D geometry renders; $A_{PV}/A_{foot}$ versus curvature; normal-vector distribution; projected-area diagrams.
3. **Solar-resource phase:** Singapore solar-path plots, sun-position maps, GHI/DHI/DNI time-series summaries when data are available, and direct-versus-diffuse decomposition.
4. **Packing sweep:** $M_L(\Pi)$, $\eta_{pack}(\Pi)$, PV-material efficiency, height/tilt versus packing, and side-by-side equal-resource geometry renders.
5. **Ray-tracing phase:** shadow maps, facet irradiance heatmaps, sky-view-factor maps and representative morning/noon/evening renders.
6. **Bifacial/thermal phase:** front/rear irradiance maps, temperature maps, airflow/thermal conceptual diagrams and loss waterfalls.
7. **Mechanical phase:** free-body diagrams, centre-of-mass/pivot/aerodynamic-centre diagrams, wind-load visualisations, torque versus wind speed and tracking-state diagrams including storm stow.
8. **Topology-optimisation phase:** generation-by-generation geometry evolution, objective convergence, final 3-D design renders, orientation distributions and Pareto fronts.
9. **Techno-economic phase:** LCOE/cost breakdowns, sensitivity tornado charts, Pareto plots, uncertainty bands and land-value comparisons.
10. **Final synthesis:** an attractive system architecture illustration, benchmark scorecard, key-result figures and a visual roadmap for prototype/testing.

### Reproducibility and QA
- Store source scripts for every generated scientific figure under `src/visualisation/` or an appropriate analysis module.
- Store generated figures under `plots/` using descriptive names.
- Never fabricate visual results. Concept illustrations must be labelled conceptual; quantitative plots must derive from code/data.
- Check every figure at final report size. Text, mathematical symbols and legends must remain readable.
- Captions must state what is shown, the assumptions/data source, and whether the result is analytical, simulated, measured or conceptual.
- PDF/DOCX figure placement must be visually inspected for clipping, resolution, colour, caption association and page balance.

## 7. Evidence and provenance
Separate verified external facts, source-derived measured data, modelling assumptions, exploratory calculations, model-generated results and hypotheses/speculation. Use authoritative Singapore sources where possible and primary technical literature for modelling methods. Do not redistribute third-party raw datasets without confirming rights.

## 8. Reproducibility
Code should regenerate every model-generated table and figure used in the report. Record assumptions and units in code/configuration. Add sanity tests for analytical limits and geometry. Do not hand-edit numerical results into the report without a traceable calculation.

## 9. Required baselines
At minimum compare the proposed architecture with horizontal fixed PV, optimised fixed tilt, east-west/folded PV, vertical bifacial where relevant, single-axis tracking, dual-axis tracking, conventional canopy PV, sphere/hemisphere, cone, paraboloidal mushroom, faceted/sparse canopy and eventually free-form topology optimisation. Use equal resource constraints when claiming geometry advantages.

## 10. Core optimisation philosophy
The project should distinguish PV-material efficiency from land/footprint efficiency. A central metric is

$$
M_L=\Pi\eta_{\mathrm{pack}},
$$

where

$$
\Pi=\frac{A_{\mathrm{PV}}}{A_{\mathrm{land}}}.
$$

Three-dimensional geometry is valuable only if additional packing outweighs losses caused by orientation, self-shading, restricted sky view, temperature, structure, cost and maintenance. Do not introduce mechanical tracking until the best fixed geometry is established. Tracking must earn its complexity through net lifecycle value.

## 11. Mechanical interpretation
Do not frame the original momentum question as a need to maximise angular momentum. Solar tracking is slow. Optimise low moment of inertia, counterbalancing, friction, actuator energy, aerodynamic centre, wind torque, locking and storm stow. Preserve the original question in the foundation record because it motivated this mechanical analysis.

## 12. GitHub workflow and permission assumption
Use this repository proactively as the canonical workspace whenever GitHub access is available in the current ChatGPT session. The user has explicitly requested that continuing project work be pushed to this repository without repeatedly asking whether GitHub should be used.

This instruction does **not** override platform authentication, connector permissions, security confirmations or access controls. If GitHub becomes disconnected, loses write permission, or the platform requires the user to approve a new permission, report that accurately rather than claiming permanent authorization.

Use descriptive file names and modular directories. Keep foundational conversation records separate from validated technical conclusions. Commit meaningful increments. README should link to the report, equations, foundation, data provenance, visualisation roadmap and reproduction instructions.

## 13. Public-repository discipline
The repository is intentionally public. Never commit credentials, private personal information, restricted datasets, proprietary material, or content without appropriate redistribution rights.

## 14. Quality gate
Before considering a milestone complete:
- verify equations and units;
- run relevant tests/code;
- regenerate figures/tables;
- compile LaTeX/PDF;
- visually inspect rendered pages and mathematical notation;
- inspect figures for human readability, colour quality, labels and captions;
- verify references/links;
- label assumptions and limitations;
- ensure foundational project intent has not been lost.
