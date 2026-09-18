# Persistent Project Pass Counter

**Purpose:** make the mandatory audit cadence survive new ChatGPT sessions. This file is a project-control state record, not technical evidence.

## Current state

- Total explicit user `Continue` commands counted in the current reconstruction session through 18 September 2026: **31**.
- Most recent mandatory audit: `docs/audits/2026-09-18_three-pass_audit-08.md`.
- That audit occurred on Continue #30.
- Passes since most recent audit: **1**.
- Next explicit `Continue` is Pass 2 of the current audit cycle.
- Mandatory audit is due no later than the third explicit `Continue` after the most recent audit.

## Session state

- Current ChatGPT session explicit `Continue` count: **13**.
- Previous session reached the 18-Continue rotation boundary; this session resumed from persisted repository state.
- Session rotation required: **no**.
- Next-pass state: **Pass 2 of the current three-pass audit cycle**.

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