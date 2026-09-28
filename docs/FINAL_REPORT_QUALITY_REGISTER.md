# Final Report Quality Register — initiated Continue #193

## Status
**TECHNICAL PROGRAMME: ON TRACK.**
**FINAL REPORT / PUBLICATION QUALITY: AT RISK — RECOVERABLE.**
**Overall ~#227 target: achievable only with continuous report repair and no major unplanned model branches.**

A successful LaTeX build is necessary but is not evidence that the scientific paper is publication-quality.

## Quality scale
Prose: CLEAR / NEEDS EDIT / MAJOR REWRITE.
Mathematics: CLEAN / NEEDS TYPESETTING / NEEDS DERIVATION REWRITE.
Figures: COMPLETE / NEEDS REVISION / MISSING.
Tables: COMPLETE / NEEDS REVISION / MISSING.
Citations: ADEQUATE / INCOMPLETE / MISSING.
Technical: VALIDATED / DEVELOPMENT_NOT_SERIS / FUTURE.

## Section register
| Final section | Prose | Mathematics | Figures | Tables | Citations | Technical | Exact defect / recovery action | Priority | Target |
|---|---|---|---|---|---|---|---|---|---|
| 1 Introduction / research question | CLEAR | CLEAN | COMPLETE | N/A | ADEQUATE | DEVELOPMENT_NOT_SERIS | Rewritten #195: land constraint, mushroom origin, fair-normalisation need and falsifiable question now explicit. | P2 | #213 full-pass |
| 2 Beginner-first explanation | CLEAR | CLEAN | COMPLETE | N/A | ADEQUATE | VALIDATED | Rewritten #195 as short physical chain and equal-land/equal-PV bridge; equation is explicitly conceptual. | P2 | #213 full-pass |
| 3 Singapore context | CLEAR | CLEAN | NEEDS REVISION | N/A | ADEQUATE | DEVELOPMENT_NOT_SERIS | Rewritten #195 with EMA solar-resource/land/deployment evidence and packing-ratio motivation. Dedicated Singapore visual remains optional P2 enhancement. | P2 | #213 full-pass |
| 4 Mushroom-head origin | CLEAR | CLEAN | COMPLETE | N/A | ADEQUATE | VALIDATED | Rewritten #195 to distinguish hypothesis, disadvantages and controlled candidate family. Concept SVG label overlap repaired by direct SVG text rendering and visually checked. | P2 | #213 full-pass |
| 5 Fair-comparison framework | NEEDS EDIT | NEEDS TYPESETTING | COMPLETE | NEEDS REVISION | INCOMPLETE | VALIDATED | Resource equations are correct but notation and explanatory bridge are fragmented | High | #195 |
| 6 Mathematical formulation | MAJOR REWRITE | NEEDS DERIVATION REWRITE | NEEDS REVISION | MISSING | INCOMPLETE | VALIDATED | Equation-heavy blocks lack consistent symbol definitions/derivation narrative; vectors/scalars inconsistent | Critical | #195–197 |
| 7 Analytical verification | NEEDS EDIT | NEEDS TYPESETTING | NEEDS REVISION | NEEDS REVISION | INCOMPLETE | VALIDATED | Verification logic needs compact benchmark table and equation references | High | #197 |
| 8 Numerical method | NEEDS EDIT | NEEDS TYPESETTING | NEEDS REVISION | MISSING | INCOMPLETE | VALIDATED | Mesh/visibility/sky integration needs diagram-led explanation and algorithm notation cleanup | High | #197 |
| 9 V&V / uncertainty | NEEDS EDIT | NEEDS TYPESETTING | COMPLETE | NEEDS REVISION | INCOMPLETE | VALIDATED | Convergence criteria dispersed; consolidate gates and uncertainty status | High | #197 |
| 10 Fixed candidates | NEEDS EDIT | CLEAN | COMPLETE | NEEDS REVISION | INCOMPLETE | VALIDATED | Candidate definitions need one comparison table and consistent naming | Medium | #196 |
| 11 Controlled fixed results | NEEDS EDIT | NEEDS TYPESETTING | COMPLETE | COMPLETE | INCOMPLETE | DEVELOPMENT_NOT_SERIS | Strong evidence but interpretation is fragmented; explain packing identity once, avoid notebook caveats | High | #197 |
| 12 Deployable/origami | MAJOR REWRITE | NEEDS DERIVATION REWRITE | NEEDS REVISION | MISSING | INCOMPLETE | VALIDATED | Kinematics notation is dense and under-explained; requires state diagrams + derivation flow | Critical | #195–197 |
| 13 Engineering implementation | MAJOR REWRITE | NEEDS TYPESETTING | MISSING | MISSING | MISSING | FUTURE | Mostly evidence-boundary text, not an engineering section yet | High | #198–202 |
| 14 Manufacturing | MAJOR REWRITE | CLEAN | MISSING | MISSING | MISSING | FUTURE | Placeholder/evidence-boundary only | High | #198–202 |
| 15 Electrical / net energy | NEEDS EDIT | NEEDS TYPESETTING | NEEDS REVISION | NEEDS REVISION | INCOMPLETE | DEVELOPMENT_NOT_SERIS | Technical chain exists; notation, parameter table, thermal/electrical boundary and adapter method need coherent final presentation | Critical | continuous #193–203 |
| 16 Cost / techno-economics | MAJOR REWRITE | NEEDS DERIVATION REWRITE | MISSING | MISSING | MISSING | FUTURE | Evidence-boundary only; no cost model yet | High | #198–207 |
| 17 Carbon / sustainability | MAJOR REWRITE | NEEDS DERIVATION REWRITE | MISSING | MISSING | MISSING | FUTURE | Evidence-boundary only; no lifecycle/carbon model yet | High | #198–207 |
| 18 Integrated comparison | MAJOR REWRITE | NEEDS DERIVATION REWRITE | MISSING | MISSING | MISSING | FUTURE | No integrated evidence layer yet | High | #208–212 |
| 19 Recommended concepts | MAJOR REWRITE | CLEAN | MISSING | MISSING | MISSING | FUTURE | Must remain scenario-specific; currently only boundary language | High | #208–212 |
| 20 Limitations / future work | NEEDS EDIT | CLEAN | MISSING | MISSING | INCOMPLETE | DEVELOPMENT_NOT_SERIS | Needs scientific limitation hierarchy, not audit history | Medium | #208–212 |
| 21 Conclusion | MAJOR REWRITE | CLEAN | MISSING | MISSING | INCOMPLETE | DEVELOPMENT_NOT_SERIS | Current Phase-I conclusion is not final-paper synthesis | High | #208–212 |
| Appendices / reproducibility | NEEDS EDIT | NEEDS TYPESETTING | NEEDS REVISION | NEEDS REVISION | INCOMPLETE | DEVELOPMENT_NOT_SERIS | Move hashes/governance here; standardize reproducibility tables and equation/code/test traceability | Medium | #213–222 |

## Mathematical typesetting defect classes found at #193 source audit
1. **Vector notation inconsistency — major.** Geometry uses bold vectors in some equations but plain symbols / prose descriptions elsewhere. Canonical policy required.
2. **Solar-angle notation inconsistency — major.** Zenith/azimuth/incidence symbols and degree/radian conventions are not introduced in one nomenclature location.
3. **Irradiance/energy symbol ambiguity — major.** Hourly POWER energy fields, interval-average W/m2 and annual Wh/m2 are described correctly in places but symbols/units are not consistently distinguished in equations.
4. **Area/resource notation fragmentation — major.** Active PV area, projected land area, packing ratio and normalized yields need one canonical symbol set and equation chain.
5. **Origami derivation density — major.** Single-crease/accordion equations are technically compact but not reader-led; assumptions and symbols are insufficiently staged.
6. **Thermal/electrical notation duplication — major.** T_m/T_module, eta/eta_m and power subscripts vary between prose, Rust-oriented text and equations.
7. **Equation cross-reference weakness — major.** Many displayed equations are unlabelled and referred to descriptively rather than with stable Eq. references.
8. **Unit typography inconsistency — major.** siunitx and inline ad-hoc W/m2, degC, Wh/m2 forms coexist.
9. **Raw implementation/audit vocabulary around equations — major readability defect.** Terms such as frozen, Audit-N, DEVELOPMENT_NOT_SERIS and commit evidence interrupt derivations in the main body.
10. **Nomenclature incompleteness — major.** Current table does not cover all resource, origami, thermal/electrical and future economics/carbon symbols.
11. **Long equation blocks without interpretation — major.** Several mathematical sections violate the plain-language -> diagram -> term -> equation progression.
12. **PDF-level equation readability not yet fully verified after latest source growth — major gate.** Page-by-page equation render QA is required; compilation alone is insufficient.

Major math/rendering defect count at initial audit: **12 classes**. This count tracks defect classes, not individual equation occurrences; the detailed page/equation list will expand during rendered-PDF inspection.

## Canonical mathematical style
- Scalars: italic Latin/Greek, e.g. $A$, $G$, $T$, $\eta$.
- Vectors: bold lowercase, e.g. $\mathbf{n}$, $\mathbf{s}$; no arrow notation.
- Matrices/tensors: bold uppercase.
- Unit symbols: upright through `siunitx`; do not embed ad-hoc unit text inside mathematical variable names.
- Named functions/operators: upright conventional operators.
- Dot product: centered dot, e.g. $\mathbf{n}\cdot\mathbf{s}$.
- Cross product: $\times$ only for vector cross products; multiplication otherwise implicit or centered dot where clarity requires.
- Differentials: upright $\mathrm{d}t$, $\mathrm{d}A$.
- Subscripts: roman for descriptive labels ($G_{\rm POA}$, $A_{\rm land}$, $P_{\rm AC}$); italic only for variable indices.
- Angles: radians in analytical equations unless explicitly marked; degrees only for reported/configuration values.
- Time index: subscript $t$ for discrete samples; annual sums explicitly include $\Delta t$.
- Temperatures: $T_a$ ambient, $T_m$ module, $T_{\rm ref}$ electrical reference, $T_{\rm NMOT}$ NMOT.
- Irradiance: $G_{\rm POA}$ total POA; component symbols must carry direct/diffuse/ground labels consistently.
- Resource accounting: $A_{\rm PV}$ active PV area; $A_{\rm land}$ projected land footprint; $\Pi=A_{\rm PV}/A_{\rm land}$; $\eta_{\rm pack}$ packing efficiency; $M_L=\Pi\eta_{\rm pack}$ land-energy multiplier.
- Electrical: $\eta_m$, $P_{\rm DC}$, $P_{\rm AC}$, $P_{\rm aux}$, $P_{\rm net}$, $E_{\rm net}$.
- Every displayed equation that is referenced later must have a LaTeX label and be referenced with `\eqref{}`.

## Plain-language -> diagram -> term -> equation rule
Mandatory for solar position, incidence, direct/diffuse/ground irradiance, visibility, sky view, resource normalization, packing, annual integration, origami kinematics, thermal model and electrical conversion. Each major derivation must state: physical question; assumptions; governing relation; substitutions; final expression; interpretation; limiting/verification case.

## Master nomenclature seed
| Symbol | Meaning | SI unit | First/final section | Notes |
|---|---|---|---|---|
| $\mathbf{s}$ | unit vector toward Sun | 1 | solar geometry | ENU frame |
| $\mathbf{n}$ | outward surface unit normal | 1 | incidence | bold-vector convention |
| $\theta_i$ | incidence angle | rad | incidence | $\cos\theta_i=\mathbf n\cdot\mathbf s$ |
| $G_{\rm DNI}$ | direct normal irradiance | W m^-2 | irradiance | weather input |
| $G_{\rm DHI}$ | diffuse horizontal irradiance | W m^-2 | irradiance | weather input |
| $G_{\rm GHI}$ | global horizontal irradiance | W m^-2 | irradiance | weather input |
| $G_{\rm POA}$ | total plane-of-array irradiance | W m^-2 | irradiance/electrical | interval-average for electrical adapter |
| $A_{\rm PV}$ | active PV area | m2 | resource accounting | equal-PV contract variable |
| $A_{\rm land}$ | projected land footprint | m2 | resource accounting | equal-land contract variable |
| $\Pi$ | packing ratio $A_{\rm PV}/A_{\rm land}$ | 1 | comparison | geometry/resource property |
| $\eta_{\rm pack}$ | PV-area productivity relative to flat reference | 1 | comparison | not electrical efficiency |
| $M_L$ | land-energy multiplier $\Pi\eta_{\rm pack}$ | 1 | comparison | irradiance-level DEVELOPMENT_NOT_SERIS |
| $T_a$ | ambient air temperature | degC | thermal | canonical T2M input |
| $T_m$ | module temperature | degC | thermal/electrical | NMOT baseline output |
| $T_{\rm NMOT}$ | nominal module operating temperature | degC | thermal | manufacturer property |
| $\eta_{\rm ref}$ | reference module efficiency | 1 | electrical | exact selected design-basis row |
| $\gamma_P$ | Pmax temperature coefficient | K^-1 | electrical | negative for selected module |
| $P_{\rm DC}$ | delivered DC power | W | electrical | distinguish ideal/delivered where needed |
| $P_{\rm AC}$ | generated AC power | W | electrical | nonnegative |
| $P_{\rm aux}$ | auxiliary demand | W | electrical | explicit scenario |
| $P_{\rm net}$ | net power $P_{\rm AC}-P_{\rm aux}$ | W | electrical | may be negative |
| $E_{\rm net}$ | integrated net energy | Wh | electrical | $\sum_tP_{{\rm net},t}\Delta t$ |
| $\lambda$ | deployable interpolation/deployment parameter | 1 | origami | must not conflict with wavelength/economic symbols |
| $C$ | future cost quantity | currency | economics | exact symbols to be fixed when model exists |
| $I_{\rm CO2e}$ | future lifecycle carbon intensity | kg CO2e / functional unit | carbon | provisional nomenclature only |

## Exit criteria
By #217: coherent prose in all 21 sections; every equation renders; every symbol defined; nomenclature consistent; figures/tables readable; citations present or flagged; no audit-notebook language in main body; no raw LaTeX/Unicode defects; completed results integrated; limitations separated from results.

By #222: two complete end-to-end scientific editing/visual passes completed.

#223–227: release-candidate QA only, not first-time rewriting.


## Audit 66 rendered-PDF defect localization
Current green descendant: commit `d4832ec0191710e6bb7c30d9ca3169d60aa3ec10`, run `36371446210`, artifact `10949153513`, 35 pages, exact PDF SHA-256 `dae6157b81802e9644f396dc6ca6bc3c0113b506c295a1449a81756d701c6900`.

Severity policy: P0 = build failure/unreadable or scientifically misleading content; P1 = major comprehension defect; P2 = readability/consistency; P3 = cosmetic.

P0 defects found and closed during Audit 66:
1. **P0 build failure:** generated fixed-comparison SVGs were absent in clean CI. Root cause at report line 776; fixed by generating reproducible publication figures before LaTeX.
2. **P0 scientific wording:** page 16 land-energy multiplier text described the Audit-59 irradiance metric as annual electrical energy. Corrected to incident POA energy and canonical (H_{\rm POA}) notation.
3. **P0 stale thermal status:** page 19 claimed thermal/module inputs remained unset despite validated NMOT/electrical foundations. Corrected to identify the subsection as a precursor and point to the current implemented layer.

**Remaining P0 count: 0.**

Actionable P1/P2 defects from the 35-page render:
- **P1, Sections 5–9, pages 11–16:** dense equation sequence and weak narrative staging. Repair: enforce physical-question -> diagram -> definitions -> equation -> interpretation -> limiting case; add stable equation labels. Target #195–196.
- **P1, Section 9, page 16:** terminology still needs global synchronization from legacy “land multiplication” to canonical “land-energy multiplier”; ensure (H_{\rm POA}), (Pi), (eta_{\rm pack}), (M_L) match publication table. Target #196.
- **P1, legacy thermal/bifacial material, page 19:** now scientifically corrected but structurally duplicated by the later electrical section. Repair: relocate bifacial formulation to future-extension subsection and remove duplicated thermal derivation. Target #199.
- **P1, origami/deployable derivations, pages 24–27 and 31–32:** notation-heavy text lacks a state/crease diagram adjacent to each derivation and variable definitions are dispersed. Target #197.
- **P1, controlled-comparison table, page 31:** readable only at small type; too many columns for the main narrative. Repair: split main scientific table from reproducibility columns or use landscape/appendix. Target #197.
- **P1, Section 32 electrical/thermal chain, pages 31–34:** current material mixes historical decisions, validation status and governing equations. Repair into one scientific sequence with parameter table and move audit chronology to appendix. Target #199.
- **P1, report spine:** current compiled document has more than the intended 21 scientific sections because historical foundation/evidence-boundary sections remain first-class sections. Repair mapping/merging without changing frozen results. Target staged #195–207.
- **P1, citations:** multiple foundational equations and Singapore-context claims lack final authoritative citations in-place. Target continuous, hard gate #217.
- **P2, reproducibility prose:** long commit/artifact hashes create overfull lines (build log reports up to 273 pt overflow around source lines 756–766 and 227 pt around 921–922). Move full hashes to appendix/manifest or break them safely. Target #195.
- **P2, unit typography:** legacy inline W/m2/degC/Wh forms remain alongside siunitx. Target #203–207 consistency sweep.
- **P2, figure/table caption style:** status/provenance wording is verbose and sometimes dominates scientific interpretation. Target #204–207.
- **P2, section transitions:** beginner-first pages transition abruptly into equation-dense material. Target #195–197.

Remaining **P1 defect classes: 8** (derivation staging; terminology/spine consistency; duplicated thermal narrative; origami exposition; table readability; electrical-section scientific flow; citation completeness; final-section coherence). P2 defects are tracked separately and are not release blockers until their scheduled checkpoints.

## Audit 66 severity checkpoint
Current master report compile P0 is CLOSED. First failures were (1) missing generated `fixed_equal_land.svg` in report CI and (2) literal `\\n` tokens at the land-energy equation around source line 333. Green 35-page evidence: `d4832ec0`, run `36371446210`, artifact `10949153513`, PDF SHA-256 `dae6157b81802e9644f396dc6ca6bc3c0113b506c295a1449a81756d701c6900`. All pages rendered for audit.

**Remaining P0 defects: 0. Remaining P1 defect classes: 11.** P1 classes 1-11 from the initial register remain. Class 12 (lack of rendered-PDF inspection) is closed as a gate, although page-level readability improvements remain P1/P2 work.

Actionable P1 sequence: Sections 1-4 #195 (context/hypothesis/transition/notation); Sections 5-8 #196 (resource chain, mathematical derivations, verification and numerical-method diagrams); Sections 9-12 #197 (V&V/results/origami derivation); Sections 13-14 #198; Sections 15-16 #199; Sections 17-21 #201-203; whole-paper P1 closure #204-207. Exact page numbers may move as repairs change pagination, so source section/equation labels are the stable defect locator until release pagination freezes.

## Continue #195 Sections 1-4 recovery checkpoint
LaTeX and Markdown front sections were rewritten as a scientific narrative rather than notebook layers. Rendered PDF pages 5-7 were inspected: prose is readable, equations are conventional and the Section-4 concept figure is now label-contained. Bibliography processing was activated after the first rendered rewrite exposed unresolved citation markers. Sections 1-4 no longer contain P1 prose/math defects; remaining issues there are P2 final-pass/citation/visual polish. The project-wide P1 classes remain concentrated in Sections 5-21 and are scheduled under the #196-207 recovery plan.
