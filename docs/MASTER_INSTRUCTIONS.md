# Master Instructions — Mushroom-Head Solar Project

> **CANONICAL GOVERNANCE NOTICE — 18 September 2026**
>
> The previous repository version condensed the user's detailed master instruction. That condensation is not an acceptable substitute for the full instruction. The detailed master instruction governs the project; summaries are checklists only and must never replace or weaken it.
>
> Required intellectual chain: Problem -> Physics -> Assumptions -> Mathematics -> Derivation -> Computation -> Results -> Interpretation -> Engineering Decision.

## Full 46-part requirement map

1. Reconstruct the project from the research-session notebook; extract objectives, questions, mechanisms, mathematics, assumptions, numerical work, sources, alternatives, corrections, uncertainties, experiments and future work; reorganise logically and resolve or flag contradictions.
2. Use slow, cumulative technical exposition: physical intuition -> definitions -> assumptions -> governing principle -> formulation -> derivation -> calculation -> result -> interpretation -> engineering implication.
3. Explain the technology and physical system before advanced modelling.
4. Use a professional research/engineering report structure, adapting headings to relevance rather than forcing a template.
5. Typeset mathematics professionally; define every important symbol, unit and physical meaning; maintain consistent notation and nomenclature.
6. Derive important relationships from first principles where appropriate and label laws, definitions, assumptions, conditions, derived relations, correlations, approximations, estimates and calculated results distinctly.
7. Show important workings step-by-step, including coordinates, conditions, assumptions, simplification, substitution, unit checks, plausibility and interpretation.
8. Perform dimensional, unit, order-of-magnitude, limiting-case and physical/literature sanity checks.
9. Follow major mathematics with Physical Interpretation and Engineering Implication.
10. Create useful labelled technical drawings and diagrams; number, caption and discuss every important figure.
11. Use external photographs/figures only when materially useful, with attribution; prefer original schematics.
12. Generate professional reproducible graphs with labelled axes/units, legends where needed, captions, and mathematical/physical/engineering interpretation.
13. Maintain parameter/data tables that distinguish measured, published, manufacturer, calculated, simulated, estimated and assumed values.
14. Use authoritative evidence and numbered citations; prefer primary sources; never fabricate sources, DOI data or values; mark unverifiable items Requires verification.
15. Distinguish sourced knowledge, assumptions, calculated/simulated results, estimates, hypotheses, proposed designs and unresolved questions.
16. Apply engineering rigour to conservation laws, conditions, properties, geometry, operation, losses, parasitics, mechanics, thermal/environmental constraints, manufacturability, reliability, scalability, cost, safety and uncertainty.
17. Formulate optimisation explicitly with decision variables, objectives, inequality/equality constraints and bounds; explain their physical meaning and trade-offs; never invent an optimum.
18. Document computational algorithms, inputs/outputs, discretisation, solver, convergence, optimisation/stopping criteria, assumptions and limitations.
19. Analyse uncertainty and sensitivity; avoid unjustified precision; identify parameters requiring better evidence.
20. Preserve meaningful alternative designs and compare them on relevant physical and engineering criteria.
21. Use professional technical English with coherent paragraphs, clear hierarchy, defined acronyms and no chatty filler or hype.
22. Preserve calculation traceability: Source/Input -> Assumption -> Equation -> Calculation -> Result -> Interpretation -> Design implication.
23. Use appendices for supporting derivations, data, parameter tables, sensitivity work, code/algorithms and supplementary calculations.
24. Maintain a complete professional Markdown report with equations, tables, figures, captions, citations, bibliography and appendices.
25. Maintain a complete compilable professional LaTeX report with proper packages, equation numbering, alignment, SI notation, references and nomenclature; compile before delivery.
26. Maintain a genuinely typeset professional PDF with title page, contents, lists where useful, page numbers, typography, rendered mathematics, figures/tables, references and appendices.
27. Render and visually inspect the PDF page-by-page; fix overflow, split equations, tiny/blurry figures, detached captions, broken references, missing glyphs/citations, bad page breaks, overlaps and unreadable labels; regenerate and recheck.
28. Preserve reproducible modular computational work rather than final answers only. Project-specific constraint: use Rust, not Python, for modelling/analysis/optimisation/visualisation code.
29. Cross-reference report equations and source implementation so principle -> equation -> code -> result -> graph -> engineering conclusion is traceable.
30. Make figures reproducible from source data through analysis/model and plotting code; preserve vector output where useful plus GitHub-friendly raster output.
31. Document data provenance, URL/DOI, access date where relevant, units, variables, preprocessing, transformations, assumptions, missing-data handling and limitations; respect redistribution rights.
32. Use this dedicated GitHub repository as the canonical workspace. Although the generic master prompt defaulted to private, this project was subsequently explicitly made public, so public-repository safety applies.
33. Maintain a meaningful repository structure for reports, equations, figures, plots, data, source, references and methodology; do not create empty directories merely to mimic a template.
34. Maintain a professional README covering overview, motivation, research question, approach, concept diagram, principal equations, methodology, results, structure, reproduction, report links, data, references, limitations and future work.
35. Maintain a valid BibTeX bibliography with real metadata and matching LaTeX citations.
36. Maintain version-control hygiene and never commit credentials, secrets, private configuration, unnecessary temporary files, restricted material or sensitive information.
37. Do not assume licensing rights; respect third-party code/data/figure/software licences and attribution.
38. Validate Markdown, LaTeX, PDF, code, reproducibility, repository paths, README, data docs, references, gitignore and secret safety before milestone publication.
39. Use the connected GitHub integration directly when available; do not stop at manual instructions when repository operations are permitted.
40. Use meaningful commits for substantive increments without manufacturing needless commits.
41. Inspect the GitHub repository after pushes; confirm visibility, README, reports, figures, plots, source, data instructions, bibliography, links and absence of credentials; fix breakage.
42. Perform a final mathematical, physical, numerical, notation, logical, evidence, figure, reproducibility and completeness review.
43. Do not over-compress. Derive rather than merely state; explain rather than merely calculate; interpret rather than merely report.
44. Maintain the actual final deliverables: README; Markdown/LaTeX/PDF reports; equations/derivations/nomenclature; figures; plots; documented data; modular source; bibliography; methodology.
45. At milestone handover, link repository/report/source/equations/figures/data/references and summarise reconstruction, research, models, checked calculations, datasets, visuals, tools, optimisation, assumptions, uncertainties, unresolved questions and next steps.
46. Apply professor/research-supervisor/professional-engineer quality: the repository must read as deliberately researched, mathematically developed, computationally implemented, documented and checked, not as an exported ChatGPT conversation.

## Project-specific non-negotiable clarifications

- Audit compliance with this master instruction at least every three project turns and record substantive audits under docs/audits/.
- Immediately after every important displayed equation, define every introduced symbol and unit; a distant nomenclature table is supplementary, not a substitute.
- Justify every non-trivial numerical constant/default and cite authoritative sources for empirical/astronomical values.
- Rust-only computational implementation is the current user constraint. Existing Python remnants are migration debt and must not remain the canonical reproduction workflow.
- Preserve the originating mushroom-head/sphere/topology/rotation question and early reasoning under docs/foundation/ as mandatory project lineage.
- Keep the investigation falsifiable: do not assume the mushroom geometry wins; use equal-resource baselines.
- Put required diagrams inside the report, not merely elsewhere in the repository.
- Do not promote exploratory calculations to validated conclusions before appropriate Singapore time-resolved data and higher-fidelity physics support them.

---

## Existing operational detail retained from the earlier repository version
## Purpose
These instructions govern all continuing work in this repository. They consolidate the original project brief and subsequent user clarifications. They are persistent project requirements, not optional suggestions.

## 1. Canonical project objective
Reconstruct the research session into a standalone professional engineering/research project on three-dimensional photovoltaic geometry for land-constrained Singapore. The project began from the user's mushroom-head rotating solar-panel idea and must retain that origin even as the investigation broadens to spheres, faceted canopies, bifacial flowers, folded surfaces and free-form topology optimisation. The work must remain scientifically falsifiable: do not assume the mushroom concept wins.

## 2. Required deliverables
Maintain and cross-check professional Markdown, LaTeX and PDF reports; complete derivations and nomenclature; reproducible analysis/optimisation code; plots, diagrams, tables and geometry renders; data provenance; bibliography; README/roadmap; and tests for important equations and routines. The public GitHub repository is the canonical workspace.

## 3. Foundation material is mandatory context
The earliest prompts and replies are foundational records. They explain the repository name and steer the project. Keep them under `docs/foundation/` and consult them when changing scope, objectives, terminology, baselines or optimisation strategy. The founding question concerns Singapore land constraints, mushroom-head/spherical PV, topology optimisation of sunlight collection and the mechanics/momentum of rotation. Future work must preserve this lineage while distinguishing originating ideas from validated findings.

## 4. Mandatory master-instruction audit every three project turns
At least once every three user/assistant project turns, pause expansion and audit the work against this file. The audit must check: foundation alignment; mathematical rendering; numerical-constant justification; variable definitions/units; source/provenance status; reproducibility; visualisation coverage; equal-resource baselines; public-repository safety; and whether any exploratory result has accidentally been written as a validated conclusion. Correct identified deficiencies before continuing major new modelling work. Record substantive audits under `docs/audits/`.

## 5. Mathematical rendering, variable definitions and numerical justification — non-negotiable
All mathematical wording and notation must render correctly in every target format.

### Equation-by-equation explanatory rule
Immediately after every important displayed equation, provide a `where` block or equivalent prose defining **every symbol introduced in that equation**, including units. Do not rely solely on a distant nomenclature table. The nomenclature is supplementary.

Example:

$$
H=15^\circ(t_{\mathrm{solar}}-12).
$$

where:
- $H$ is the solar hour angle (degrees, $^\circ$);
- $t_{\mathrm{solar}}$ is local apparent solar time (hours, h);
- $15^\circ\,\mathrm{h}^{-1}$ is the Earth's mean angular rotation rate relative to solar time, obtained from $360^\circ/24\,\mathrm h$;
- $12\,\mathrm h$ denotes apparent solar noon, at which $H=0^\circ$.

### Numerical constants
Every non-trivial numerical constant appearing in an equation, model, table or code default must be justified. For each constant, state which category it belongs to:
1. exact definition/conversion;
2. astronomical/geometrical approximation coefficient;
3. externally sourced measured/empirical value;
4. engineering design assumption;
5. numerical/computational setting.

Give the origin and units. Cite an authoritative source for categories 2 and 3. Explain the rationale and sensitivity for category 4. Explain convergence/numerical consequences for category 5 where material.

Never write unexplained expressions such as

$$
\delta(n)=23.45^\circ\sin\left[\frac{360^\circ}{365}(284+n)\right]
$$

without explaining the provenance and approximation behind $23.45^\circ$, $360^\circ$, $365$ and the phase shift $284$. If an approximation is retained for teaching or preliminary modelling, identify it explicitly as an approximation and state its expected role/limitations. Prefer a traceable higher-accuracy solar-position method for validated results.

### Markdown/GitHub
Use GitHub-compatible `$...$` and `$$...$$`; avoid ambiguous pseudo-equations; escape Markdown-sensitive characters; define symbols immediately after equations; and use consistent mathematical typography.

### LaTeX
Equations must compile. Use `amsmath`, `amssymb`, `mathtools` and `siunitx` as appropriate. Use proper alignment environments. Number referenced equations and use labels/references consistently. Maintain consistent scalar/vector/matrix conventions.

### PDF/DOCX visual QA
Visually inspect mathematical glyphs, fractions, matrices, Greek letters and units after rendering. No clipping/overflow. Restructure wide equations instead of shrinking them illegibly.

### Mathematical correctness
Check dimensions, units and limiting cases. Distinguish exact identities, approximations, assumptions, empirical correlations and optimisation objectives. Preserve coordinate/sign conventions. Never promote exploratory outputs to measured/validated results.

## 6. Logic and exposition
Write for a technically capable new researcher. For important equations: state the physical problem; define coordinates; state assumptions; derive step-by-step; justify every numerical coefficient; define every variable and unit immediately afterward; check dimensions and limiting cases; interpret physically; and explain implementation.

## 7. Visualisation and human-centred design — required at every phase
Every major phase must include a visualisation plan and, where results exist, reproducible visuals. Use coherent semantic colour: solar/direct = amber/gold; conventional baseline = slate/deep blue; mushroom/faceted candidate = teal/green; diffuse sky = cyan; losses/wind = red/orange; uncertainty = grey; competing geometry = violet where useful. Use colour-blind-aware combinations, adequate contrast and greyscale-redundant line styles/markers. Avoid rainbow/jet maps and decorative distortion.

Required visuals span concept comparisons, geometry cross-sections/renders, Singapore solar paths, packing curves, shadow/irradiance maps, bifacial/thermal maps, mechanical free-body and wind diagrams, topology evolution/Pareto fronts, techno-economic sensitivity and final system architecture. Quantitative figures must come from code/data; conceptual figures must be labelled conceptual. Captions must state assumptions/data provenance and whether results are analytical, simulated, measured or conceptual.

## 8. Evidence and provenance
Separate verified external facts, measured/source-derived data, assumptions, exploratory calculations, model-generated results and hypotheses. Use authoritative Singapore sources and primary technical literature where possible. Do not redistribute data without rights.

## 9. Reproducibility
Code must regenerate model-generated tables and figures. Record assumptions, constants, provenance and units in code/configuration. Add sanity tests. Do not hand-edit numerical results without traceable calculations.

## 10. Required baselines
Compare horizontal fixed, optimised fixed tilt, east-west/folded, vertical bifacial where relevant, single-axis, dual-axis, conventional canopy, sphere/hemisphere, cone, paraboloidal mushroom, faceted/sparse canopy and eventually free-form topology optimisation under equal resource constraints.

## 11. Core optimisation philosophy
Distinguish PV-material efficiency from footprint efficiency. A central metric is

$$
M_L=\Pi\eta_{\mathrm{pack}},
$$

where $M_L$ is the land multiplication factor (dimensionless), $\Pi$ is the PV packing ratio (dimensionless), and $\eta_{\mathrm{pack}}$ is the average productivity of packed PV relative to the chosen baseline (dimensionless).

The packing ratio is

$$
\Pi=\frac{A_{\mathrm{PV}}}{A_{\mathrm{land}}},
$$

where $A_{\mathrm{PV}}$ is total active PV surface area (m$^2$) and $A_{\mathrm{land}}$ is the constrained horizontal footprint/site area (m$^2$).

Three-dimensional geometry is valuable only if packing gains outweigh orientation, self-shading, sky-view, temperature, structural, cost and maintenance penalties. Establish the best fixed geometry before allowing tracking.

## 12. Mechanical interpretation
Do not interpret the original momentum question as a requirement to maximise angular momentum. Solar tracking is slow. Optimise low inertia, counterbalancing, friction, actuator energy, aerodynamic centre, wind torque, locking and storm stow. Preserve the original question because it motivated the mechanical analysis.

## 13. GitHub workflow and permission assumption
Use this repository proactively whenever GitHub access is available. The user has requested normal continuing project work be pushed without repeated permission prompts. This does not override platform authentication or security controls. Use descriptive files and meaningful commits; keep foundation records distinct from validated conclusions.

## 14. Public-repository discipline
The repository is public. Never commit credentials, private personal information, restricted datasets, proprietary material or content without redistribution rights.

## 15. Quality gate
Before a milestone is complete: verify equations/units/constants; run tests; regenerate figures/tables; compile LaTeX/PDF; visually inspect pages/math/figures; verify references; label assumptions/limitations; and ensure foundation intent has not been lost.
