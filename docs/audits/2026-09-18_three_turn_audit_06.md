# Three-Turn Master-Instruction Audit 06 — 2026-09-18

## Scope
This audit follows the recent analytical development of directional response, paraboloidal mushroom geometry, array packing, and the preliminary Singapore solar-resource model. It is performed before further major modelling.

## Foundation alignment — PASS
The work remains anchored to Singapore land constraints, the originating mushroom-head and sphere questions, topology/geometry optimisation, and the later mechanical interpretation of rotation. The project remains falsifiable: no claim that the mushroom geometry wins has been validated.

## Mathematical rendering and equation-level definitions — CORRECTION REQUIRED
Recent conversational derivations are mathematically useful, but they do not consistently satisfy the repository rule that every important displayed equation be followed immediately by definitions and units for every introduced symbol. Before these derivations enter the canonical report, rewrite them to the equation-by-equation standard.

## Numerical constants — CORRECTION REQUIRED
Recent discussion introduced illustrative constants and thresholds (including example aspect ratios, mesh sizes, angular examples, approximate Singapore irradiation values, and a simplified solar-declination formula). These must not enter validated modelling without classification and provenance. In particular, the simplified declination coefficients 23.45 degrees, 365 days, and phase shift 284 are teaching approximations and should not be the validated solar-position method. The prior value R_beam ~= 1.29904 remains exploratory until its exact normalization, code path, convergence and baseline are verified.

## Variables and units — PARTIAL PASS
Core quantities are dimensionally coherent, including H/R, P/R, A_PV/A_land, effective projected area, irradiance and annual land-normalized yield. Canonical documentation must still define every symbol immediately after each important equation and preserve a single coordinate/sign convention.

## Source and provenance status — PARTIAL PASS
Singapore solar-resource discussion has identified authoritative-source candidates (EMA and SERIS), but source-derived values must be captured in the repository bibliography/provenance files before being treated as inputs. Restricted or non-redistributable measured datasets must not be committed to this public repository.

## Reproducibility — CORRECTION REQUIRED BEFORE NEW NUMERICAL CLAIMS
The semi-analytical directional solver, analytical unit tests, weather weighting, and array-shadow model must be implemented in reproducible code with tests. Numerical tables and plots must be generated from code rather than copied manually from chat derivations.

## Visualisation coverage — PLANNED, NOT YET COMPLETE
Next phase must generate reproducible: (1) C(theta_z,h) directional-response curves; (2) mushroom cross-sections for the same h values; (3) Singapore solar-path/irradiance weighting; (4) absolute directional gain Delta C rather than only ratios; and later (5) array shadow maps. Apply the repository semantic colour system and redundant line styles/markers.

## Equal-resource baselines — CORRECTION REQUIRED
Do not benchmark only against a horizontal plate. Maintain two explicit comparison families: equal land/site footprint and equal active PV area or installed capacity. Required baseline queue remains horizontal fixed, optimised fixed tilt, east-west/folded, vertical bifacial where relevant, single-axis, dual-axis, conventional canopy, sphere/hemisphere, cone, paraboloidal mushroom, faceted/sparse canopy, and later free-form topology optimisation. Establish the best fixed geometry before tracking.

## Public-repository safety — PASS
No credentials, personal information, restricted datasets, or proprietary content are to be committed. External data should be referenced with provenance and licensing/redistribution status.

## Exploratory-versus-validated conclusions — CORRECTION APPLIED
The value R_beam ~= 1.29904 is explicitly classified as exploratory, not an annual electrical-energy gain and not evidence that the mushroom wins. Illustrative analytical values are model examples, not measured Singapore performance. The hypothesis that weather weighting may reduce a uniform-angular advantage remains a hypothesis until computed from traceable data.

## Mechanical interpretation — PASS
Rotation remains a later-stage optimisation variable. The original momentum question is preserved, but the engineering objective is low inertia, counterbalancing, friction/actuator energy, aerodynamic centre, wind torque, locking and storm stow rather than maximising angular momentum.

## Gate before major expansion
1. Implement and test the semi-analytical fixed-geometry solver.
2. Verify the existing R_beam normalization and convergence before reusing it.
3. Establish a traceable validated solar-position method and Singapore weather-data provenance.
4. Generate code-derived directional-response and solar-weighting visuals.
5. Compare fixed geometries under equal-land and equal-PV-resource constraints before introducing tracking.

**Audit status:** PASS WITH REQUIRED CORRECTIONS. Major new numerical conclusions are gated on the five actions above.
