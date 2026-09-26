# Persistent Project Pass Counter

**Purpose:** make the mandatory audit cadence survive new ChatGPT sessions. This file is a project-control state record, not technical evidence.

## Current state

- Total explicit user `Continue` commands counted through 26 September 2026: **154**.
- Most recent mandatory audit: `docs/audits/2026-09-26_three-pass_audit-50.md`.
- That audit occurred on Continue #154.
- Passes since most recent audit: **0**.
- Audit 49 identified duplicated binary-local normalization as the architectural blocker; Pass #152 introduced the canonical resource API. Continue #153 diagnosed run 36238259282: compilation reached the new library tests but failed because an `assert_eq!` on `Result<Vec<Triangle>, _>` unnecessarily required `Triangle: PartialEq`. Commit `16ca8a953d47d03c3d45f3d1d28b16282833df23` corrects the test to `matches!` without weakening the invalid-target assertion. Fresh Rust evidence run 36238709379 and complete e2e run 36238709394 are executing. **No fresh performance result is promoted and the paraboloid freeze remains withheld for mandatory Audit 50**.

## Session state

- Current ChatGPT session explicit `Continue` count: **6**.
- Persisted technical/audit state was recovered from the master source, this counter, the audit ledger, Audit 46, Audit 47 and repository history before substantive work.
- Session rotation required: **no**.
- Next-pass state: **Continue #154 is Pass 3 of audit cycle 50 and must perform mandatory Audit 50 before substantive work. Inspect runs 36238709379 and 36238709394 plus retained artifacts; only then decide formal paraboloid freeze.**

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
