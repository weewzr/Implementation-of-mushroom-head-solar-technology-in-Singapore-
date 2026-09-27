# Persistent Project Pass Counter

**Purpose:** make the mandatory audit cadence survive new ChatGPT sessions. This file is a project-control state record, not technical evidence.

## Current state

- Total explicit user `Continue` commands counted through 27 September 2026: **180**.
- Most recent mandatory audit: `docs/audits/2026-09-27_early-major-result_audit-59.md`.
- That audit occurred on Continue #179.
- Passes since most recent audit: **1**.
- Audit 50 at Continue #154 correctly withheld release pending the completed canonical consumer refactor. Continue #155 reconciled stale governance state against post-audit evidence: commit `53d7d3420202ea81bf5b0a8d40b1ee87a93a68c4`, Rust run `36240614922` PASS, e2e run `36240614938` PASS, artifact `10905852093` SHA-256 `e450ccd79f303bd1a701293dbbc5613f156520c122a92ccbb6229b08e2eaf110`. Fresh artifact inspection closes exact-resource, convergence, crossover, sensitivity and stability gates; Markdown/LaTeX/traceability parity is synchronized. **Paraboloid geometric-response result is FROZEN DEVELOPMENT_NOT_SERIS and the fixed-geometry equal-resource comparison phase is authorized.** Continue #156 implemented `src/fixed_geometry.rs` and geometry-agnostic `src/annual_irradiance.rs`; candidate topology now feeds canonical resource normalization and shared visibility/sky/annual evaluation contracts. Audit 51 found framework runs `36288726695` and `36288726639` fail compilation because the shared annual evaluator had an ambiguous `sum()` type. Commit `f6e46aafc0c799ba69274faffec725abc9340d96` corrects the compile defect. Continue #158 inspected post-compile-fix runs `36289462888`/`36289462848`: compilation succeeds and 67/68 framework/library tests pass; the sole failure is the hemisphere analytical area-ratio tolerance at the intermediate mesh. Commit `adde4deec12cdc5e44cc340ac59ea7edf9f9080a` strengthens this to monotone multi-resolution convergence plus a refined analytical tolerance. Audit 52 at Continue #160 confirms whole-crate run `36290127196` PASS and e2e run `36290127283` PASS at `adde4deec12cdc5e44cc340ac59ea7edf9f9080a`. E2E artifact `10922505206` SHA-256 `5cb924eaaa8d69afd4b3852eb7f3c6e304735b527d30cadd245910feb2c84e6e`; Rust artifact `10921797900` SHA-256 `be82d75632c40b4deea0867fc5a61f4c84f79a143e4951529bffcb359f804ae6`. **The common fixed-geometry comparison framework is FROZEN and controlled matched-resource annual candidate-comparison execution is authorized for the next session. No comparison/ranking has yet been executed or promoted.**

## Session state

- Current ChatGPT session explicit `Continue` count: **8**.
- Persisted technical/audit state was recovered from the master source, this counter, the audit ledger, Audit 46, Audit 47 and repository history before substantive work.
- Session rotation required: **no — new session resumed from persisted post-Audit-56 state; 8/12 Continues used.**
- Next-pass state: **Early Major-Result Audit 59 completed at Continue #179; passes since audit = 0; session Continue 7/12. Controlled fixed-geometry comparison is FROZEN DEVELOPMENT_NOT_SERIS from run 36323622005 commit 137327c8e54f6001bbac8a72e85a90c7aa58d0a8 artifact 10933222710 SHA-256 4c3ef238d1dd673b0389265adf9e320c1b2c63f5910a02c37f3cd1b8ddc5e51b after direct retained-file and recomputed identity/convergence/conservation audit. No universal winner/ranking. Full re-render of corrected PDF artifact 10933596872 still shows material page-2/page-3 SVG text overlap, so communication architecture remains HOLD and needs one targeted source correction/rebuild rather than another broad audit loop. Final-report completeness matrix created at docs/FINAL_REPORT_COMPLETENESS_MATRIX.md; Sections 5,9,10,11 advanced. Project remains on schedule for approximately #227. Next pass should fix the two communication figures, integrate audited comparison tables/plots into Sections 5/9/10/11, then begin accordion/fan if the communication gate closes.**

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
