# Persistent Project Pass Counter

**Purpose:** make the mandatory audit cadence survive new ChatGPT sessions. This file is a project-control state record, not technical evidence.

## Current state

- Total explicit user `Continue` commands counted through 18 September 2026: **118**.
- Most recent mandatory audit: `docs/audits/2026-09-18_three-pass_audit-37.md`.
- That audit occurred on Continue #117.
- Passes since most recent audit: **1**.
- Current explicit `Continue` is project #118 / Pass 1 of audit cycle 38. Diagnosed the report build failure from GitHub Actions logs as a LaTeX-unsafe SVG text label (`A_land` / `A_PV`) and replaced it with plain-language labels; corrective build evidence is pending.

## Session state

- Current ChatGPT session explicit `Continue` count: **2**.
- This is a new session bootstrap; persisted technical/audit state has been recovered from the master source, this counter, the audit ledger and recent repository history.
- Session rotation required: **no**.
- Next-pass state: **project #119 / Pass 2 of audit cycle 38 and session Continue #3; inspect the corrective report build, fix the next compile defect if any, and only begin page QA after a successful PDF artifact exists**.

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
