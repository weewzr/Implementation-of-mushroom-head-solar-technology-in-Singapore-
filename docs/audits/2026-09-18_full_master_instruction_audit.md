# Full Master-Instruction Audit — 18 September 2026

**Trigger:** user identified that the mandatory three-turn audit had been missed. This audit therefore supersedes the narrower continuation passes and pauses major new modelling until governance defects are corrected.

## Executive finding

The repository has useful analytical foundations, but it is **not compliant enough to continue claiming milestone progress under the master instruction**. The largest problems are: (1) the three-turn audit cadence was not reliably enforced; (2) the Markdown, LaTeX, equations and Rust implementation are not yet mutually complete; (3) `equations/derivations.md` violates the immediate symbol/unit-definition rule; (4) the README previously described Rust as canonical while legacy Python references remain in report/data text; (5) no final compiled and visually inspected PDF exists; (6) quantitative visualisation, uncertainty/sensitivity, full equal-resource baselines, validated time-resolved Singapore weather, shadowing, bifacial/thermal/electrical/mechanical layers and lifecycle optimisation are incomplete.

## 46-requirement compliance matrix

| # | Requirement | Status | Corrective action |
|---:|---|---|---|
1|Research-session reconstruction|PARTIAL|Foundation records exist, but create a consolidated reconstruction ledger linking notebook ideas, corrections, unresolved questions and repository artifacts.|
2|Slow cumulative exposition|PARTIAL|Markdown is better than LaTeX; enforce Problem→Physics→Assumptions→Derivation→Computation→Interpretation throughout.|
3|Technology before modelling|PARTIAL|Add a newcomer system-overview section and energy-flow/component diagram before advanced equations.|
4|Professional structure|PARTIAL|Expand both report formats to the full justified engineering structure and keep them in parity.|
5|Professional mathematics|PARTIAL|Main Markdown often complies; derivations and parts of LaTeX do not immediately define every symbol/unit.|
6|Derive rather than state|PARTIAL|Paraboloid is derived; solar, irradiance, mechanics, thermal, bifacial and optimisation layers require fuller first-principles derivations/status labels.|
7|Show workings|PARTIAL|Paraboloid workings exist; other important equations need assumptions, conditions, substitution, units and plausibility checks.|
8|Dimensional/sanity checks|PARTIAL|Some checks exist; create systematic tests/checks for every model layer.|
9|Physical interpretation + engineering implication|PARTIAL|Not consistently present after every major mathematical development.|
10|Technical figures|PARTIAL|Conceptual figures exist, but required coordinate, free-body, workflow, solar-path, shadowing and optimisation figures remain incomplete.|
11|External figures|PASS/PENDING|Original schematics are preferred; future external material must retain attribution/licensing.|
12|Graphs/data visualisation|FAIL|No complete reproducible quantitative plot suite tied to validated inputs exists.|
13|Datasets/parameter tables|PARTIAL|Provenance notes exist; consolidated typed parameter table remains incomplete.|
14|References/evidence|PARTIAL|EMA/SERIS/NREL sources exist; Cooper/PV modelling/shading/bifacial/thermal/wind/optimisation literature coverage is incomplete.|
15|Knowledge vs assumption|PARTIAL|Good warnings exist, but every results table/figure must carry explicit status labels.|
16|Engineering rigour|PARTIAL|Optical/geometry foundations exist; structural, thermal, wind, electrical mismatch, reliability, cost and safety are incomplete.|
17|Optimisation formulation|PARTIAL|Variables/objective are introduced; explicit constraint functions, multi-objective terms and solver formulation remain incomplete.|
18|Computational methods|PARTIAL|Rust modules/tests started; mesh, timestep, solver, convergence and stopping criteria are not fully documented/implemented.|
19|Uncertainty/sensitivity|FAIL|No systematic uncertainty propagation or sensitivity suite yet.|
20|Alternative designs|PARTIAL|Candidate family documented; fair numerical comparison not yet complete.|
21|Professional style|PARTIAL|Generally technical, but repository contains historical/reconstruction language that needs final editorial harmonisation.|
22|Traceability|PARTIAL|Need an explicit equation→Rust function→dataset→figure→conclusion traceability table.|
23|Appendices|FAIL/PARTIAL|Supporting material exists as separate files but final report appendices are incomplete.|
24|Complete Markdown report|PARTIAL|Substantial but not complete under the master structure or validated-results standard.|
25|Complete compilable LaTeX|FAIL/PARTIAL|LaTeX is materially shorter than Markdown and has not been demonstrated as final compiled parity.|
26|Professional PDF|FAIL|Final PDF deliverable is not established.|
27|PDF page-by-page QA|FAIL|Cannot pass until the full LaTeX report is compiled and every page inspected.|
28|Reproducible modular computation|PARTIAL|Rust crate exists; legacy Python references/code are migration debt; full analysis pipeline not yet Rust.|
29|Equations connected to code|FAIL/PARTIAL|Some comments exist, but systematic cross-references are absent.|
30|Figure reproducibility|FAIL/PARTIAL|Conceptual SVGs may exist; quantitative source→Rust→data→figure pipeline is incomplete.|
31|Data provenance|PARTIAL|Data README is useful; time-resolved Singapore dataset acquisition/licensing and preprocessing are unresolved.|
32|Dedicated GitHub repository|PASS|Dedicated public repository is canonical; public status reflects later project-specific instruction.|
33|Repository structure|PARTIAL|Core directories exist; inspect/migrate legacy Python and avoid stale paths.|
34|Professional README|PARTIAL|Good overview, but reproduction/status sections must remain synchronized with actual Rust capability and report deliverables.|
35|BibTeX references|PARTIAL|Valid core entries exist; bibliography does not yet cover all report claims/models.|
36|Version-control hygiene|PARTIAL|`.gitignore` protects common secrets/build artifacts but should add Rust `target/`; repository-wide secret/restricted-material review remains required.|
37|Licensing|PARTIAL|Cargo says UNLICENSED; third-party data/figure/code licensing register is still required.|
38|Validate before pushing|FAIL/PARTIAL|Writes have occurred without a complete local compile/test/render gate because GitHub integration alone cannot execute the repository.|
39|Use GitHub integration|PASS|Repository is being updated directly through connected GitHub tools.|
40|Meaningful commits|PASS|Recent commits are substantive and descriptively named.|
41|Verify repository after push|PARTIAL|Files were re-fetched, but complete link/PDF/figure/source/data verification remains outstanding.|
42|Final technical review|FAIL|Project is not at final-review stage; this audit establishes the backlog.|
43|Do not over-compress|PARTIAL|The restored master file addresses this, but report sections still need expansion and derivational parity.|
44|Final deliverables|FAIL/PARTIAL|Several source deliverables exist; PDF, complete figures/plots, validated data pipeline and full computational model are incomplete.|
45|Project handover|NOT DUE|Only valid after the quality gates pass.|
46|Ultimate quality standard|NOT YET|Repository is a serious work-in-progress, not yet professor/panel handover quality.|

## Mandatory audit checklist from the operational master rule

- **Foundation alignment:** PASS. Mushroom-head, sphere, topology optimisation and rotation/momentum lineage remains explicit.
- **Mathematical rendering:** PARTIAL. Main Markdown is stronger than derivations/LaTeX.
- **Numerical constants:** PARTIAL. Canonical 1/2/2 resource assumptions and Cooper constants are labelled; all future numerical settings require provenance/convergence.
- **Variable definitions/units:** FAIL in `equations/derivations.md`; correct before adding major new equations.
- **Sources/provenance:** PARTIAL. EMA confirms ~1,580 kWh m^-2 yr^-1 and 3 GWp by 2030; SERIS confirms 25 GHI stations, 10 full meteorological stations and 1-second resolution; NREL SPA is the intended solar-position validation basis.
- **Reproducibility:** PARTIAL. Rust foundation exists; no complete validated end-to-end pipeline.
- **Visualisation coverage:** FAIL/PARTIAL. Required quantitative visuals are not yet generated from validated Rust outputs.
- **Equal-resource baselines:** PARTIAL. Contract exists; actual fair simulations are incomplete.
- **Public-repository safety:** PASS with ongoing licensing/data review.
- **Exploratory-result quarantine:** MUST BE ENFORCED. Any earlier `R_beam` or percentage gain is geometry-only/exploratory unless reproduced in the current canonical Rust path and validated; it must not be described as annual electricity performance.

## Corrective priority before major optimisation

1. Repair governance: add an audit ledger/checkpoint file and never allow more than three project turns without an audit.
2. Repair `equations/derivations.md` to the immediate where/units/provenance standard.
3. Add Rust `target/` to `.gitignore`; inventory and quarantine/migrate Python remnants.
4. Create a report/code traceability matrix.
5. Bring LaTeX into parity with Markdown and compile it in an execution-capable environment.
6. Obtain/define a legally usable time-resolved Singapore irradiance workflow and validate solar position against NREL SPA.
7. Only then run equal-resource baseline meshes, convergence, shadowing/diffuse/bifacial/thermal/electrical layers.
8. Add uncertainty/sensitivity and mechanics/wind/lifecycle objectives before declaring an optimum.
9. Generate reproducible figures and complete PDF page-by-page QA.

## Audit cadence correction

The missed cadence is itself a process defect. From this audit onward, `docs/audits/AUDIT_LEDGER.md` is the explicit checkpoint. A substantive audit is required at least every three project turns, and earlier whenever a major modelling layer or claimed result is introduced.