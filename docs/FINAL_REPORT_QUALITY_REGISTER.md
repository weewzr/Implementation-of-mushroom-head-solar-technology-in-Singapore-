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
| 1 Introduction / research question | NEEDS EDIT | CLEAN | NEEDS REVISION | MISSING | INCOMPLETE | DEVELOPMENT_NOT_SERIS | Too much project-status framing; sharpen Singapore problem, hypothesis and falsifiability | High | #195 |
| 2 Beginner-first explanation | NEEDS EDIT | CLEAN | COMPLETE | MISSING | INCOMPLETE | VALIDATED | Transition to engineering tone is abrupt; reduce repeated warnings | High | #195 |
| 3 Singapore context | MAJOR REWRITE | CLEAN | MISSING | MISSING | MISSING | DEVELOPMENT_NOT_SERIS | Context/evidence base too thin for final paper; requires authoritative Singapore land/solar sourcing and visual | High | #196 |
| 4 Mushroom-head origin | NEEDS EDIT | NEEDS TYPESETTING | COMPLETE | MISSING | INCOMPLETE | VALIDATED | Separate historical concept from candidate definition; define geometry symbols once | Medium | #196 |
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
