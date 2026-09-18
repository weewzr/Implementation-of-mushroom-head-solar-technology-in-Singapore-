# Technical Report PDF Visual-QA Checklist

This checklist is the retained inspection protocol for the compiled `report/project_technical_report.pdf`. It does **not** certify that the current report has passed QA. Complete it only against an actual compiled artifact.

## Build identity

- Source commit:
- Workflow/run or reproducible local build identifier:
- PDF artifact checksum:
- Build log location:
- Build engine/toolchain:
- Reviewer/date:

## Whole-document checks

- [ ] PDF opens without corruption.
- [ ] Page count recorded.
- [ ] No blank/unintended pages.
- [ ] No clipped text, equations, figures, captions or page numbers.
- [ ] No content extends beyond margins.
- [ ] Fonts and mathematical symbols render consistently.
- [ ] Hyperlinks/references do not visibly corrupt layout.
- [ ] No unresolved `??` references/citations.
- [ ] Section order and hierarchy match the source.
- [ ] Figure captions remain legible at normal page viewing scale.
- [ ] Figure numbering and in-text references agree.

## Required figure inspection

For each figure, record page number, legibility, clipping, scale, caption fit and whether the visual meaning matches the surrounding equations/text.

| Figure | Page | Scale/legibility | Clipping/overflow | Caption/reference | Status |
|---|---:|---|---|---|---|
| Equal-footprint comparison | | | | | OPEN |
| ENU coordinate convention | | | | | OPEN |
| Solar ray-tracing schematic | | | | | OPEN |
| Direct-incidence geometry | | | | | OPEN |
| Paraboloid geometry | | | | | OPEN |
| Mechanical free-body/tracking | | | | | OPEN |
| Validation-first method flow | | | | | OPEN |

## Equation inspection

- [ ] ENU Sun-vector equation is readable and not broken awkwardly.
- [ ] Facet-normal equation is readable.
- [ ] Direct/diffuse incidence equations and positive-part notation render correctly.
- [ ] Paraboloid area/packing derivation is visually coherent.
- [ ] Diffuse analytical-limit boxed result is not clipped.
- [ ] Land-multiplication equations and logarithmic stationary condition render correctly.
- [ ] Mechanics and tracking-control equations fit margins.
- [ ] Bifacial/thermal equations render correctly.
- [ ] SI units and degree symbols render consistently.

## Float and pagination inspection

- [ ] Figures appear near the section that introduces them.
- [ ] No figure separates a defining equation from its immediate explanation in a misleading way.
- [ ] No isolated heading at page bottom.
- [ ] No caption stranded from its figure.
- [ ] No excessive whitespace caused by float placement.
- [ ] Method-flow figure is readable without rotating/zooming excessively.

## Evidentiary-status inspection

- [ ] Conceptual diagrams remain labelled conceptual/non-validated where appropriate.
- [ ] Historical 57% diffuse share is not visually presented as validated input.
- [ ] Historical 23% efficiency is not visually presented as selected canonical module data.
- [ ] No diagram implies an optimum, validated load or validated annual-yield result.
- [ ] Open data/convergence/uncertainty gates remain visibly distinguishable from passed gates.

## Release criterion

The PDF may be marked **visually QA-passed** only after every applicable item above is checked against a retained compiled artifact and all material defects are corrected and rebuilt. Source inspection alone is insufficient.
