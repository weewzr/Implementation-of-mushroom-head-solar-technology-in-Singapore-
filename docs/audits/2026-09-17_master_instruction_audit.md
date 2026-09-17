# Master-Instruction Audit — 17 September 2026

## Reason for audit
The user requested an explicit audit and additionally required that such audits occur at least every three project turns. The user also identified an important deficiency: equations contained numerical constants and variables without immediate justification/units.

## Findings

### Foundation alignment — PASS
The original mushroom-head/sphere/topology/momentum question is preserved under `docs/foundation/` and remains the design lineage.

### Mathematical rendering — PARTIAL / CORRECTION REQUIRED
GitHub-compatible display mathematics is generally used, but several equations in reports/equation notes define symbols only implicitly or remotely. This does not meet the strengthened standard.

### Numerical constants — PARTIAL / CORRECTION REQUIRED
Examples requiring explicit provenance include the preliminary solar-declination approximation coefficients $23.45^\circ$, $365$, $284$, and the $15^\circ\,\mathrm{h}^{-1}$ hour-angle coefficient. Singapore latitude, provisional module efficiency, albedo/bifaciality defaults, mesh resolution, timestep sampling and canonical 1 m²/2 m²/2 m design constraints also require category labels and rationale/source.

### Variable definitions and units — PARTIAL / CORRECTION REQUIRED
A nomenclature exists, but the new rule requires a local `where` block after every important displayed equation. Existing report/equation files need retrofitting.

### Evidence/provenance — PASS WITH OPEN ITEMS
The project distinguishes exploratory outputs from validated results. The earlier 57% diffuse share remains provisional until its exact source/context is verified. External numerical inputs must continue to be sourced.

### Reproducibility — PASS FOR CURRENT STAGE
Analytical geometry, candidate geometry, mesh, irradiance and ray-tracing code are in the repository with sanity tests. Generated quantitative outputs must be regenerated before entering final reports.

### Visualisation — PASS FOR PLAN / PARTIAL FOR OUTPUTS
A project visual design system and phase-by-phase plan exist. Some visualization scripts exist; the complete required visual set will grow with each modelling phase.

### Equal-resource comparison — PASS
Packing ratio and equal-footprint/equal-PV constraints are explicit in current geometry comparison work.

### Public repository safety — PASS
No credentials or restricted raw datasets have intentionally been committed.

### Exploratory versus validated claims — PASS WITH WATCH ITEM
Current self-shadowing and solar-path computations are explicitly labelled geometric/simulated rather than annual Singapore yield.

## Mandatory corrective actions
1. Retrofit `report/project_technical_report.md` so every important equation is followed immediately by definitions and units.
2. Add a constants/provenance register classifying every numerical constant as exact conversion, astronomical approximation, sourced empirical value, engineering assumption or numerical setting.
3. Correct and justify the solar-position equations before relying on them for validated results.
4. Apply the same explanatory pattern to `equations/governing_equations.md` and derivation notes.
5. Mirror the corrected notation into LaTeX and final PDF.
6. Repeat this audit no later than three project turns after this audit.

## Audit conclusion
The project is directionally consistent with the master instructions but the mathematical exposition standard was not yet sufficiently strict. New modelling should pause until the equation/constant documentation is corrected.
