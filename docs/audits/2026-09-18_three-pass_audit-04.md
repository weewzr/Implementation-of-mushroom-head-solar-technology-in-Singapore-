# Three-Pass Master-Instruction Audit 04 — 18 September 2026

**Trigger:** mandatory Pass 3 checkpoint after two foundation passes focused on LaTeX derivation parity and solar-position bibliography/provenance. No new modelling layer was introduced before this audit.

## Executive result

Foundation quality improved again. The recurring ENU Markdown display-math defect is now verified fixed in the repository. Land-multiplication and slow-tracking derivations have been expanded in LaTeX, and solar-position provenance is stronger without fabricating an unverified Cooper primary-source entry. The project nevertheless remains **not ready for model expansion** because report parity, Rust execution evidence, data acquisition, uncertainty, quantitative figures and current PDF QA remain incomplete.

## Mandatory audit checks

| Area | Status | Finding |
|---|---|---|
| Foundation alignment | PASS | Research lineage and falsifiability remain explicit; no geometry is promoted as optimal. |
| Mathematical correctness/rendering | PARTIAL, improved | ENU display equations are now verified as proper GitHub `$$...$$` blocks. Paraboloid and packing derivations have useful dimensional/limiting checks. Full report rendering still requires compiled PDF QA. |
| Immediate definitions and units | PARTIAL, improved | Coordinate, solar, packing and mechanics layers are substantially better in both formats. Some LaTeX equations still use compact prose and need the full derivation-file standard. |
| Numerical-constant provenance | PASS/PARTIAL | Typed register and constants register classify current known values. Duffie & Beckman/pvlib/NREL provenance is stronger; original Cooper-paper metadata remains explicitly Requires verification rather than fabricated. |
| Source/data provenance | PARTIAL | Core contextual sources are recorded. Time-resolved Singapore GHI/DHI/DNI/weather acquisition, licence and preprocessing are unresolved. |
| Rust reproducibility and tests | PARTIAL | Canonical Rust source and source-level tests exist, but there is still no recorded execution evidence from `cargo test`. ENU vector constructors/sanity tests remain to be implemented. |
| Equation-code-evidence traceability | PARTIAL, improved | Packing/mechanics now link Markdown, LaTeX and derivations. Some future/missing layers remain correctly marked missing. |
| Visualisation coverage | FAIL/PARTIAL | Conceptual figures exist, but quantitative, convergence, uncertainty and validation figures are absent. |
| Equal-resource fairness | PARTIAL | Common 1/2/2 experiment and candidate contract are explicit; no full candidate simulation comparison has been run. |
| Uncertainty/sensitivity | FAIL | Requirements are documented but no calculations exist. |
| Markdown/LaTeX parity | PARTIAL, improved | Land multiplication and slow-tracking derivations are closer to parity. Markdown still contains diffuse analytical limit, sphere baseline, tracking-control, bifacial/thermal and free-form optimisation material not equivalently developed in LaTeX. |
| PDF compilation and visual QA | FAIL | Current LaTeX has changed materially since the available PDF. No current compile/page-by-page inspection evidence exists. |
| Public repository safety/licensing | PARTIAL/PASS | No credential issue identified. Data/figure licensing register remains incomplete. |
| Exploratory vs validated status | PASS | Historical assumptions remain quarantined and no Singapore performance percentage is promoted. |
| Unresolved technical debt | HIGH | LaTeX parity, Rust execution/ENU API tests, time-series data workflow, bibliography completion, licensing register, quantitative figures, uncertainty/sensitivity and current PDF QA. |

## Audit correction discovered

The README repository-structure section still describes Python-era subdirectories (`src/models`, `src/analysis`, `src/optimisation`, `src/visualisation`) even though the canonical Rust crate currently uses top-level modules such as `src/geometry.rs`, `src/solar.rs`, `src/mesh.rs` and `src/candidates.rs`. This is documentation drift and must be corrected now.

## Decision

**Continue foundation repair. Model expansion remains blocked.** The next cycle should continue report/repository consistency and Rust reproducibility foundations.