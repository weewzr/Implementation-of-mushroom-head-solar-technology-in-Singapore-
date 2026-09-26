# Persistent Project Pass Counter

**Purpose:** make the mandatory audit cadence survive new ChatGPT sessions. This file is a project-control state record, not technical evidence.

## Current state

- Total explicit user `Continue` commands counted through 26 September 2026: **151**.
- Most recent mandatory audit: `docs/audits/2026-09-26_three-pass_audit-49.md`.
- That audit occurred on Continue #151.
- Passes since most recent audit: **0**.
- Audit 49 inspected corrected run 36237551175. Whole-crate `cargo test --all-targets` failed compilation because the response source contains a literal `\\n` token introduced during the binary-local normalization correction; no end-to-end response artifacts were produced. Audit 49 also identified duplicated binary-local geometry/resource construction as the root architectural blocker. **Paraboloid-response freeze remains withheld until a canonical tested Rust discrete geometry/resource-normalization API is implemented and used by every response/convergence/robustness/attribution path, followed by successful whole-crate/e2e evidence and report/traceability parity closure**.

## Session state

- Current ChatGPT session explicit `Continue` count: **3**.
- Persisted technical/audit state was recovered from the master source, this counter, the audit ledger, Audit 46, Audit 47 and repository history before substantive work.
- Session rotation required: **no**.
- Next-pass state: **Continue #152 is Pass 1 of audit cycle 50. Audit-49 corrective architecture is the sole priority: implement and test the canonical discrete geometry/resource-normalization API before repairing consumers; no new modelling, response freeze or fixed-geometry comparison is authorized.**

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
