# Persistent Project Pass Counter

**Purpose:** make the mandatory audit cadence survive new ChatGPT sessions. This file is a project-control state record, not technical evidence.

## Current state

- Total explicit user `Continue` commands counted through 28 September 2026: **187**.
- Most recent audit: `docs/audits/2026-09-27_early-major-result_audit-62.md`.
- That audit occurred on Continue #186.
- Passes since most recent audit: **1**.
- Audit 50 at Continue #154 correctly withheld release pending the completed canonical consumer refactor. Continue #155 reconciled stale governance state against post-audit evidence: commit `53d7d3420202ea81bf5b0a8d40b1ee87a93a68c4`, Rust run `36240614922` PASS, e2e run `36240614938` PASS, artifact `10905852093` SHA-256 `e450ccd79f303bd1a701293dbbc5613f156520c122a92ccbb6229b08e2eaf110`. Fresh artifact inspection closes exact-resource, convergence, crossover, sensitivity and stability gates; Markdown/LaTeX/traceability parity is synchronized. **Paraboloid geometric-response result is FROZEN DEVELOPMENT_NOT_SERIS and the fixed-geometry equal-resource comparison phase is authorized.** Continue #156 implemented `src/fixed_geometry.rs` and geometry-agnostic `src/annual_irradiance.rs`; candidate topology now feeds canonical resource normalization and shared visibility/sky/annual evaluation contracts. Audit 51 found framework runs `36288726695` and `36288726639` fail compilation because the shared annual evaluator had an ambiguous `sum()` type. Commit `f6e46aafc0c799ba69274faffec725abc9340d96` corrects the compile defect. Continue #158 inspected post-compile-fix runs `36289462888`/`36289462848`: compilation succeeds and 67/68 framework/library tests pass; the sole failure is the hemisphere analytical area-ratio tolerance at the intermediate mesh. Commit `adde4deec12cdc5e44cc340ac59ea7edf9f9080a` strengthens this to monotone multi-resolution convergence plus a refined analytical tolerance. Audit 52 at Continue #160 confirms whole-crate run `36290127196` PASS and e2e run `36290127283` PASS at `adde4deec12cdc5e44cc340ac59ea7edf9f9080a`. E2E artifact `10922505206` SHA-256 `5cb924eaaa8d69afd4b3852eb7f3c6e304735b527d30cadd245910feb2c84e6e`; Rust artifact `10921797900` SHA-256 `be82d75632c40b4deea0867fc5a61f4c84f79a143e4951529bffcb359f804ae6`. **The common fixed-geometry comparison framework is FROZEN and controlled matched-resource annual candidate-comparison execution is authorized for the next session. No comparison/ranking has yet been executed or promoted.**

## Session state

- Current ChatGPT session explicit `Continue` count: **3**.
- Persisted technical/audit state was recovered from the master source, this counter, the audit ledger, Audit 46, Audit 47 and repository history before substantive work.
- Session rotation required: **no — new session resumed successfully from persisted Continue-184 rotation state; 3/12 Continues used.**
- Next-pass state: **Continue #187 completed as Pass 1 of Audit Cycle 63; new-session Continue 3/12; passes since Early Audit 62 = 1. PHASE I COMPLETE. Exact Phase-I report baseline remains commit 3f54605f53a6e88f145713f69d4136da297f2608 / run 36330877451 PASS / artifact 10935284619 / artifact SHA-256 3f4b0efe50648c2f6bb23890a893d88b04848d288ffe363d8275a18fd9749ec3 / exact 32-page PDF SHA-256 2315a125011117a0210ab15f63c2d011941271cae0057a0bd816b8bb61074838. Frozen layers preserved: Audit-59 comparison DEVELOPMENT_NOT_SERIS, Audit-62 communication architecture and narrow electrical kernel, Audit-56 single crease, Audit-60 accordion/fan. Phase II is active at #188 under docs/PHASE_II_ELECTRICAL_ENGINEERING_PLAN.md. #188 is thermal-model evidence selection/contract only; no annual electrical kWh. Normal mandatory Audit 63 is due #189. Overall ~#227 target remains ON SCHEDULE.**

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
