# PDF and Mathematical Rendering QA Checklist

This checklist implements the master-instruction quality gate. A successful LaTeX compilation is necessary but not sufficient.

## Automated build gate
- LaTeX exits successfully with `-halt-on-error`.
- Cross-references resolve after `latexmk` convergence.
- `pdfinfo` recognizes the output.
- `pdftotext` returns non-empty text.
- The GitHub Actions PDF artifact is retained for inspection.

## Required human/visual gate
Render every PDF page to an image and inspect at readable zoom. Confirm:

### Mathematics
- fractions and radicals are complete;
- superscripts/subscripts are legible;
- Greek letters are correct;
- vector notation is visually consistent;
- equation numbers do not collide with equations;
- no equation crosses a margin;
- multiline equations use alignment rather than tiny type;
- units render correctly (`siunitx` output included);
- minus signs, multiplication, inequalities and derivative notation are unambiguous.

### Equation explanations
- every important displayed equation is immediately followed by definitions of all introduced variables;
- every variable has units or is explicitly labelled dimensionless;
- non-trivial numerical constants are justified and classified;
- sourced constants have citations/provenance;
- design assumptions are not written as physical constants.

### Figures and colour
- figures remain legible at final page size;
- semantic project colours are consistent;
- colour is not the only differentiator;
- legends/captions do not overlap plots;
- captions state whether a figure is conceptual, analytical, simulated or measured.

### Page design
- no clipped content;
- no orphaned heading with no following text;
- tables fit within margins;
- whitespace and hierarchy are visually balanced;
- references and hyperlinks render correctly.

## Shipping rule
Do not label a PDF `QA-passed` until the visual gate has been performed on rendered page images. If the environment cannot retrieve/render the GitHub Actions artifact, record the build as `automated-build pending/complete` separately from `visual-QA pending` rather than implying the latter was completed.
