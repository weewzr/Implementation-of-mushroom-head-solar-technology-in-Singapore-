# LaTeX Compile and PDF Visual-QA Protocol

**Purpose:** define the evidence required before the technical-report PDF can be treated as a current project deliverable. Source-level inspection is not a substitute for compilation and rendered-page inspection.

## 1. Source lock

Before compiling, record:

- repository and branch;
- exact Git commit SHA;
- main source: `report/project_technical_report.tex`;
- bibliography and figure dependencies used;
- whether the working tree is clean.

Do not silently compile a locally modified source while citing a different repository commit.

## 2. Compile gate

The report uses the `svg` package and `\includesvg`, so the compilation environment must support the required SVG conversion path. Record the TeX engine/distribution and relevant version information.

A valid compile record must preserve:

1. command(s) used;
2. exit status;
3. warnings and errors;
4. unresolved references/citations;
5. missing figures/fonts/packages;
6. output PDF checksum and page count.

Compilation is **FAIL** if the PDF is produced only after ignoring a material error, if figures are missing, or if equations/references are silently dropped.

## 3. Page-by-page visual inspection

Inspect every rendered page, not only the source. Record page-level PASS/FAIL for:

- clipped or overflowing text;
- malformed equations or missing symbols;
- equation numbering/reference correctness;
- figure visibility and legibility;
- caption placement;
- tables crossing margins;
- headings/orphaned lines and excessive whitespace;
- broken Unicode/special characters;
- inconsistent fonts or sizing;
- bibliography/citation rendering;
- page breaks that separate an equation/figure from essential explanation.

Any material defect must be corrected in source and the complete compile/inspection repeated.

## 4. Mathematical spot checks

At minimum visually verify the rendered forms of:

- ENU Sun vector and facet normal;
- Cooper declination and solar elevation;
- direct/diffuse incidence;
- paraboloid area and packing ratio;
- diffuse-light analytical limit;
- land-multiplication identity and stationary condition;
- mechanics torque/drag relations;
- equal-resource constraints.

These checks are especially important because Audit 07 caught source corruption in the diffuse section before a formal compile.

## 5. Evidence record

Store a dated QA record under `docs/audits/` containing:

- validated commit SHA;
- compile environment and commands;
- compile result;
- PDF checksum/page count;
- page-by-page inspection table;
- defects found and corrective commits;
- final disposition: PASS, PARTIAL or FAIL.

A PDF may be called **current compiled report** only when its source commit and QA record are identified.


## CI compile-evidence path

The repository includes `.github/workflows/latex-compile-evidence.yml` to compile the report at an exact commit and retain the build log, toolchain identity, exit status, PDF checksum/page count and generated PDF as CI evidence.

**Compile evidence is not visual QA.** A successful CI compile can close only the source-to-PDF compilation gate for the exact commit it records. The PDF must still be rendered and inspected page-by-page under Sections 3–5 before it can be called visually QA-passed. Workflow presence alone is not evidence that compilation succeeded.

## Current status — 18 September 2026

**FAIL / not yet performed.** The LaTeX source has changed materially since the available PDF. The diffuse-section source defect found during Audit 07 has been corrected at source level, but no current compile and page-by-page rendered inspection has yet been recorded.

As of project Continue #44, no observable workflow run was found for the newly added LaTeX compile-evidence workflow on the inspected commits. Therefore the CI workflow has **not** yet closed the compilation gate.

**Continue #115 observability check.** A direct commit-scoped workflow lookup was performed for LaTeX parity commit `121361d99f7868a092e8cc3ae7733ed3a1ceef71`; the connected GitHub surface returned an empty `workflow_runs` list. The compile-evidence workflow is present on `main` and is configured for report-path pushes, but workflow definition plus an empty run lookup is **not** qualifying compile evidence. The compilation and page-by-page visual-QA gates therefore remain open. This negative lookup is recorded to prevent later work from silently treating CI presence as execution.
