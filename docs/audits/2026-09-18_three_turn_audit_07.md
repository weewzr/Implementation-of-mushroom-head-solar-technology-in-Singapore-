# Three-Turn Master-Instruction Audit 07 — 2026-09-18

## Trigger and correction
This audit is being performed after the user correctly identified that the three-turn checkpoint had been reached. Expansion stops here until the audit is completed. The master instructions were re-read from `docs/MASTER_INSTRUCTIONS.md`.

## Foundation alignment — PASS
The project remains about land-constrained Singapore, originating from the mushroom-head rotating PV and sphere comparison, with topology optimisation and the mechanics/momentum question preserved. The investigation remains falsifiable and does not assume the mushroom wins.

## Mathematical rendering — PARTIAL / CORRECTION REQUIRED
Recent explanatory mathematics improved equation-level symbol definitions, but the Rust kernel documentation does not yet satisfy the same equation-by-equation exposition standard as the Markdown/LaTeX reports. Future canonical mathematical documentation must define every introduced symbol and unit immediately after important equations. Rust comments should reference the corresponding documented equations rather than become the sole mathematical specification.

## Numerical constants — FAILING ITEMS IDENTIFIED
The existing Rust kernel contains several constants/defaults that require classification and justification before validated use:
- `EPS = 1e-9`: category 5 numerical tolerance; requires scale/sensitivity justification.
- `RAY_OFFSET_MULTIPLIER = 100`: category 5 computational setting; requires ray self-intersection sensitivity study.
- small-k switch `1e-3`, inverse tolerance `1e-12`, bisection upper bound `16`, and `100` iterations: category 5 settings requiring documented rationale/convergence consequences.
- mesh test `n_r=16`, `n_phi=64`, tolerance `0.02`: category 5 test settings; currently illustrative, not a validation threshold.
- Cooper declination coefficients `23.45 deg`, `360 deg`, `365`, phase shift `284`, and equinox day example `81`: category 2 astronomical approximation coefficients. They require authoritative provenance and must remain preliminary. A higher-accuracy traceable solar-position method is required for validated Singapore results.
- `15 deg/h` and `12 h` in preliminary hour-angle construction must be documented: 15 deg/h derives from 360 deg / 24 h (definition/ideal solar-time conversion); 12 h denotes apparent solar noon.

No numerical output depending materially on these settings is to be labelled validated until this provenance and sensitivity work is completed.

## Variables and units — PARTIAL PASS
The Rust coordinate convention is explicit: x east, y north, z up; geometry coordinates are metres and unit vectors dimensionless. Project documentation must additionally keep `h=H/R`, packing ratio, effective area, solar angles and land-normalized quantities consistently defined with units immediately after equations.

## Source/provenance — CORRECTION REQUIRED
The preliminary Cooper solar model currently lacks an authoritative citation in code/provenance documentation. Singapore weather values/data must come from traceable authoritative sources such as EMA/SERIS or appropriately documented meteorological datasets, with redistribution rights checked before committing raw data.

## Reproducibility — PARTIAL PASS
The repository has a Rust crate, Rust tests and GitHub Actions infrastructure. The user has now specified Rust rather than Python for computational work. Python files added in the immediately preceding work were removed. New modelling, numerical analysis and dataset generation should therefore be Rust-native. Existing legacy Python files elsewhere in the repository are not automatically deleted because they predate this instruction and may document earlier work; they should not be used for new computational results unless the user later changes this requirement.

## Visualisation coverage — CORRECTION REQUIRED
The deleted Python plotting script means the fixed-paraboloid visual package still needs a Rust-native generation path. Required next visuals remain: geometry cross-sections, directional response C(theta_z,h), absolute gain Delta C, and convergence. Quantitative visuals must be code-generated and labelled analytical/simulated; colour semantics and greyscale redundancy remain mandatory.

## Equal-resource baselines — NOT YET SATISFIED
Current development is still concentrated on the paraboloidal mushroom and horizontal reference. This is acceptable only as solver validation, not as a technology comparison. Before conclusions, compare required fixed geometries under both equal-land and equal-active-PV/installed-resource constraints. Establish the best fixed geometry before introducing tracking.

## Public-repository safety — PASS
No credentials, private personal data, restricted datasets or proprietary material were introduced. Continue checking data redistribution rights before committing measured datasets.

## Exploratory versus validated — PASS WITH QUARANTINE
`R_beam ~= 1.29904` remains quarantined as exploratory. It is not a validated 29.9% energy gain. No generated directional-response values have yet been claimed as executed/verified Rust results.

## Mechanical interpretation — PASS
Tracking remains deferred until fixed geometry is established. Momentum is retained as historical motivation; engineering optimisation will target inertia, counterbalancing, friction, actuator energy, aerodynamic centre, wind torque, locking and storm stow.

## Required corrections before major expansion
1. Implement the semi-analytical directional solver and tests in Rust only.
2. Add a Rust-native numerical output/visualisation pipeline.
3. Document and sensitivity-test category-5 Rust constants.
4. Add authoritative provenance for any retained astronomical approximation, then replace preliminary solar positioning with a higher-accuracy validated method before Singapore yield claims.
5. Run Rust tests/CI and inspect logs before reporting model-generated values as verified.
6. Keep `R_beam ~= 1.29904` quarantined until its exact original normalization and convergence are reproduced.

**Audit status:** PASS WITH CORRECTIONS; modelling may continue only along the corrective sequence above.
