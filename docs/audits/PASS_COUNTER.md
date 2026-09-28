# Persistent Project Pass Counter

**Purpose:** make the mandatory audit cadence survive new ChatGPT sessions. This file is a project-control state record, not technical evidence.

## Current state

- Total explicit user `Continue` commands counted through 28 September 2026: **195**.
- Most recent audit: `docs/audits/2026-09-28_three-pass_audit-66.md`.
- That audit occurred on Continue #194.
- Passes since most recent audit: **1**.
- Audit 50 at Continue #154 correctly withheld release pending the completed canonical consumer refactor. Continue #155 reconciled stale governance state against post-audit evidence: commit `53d7d3420202ea81bf5b0a8d40b1ee87a93a68c4`, Rust run `36240614922` PASS, e2e run `36240614938` PASS, artifact `10905852093` SHA-256 `e450ccd79f303bd1a701293dbbc5613f156520c122a92ccbb6229b08e2eaf110`. Fresh artifact inspection closes exact-resource, convergence, crossover, sensitivity and stability gates; Markdown/LaTeX/traceability parity is synchronized. **Paraboloid geometric-response result is FROZEN DEVELOPMENT_NOT_SERIS and the fixed-geometry equal-resource comparison phase is authorized.** Continue #156 implemented `src/fixed_geometry.rs` and geometry-agnostic `src/annual_irradiance.rs`; candidate topology now feeds canonical resource normalization and shared visibility/sky/annual evaluation contracts. Audit 51 found framework runs `36288726695` and `36288726639` fail compilation because the shared annual evaluator had an ambiguous `sum()` type. Commit `f6e46aafc0c799ba69274faffec725abc9340d96` corrects the compile defect. Continue #158 inspected post-compile-fix runs `36289462888`/`36289462848`: compilation succeeds and 67/68 framework/library tests pass; the sole failure is the hemisphere analytical area-ratio tolerance at the intermediate mesh. Commit `adde4deec12cdc5e44cc340ac59ea7edf9f9080a` strengthens this to monotone multi-resolution convergence plus a refined analytical tolerance. Audit 52 at Continue #160 confirms whole-crate run `36290127196` PASS and e2e run `36290127283` PASS at `adde4deec12cdc5e44cc340ac59ea7edf9f9080a`. E2E artifact `10922505206` SHA-256 `5cb924eaaa8d69afd4b3852eb7f3c6e304735b527d30cadd245910feb2c84e6e`; Rust artifact `10921797900` SHA-256 `be82d75632c40b4deea0867fc5a61f4c84f79a143e4951529bffcb359f804ae6`. **The common fixed-geometry comparison framework is FROZEN and controlled matched-resource annual candidate-comparison execution is authorized for the next session. No comparison/ranking has yet been executed or promoted.**

## Session state

- Current ChatGPT session explicit `Continue` count: **11**.
- Persisted technical/audit state was recovered from the master source, this counter, the audit ledger, Audit 46, Audit 47 and repository history before substantive work.
- Session rotation required: **no — new session resumed successfully from persisted Continue-184 rotation state; 3/12 Continues used.**
- Next-pass state: **Continue #195 completed as Pass 1 of Audit Cycle 67; total project Continue 195; new-session Continue 11/12; passes since Audit 66 = 1. TECHNICAL PROGRAMME ON TRACK; FINAL REPORT QUALITY AT RISK — RECOVERABLE; #227 feasibility AMBER. Timestep annual + monthly aggregate-back PASS: 87,840 rows, 8,784 per accepted geometry/resource case, annual max total residual ~7.683e-9 Wh, monthly 480 component checks with zero failures and max absolute residual 5.06e-7 Wh under tolerance max(1e-6 Wh,1e-9*|frozen|). Provenance manifest committed at docs/evidence/timestep_irradiance_provenance_manifest.json. Bounded annual electrical pre-audit package implementation exists but canonical retained workflow evidence is still pending at pass close; therefore annual electrical evidence is NOT YET PROMOTED/FROZEN. Sections 1-4 rewritten in LaTeX/Markdown; bibliography activated; concept figure overlap repaired. Current readable report evidence includes 0ea105be / run 36381080050 PASS / artifact 10953395151 / artifact SHA-256 1388b8aa06d2ad5e15175ccb307a9b9c4c1a0cb36d4a2f49e7b70f8736669eaf / PDF SHA-256 766f9fa3d14fe40b5cc6882383a1c3c4844e2d79f8c7f026ad058a90594ecce4. #196 must continue Phase-II bounded engineering work AND repair Sections 5-8; #197 is mandatory Audit 67 plus Phase-II/report checkpoint and 12/12 session rotation.**

## Session-rotation rule

- Maximum explicit `Continue` commands per ChatGPT session: **12**.
- A new session resets only the per-session counter; total project count, audit cadence, technical state and unresolved deficiencies persist.
- On the 12th Continue, complete any due audit/state recording but do not begin another substantive pass in that session.

## New-session bootstrap rule

At the first substantive project turn in every new session:
1. Consult the Project Source `MASTER_INSTRUCTIONS_MUSHROOM_HEAD_SOLAR`.
2. Read `docs/audits/PASS_COUNTER.md` and `docs/audits/AUDIT_LEDGER.md` before doing substantive work.
3. Treat every explicit user message `Continue` as one project pass.
4. Update this file on every explicit `Continue`, not only on audit turns.
5. If `passes since most recent audit` becomes 3, perform the master-instruction audit before substantive work, record it in `AUDIT_LEDGER.md`, then reset the counter to 0.
6. Audit earlier when a major model layer, numerical result, optimisation result, design conclusion or milestone is introduced.
7. If this state file is missing, inconsistent, or stale, do not guess the cadence. Reconstruct it from the audit ledger/project history or perform an audit conservatively before expansion.

## Scope clarification

The counter tracks explicit `Continue` commands because the user requested that a `Continue` count as a project pass. Other substantive user turns may still trigger an early audit when required by the master instruction.
