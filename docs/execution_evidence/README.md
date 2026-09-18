# Rust execution evidence

This directory stores immutable-style execution records required by `docs/rust_execution_evidence_protocol.md`.

## Current status

No qualifying Rust execution record has yet been captured for the canonical repository.

## Required filename convention

Use:

`YYYY-MM-DD_<short-commit-sha>_rust_execution.md`

Each record must identify the exact commit and include the complete required command evidence:

- `rustc --version --verbose`
- `cargo --version --verbose`
- `cargo test`
- `cargo run --release`
- exit statuses, UTC timestamp, OS/architecture, working-tree cleanliness, and execution environment

A record is evidence only for the exact commit it names. Later source changes require a new record.

## Validation boundary

Successful Rust execution validates only the tested software/analytical properties. It does not validate Singapore irradiance data, annual yield, bifacial/thermal/structural models, optimisation, uncertainty, or economics.
