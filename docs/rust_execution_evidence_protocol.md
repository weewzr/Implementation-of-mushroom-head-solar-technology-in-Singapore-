# Rust Execution-Evidence Protocol

**Purpose:** distinguish source-level test presence from evidence that the canonical Rust repository has actually compiled and executed its tests. This protocol is a foundation quality gate; it is not a numerical-model result.

## Required command set

At a clean checkout of the exact commit being validated, record:

```text
rustc --version --verbose
cargo --version --verbose
cargo test
cargo run --release
```

Do not edit source files between recording the commit identifier and executing the commands.

## Evidence record

A valid execution record must contain:

1. repository full name and branch;
2. exact Git commit SHA;
3. UTC execution timestamp;
4. operating system and architecture;
5. `rustc --version --verbose` output;
6. `cargo --version --verbose` output;
7. complete `cargo test` summary, including number of passed/failed/ignored tests;
8. exit status for `cargo test`;
9. `cargo run --release` output and exit status;
10. whether the working tree was clean;
11. identity of the execution environment (local machine, CI runner, container, etc.);
12. links or immutable identifiers for CI logs/artifacts when applicable.

## Acceptance gate

The Rust source may be labelled **execution-verified at commit X** only when:

- compilation completes successfully;
- `cargo test` exits with status 0;
- no test failure is hidden by filtering or ignored error handling;
- the recorded commit is the commit whose source is being discussed;
- logs are retained or reproducibly accessible.

Passing source tests establishes only the tested analytical/software properties. It does **not** validate Singapore annual yield, weather inputs, irradiance physics, optimisation, uncertainty, structure or economics.

## Failure handling

If compilation or a test fails:

- preserve the failing output;
- identify the failing commit and toolchain;
- correct the source or test rather than deleting the evidence;
- execute the full command set again at the corrected commit;
- retain the failure as engineering history when it materially explains a correction.

## Current status — 18 September 2026

**Not yet execution-verified.** Source-level Rust tests are present in the repository, including analytical geometry, solar geometry, ENU vector, facet-incidence and equal-resource-contract checks, but no qualifying execution record has yet been persisted.

README commands such as `cargo test` are reproduction instructions and must not be cited as proof that the tests passed.

## CI evidence path

The repository now includes `.github/workflows/rust-evidence.yml`, which is intended to generate the required execution record automatically on relevant pushes and manual dispatches. The workflow captures the exact commit/environment, toolchain versions, canonical test/run output, exit statuses and an evidence artifact.

**Workflow presence is not execution evidence.** A qualifying CI result requires an actual completed run associated with the exact commit being validated, successful canonical command exit statuses, and retained logs/artifacts. If no workflow run is observable for a commit, its Rust execution status remains **not verified**.
