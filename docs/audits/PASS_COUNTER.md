# Persistent Project Pass Counter

**Purpose:** make the mandatory audit cadence survive new ChatGPT sessions. This file is a project-control state record, not technical evidence.

## Current state

- Total explicit user `Continue` commands counted through 19 September 2026: **126**.
- Most recent mandatory audit: `docs/audits/2026-09-19_three-pass_audit-40.md`.
- That audit occurred on Continue #126.
- Passes since most recent audit: **0**.
- Current explicit `Continue` is project #126 / mandatory Audit 40 completed. Rust execution and irradiance closure/POA foundations pass. Trustworthy annual Baseline Model Phase remains blocked by absence of authorised canonical annual Singapore irradiance data and absence of an implemented SPA-equivalent solver that reproduces the authoritative fixture. No annual-yield result is promoted.

## Session state

- Current ChatGPT session explicit `Continue` count: **10**.
- This is a new session bootstrap; persisted technical/audit state has been recovered from the master source, this counter, the audit ledger and recent repository history.
- Session rotation required: **no**.
- Next-pass state: **project #127 / Pass 1 of audit cycle 41 and session Continue #11; close the two science release gates directly, then authorize Baseline Model Phase if they pass**.

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
