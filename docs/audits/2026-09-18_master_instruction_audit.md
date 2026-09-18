# Master-Instruction Compliance Audit — 18 September 2026

## Scope

This audit was triggered immediately after restoring the detailed 46-part master instruction. It checks the current repository against the canonical governance requirements rather than treating the earlier condensed instruction as sufficient.

## Findings

| Area | Status | Finding / required correction |
|---|---|---|
| Foundation alignment | PASS | Originating mushroom-head, sphere, topology-optimisation and rotation/momentum questions are preserved under `docs/foundation/`. |
| Full master governance | PASS | `docs/MASTER_INSTRUCTIONS.md` now explicitly maps all 46 requirements and states that summaries cannot replace them. |
| Mathematical rendering | PARTIAL | Markdown is substantially improved. LaTeX remains shorter than the Markdown report and requires a full equation-by-equation parity audit. |
| Variable definitions and units | PARTIAL | The main Markdown governing equations generally comply; `equations/derivations.md` still needs immediate definitions after every important displayed equation. |
| Numerical-constant provenance | PARTIAL | Several constants are categorised, but the repository still contains provisional/illustrative values that require source verification or removal from validated results. |
| Evidence / bibliography | PARTIAL | Singapore sources exist, but the bibliography is incomplete relative to citations mentioned in the report (notably Cooper/pvlib/Sandia and later required primary literature). |
| Reproducibility | FAIL -> remediation started | README previously pointed to Python and no `Cargo.toml` existed. A Rust crate is now being established as the canonical implementation. |
| Visualisation coverage | PARTIAL | Conceptual figures exist in the LaTeX work, but the master instruction requires substantially more quantitative and engineering visual coverage. |
| Equal-resource baselines | PARTIAL | Canonical constraints and baseline philosophy are documented, but the complete baseline suite has not yet been run under identical resources. |
| Optimisation | PARTIAL | Decision variables/objective are introduced; full constraint functions, solver, convergence criteria and Pareto/multi-objective treatment remain outstanding. |
| Uncertainty / sensitivity | FAIL | Required systematic uncertainty propagation and sensitivity analysis are not yet implemented. |
| PDF deliverable and page QA | FAIL | A final compiled, page-by-page visually inspected PDF is not yet present as a completed deliverable. |
| Public-repository safety | PASS with continuing review | Repository is public; no credentials should be committed. Third-party data redistribution rights remain a required check. |
| Scientific falsifiability | PASS | Repository explicitly states that mushroom geometry is a founding hypothesis, not a presumed winner. |

## Immediate corrective sequence

1. Establish and test the Rust crate, then migrate analytical and numerical modelling away from Python.
2. Bring the LaTeX report to content and equation-definition parity with the Markdown report.
3. Repair bibliography completeness and numerical provenance before using values in validated calculations.
4. Implement equal-resource geometry baselines in Rust.
5. Add time-resolved Singapore solar input acquisition/provenance and a validated solar-position implementation.
6. Add self-shadowing/sky-view, bifacial, thermal, mechanical/wind and lifecycle layers incrementally with tests.
7. Implement convergence, sensitivity and uncertainty analysis.
8. Generate reproducible figures from Rust outputs.
9. Compile the complete LaTeX report, render the PDF, inspect every page and iterate until clean.
10. Re-run the full 46-point technical review before milestone handover.

## Important status statement

The repository is **not yet complete** under the master instruction. Existing numerical percentages remain exploratory unless explicitly validated. This audit is a corrective checkpoint and not a declaration of completion.
