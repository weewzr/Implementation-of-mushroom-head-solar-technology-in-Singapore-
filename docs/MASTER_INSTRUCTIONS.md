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
- plots, diagrams, tables and data outputs;
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
Write as if explaining to a technically capable new researcher who knows the underlying engineering discipline but not this project. Logic should be slow, explicit and consistent. For important equations:

1. state the physical problem;
2. define variables and coordinate system;
3. state assumptions;
4. derive the equation step by step;
5. check units and limiting cases;
6. interpret the result physically;
7. explain how it enters the computational model.

Paragraphs should explain what equations mean after the derivation, not merely present algebra.

## 6. Evidence and provenance
Separate:

- verified external facts;
- source-derived measured data;
- modelling assumptions;
- exploratory calculations;
- model-generated results;
- hypotheses/speculation.

Use authoritative Singapore sources where possible (EMA, SERIS/NUS, government/standards) and primary technical literature for modelling methods. Do not redistribute third-party raw datasets without confirming rights.

## 7. Reproducibility
Code should regenerate every model-generated table and figure used in the report. Record assumptions and units in code/configuration. Add sanity tests for analytical limits and geometry. Do not hand-edit numerical results into the report without a traceable calculation.

## 8. Required baselines
At minimum compare the proposed architecture with horizontal fixed PV, optimised fixed tilt, east-west/folded PV, vertical bifacial where relevant, single-axis tracking, dual-axis tracking, conventional canopy PV, sphere/hemisphere, cone, paraboloidal mushroom, faceted/sparse canopy and eventually free-form topology optimisation.

Use equal resource constraints when claiming geometry advantages.

## 9. Core optimisation philosophy
The project should distinguish PV-material efficiency from land/footprint efficiency. A central metric is

$$
M_L=\Pi\eta_{\mathrm{pack}},
$$

where

$$
\Pi=\frac{A_{\mathrm{PV}}}{A_{\mathrm{land}}}.
$$

Three-dimensional geometry is valuable only if additional packing outweighs losses caused by orientation, self-shading, restricted sky view, temperature, structure, cost and maintenance.

Do not introduce mechanical tracking until the best fixed geometry is established. Tracking must earn its complexity through net lifecycle value.

## 10. Mechanical interpretation
Do not frame the original momentum question as a need to maximise angular momentum. Solar tracking is slow. Optimise low moment of inertia, counterbalancing, friction, actuator energy, aerodynamic centre, wind torque, locking and storm stow. Preserve the original question in the foundation record because it motivated this mechanical analysis.

## 11. Repository discipline
Use descriptive file names and modular directories. Keep foundational conversation records separate from validated technical conclusions. Commit meaningful increments. README should link to the report, equations, foundation, data provenance and reproduction instructions.

## 12. Public-repository discipline
The repository is intentionally public. Never commit credentials, private personal information, restricted datasets, proprietary material, or content without appropriate redistribution rights.

## 13. Quality gate
Before considering a milestone complete:

- verify equations and units;
- run relevant tests/code;
- regenerate figures/tables;
- compile LaTeX/PDF;
- visually inspect rendered pages;
- verify references/links;
- label assumptions and limitations;
- ensure foundational project intent has not been lost.
