# Three-Pass Master-Instruction Audit — 18 September 2026

**Trigger:** mandatory Pass 3 checkpoint after the full audit. This audit was performed before any new modelling layer, as required by the persistent project instructions.

## Governance test result

PASS. This `Continue` did not blindly advance the model. The master requirements were consulted first, including the source requirements for uncertainty/sensitivity, traceability, professional Markdown/LaTeX/PDF deliverables and page-by-page PDF QA. The repository was then inspected against the foundation-first control rules.

## Audit findings

| Audit area | Status | Evidence / deficiency |
|---|---|---|
| Foundation alignment | PASS | Founding Singapore land constraint -> mushroom/sphere -> 3-D geometry -> packing -> shadowing/bifacial -> mechanics -> topology lineage remains explicit. |
| Mathematical correctness/rendering | PARTIAL | Paraboloid derivation has dimensional/limiting checks. Other layers remain preliminary. Markdown source renders math conventionally; final cross-format visual verification remains outstanding. |
| Immediate variable definitions and units | PARTIAL | `equations/derivations.md` was repaired. Main Markdown generally complies. LaTeX still has equations whose explanatory text is less complete than the Markdown derivation standard. |
| Numerical-constant provenance | PARTIAL | Cooper constants and canonical 1/2/2 design cell are categorised. A complete typed register for all future physical/model/numerical constants is not yet present. |
| Source/data provenance | PARTIAL | EMA/SERIS/NREL basis exists. Time-resolved Singapore irradiance acquisition/licensing/preprocessing remains unresolved. The provisional 57% diffuse fraction remains quarantined. |
| Rust reproducibility/tests | PARTIAL | Canonical Rust modules exist for geometry, preliminary solar geometry, facets/direct incidence and equal-resource contracts, with analytical sanity tests. The connected GitHub integration cannot itself execute `cargo test`; execution evidence must be produced in an execution-capable environment before claiming tests passed. |
| Equation-code-evidence traceability | PARTIAL/PASS foundation | Matrix exists and correctly marks missing layers. It needs extension as each new layer is implemented. |
| Visualisation coverage | FAIL/PARTIAL | Conceptual SVG figures are referenced, but no validated quantitative plot suite or convergence figures exist. |
| Equal-resource fairness | PARTIAL | Common resource contract exists; actual candidate meshes and simulations have not yet demonstrated equal-resource numerical comparisons. |
| Uncertainty/sensitivity | FAIL | No systematic uncertainty propagation or sensitivity suite exists. This blocks validated optimisation conclusions. |
| Markdown/LaTeX parity | FAIL/PARTIAL | Markdown is materially more complete than LaTeX. This is a current foundation defect. |
| PDF compilation/visual QA | FAIL | A prior PDF exists in project sources, but the current revised repository report has not been demonstrated to be recompiled and page-by-page inspected after recent changes. |
| Public repository safety/licensing | PARTIAL/PASS | `.gitignore` covers secrets and Rust target. Cargo remains UNLICENSED. Third-party data/figure licensing still needs an explicit register. |
| Exploratory vs validated status | PASS with one correction required | Technical report now states no validated Singapore performance result. README still contains wording such as 'Current findings' that can read more strongly than the evidence; convert these to hypotheses/analytical observations. |
| Unresolved technical debt | HIGH | Coordinate/sign convention not yet locked across report/LaTeX/Rust; LaTeX parity, parameter register, Rust execution evidence, data workflow, visuals, uncertainty and PDF QA remain major gates. |

## Material corrections required before model expansion

1. Replace potentially over-strong README `Current findings` language with an evidence-status table separating analytical observations, hypotheses and unresolved questions.
2. Create one canonical coordinate/sign convention document and mirror it in Rust comments and both reports.
3. Bring LaTeX into equation/definition/status parity with the Markdown foundation before adding ray tracing.
4. Create a typed parameter/provenance register covering symbol, value/range, units, status category, source/basis and sensitivity requirement.
5. Obtain actual Rust compile/test evidence in an execution-capable environment; do not infer passing tests from source inspection.
6. Treat the existing PDF as stale relative to current repository changes until it is recompiled and visually inspected.

## Decision

**Do not advance to new physics or optimisation.** Continue foundation repair. The next pass should address evidence-status language and coordinate/sign conventions first.