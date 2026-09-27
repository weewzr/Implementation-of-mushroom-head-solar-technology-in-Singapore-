# Mandatory Audit 51 — Continue #157

**Date:** 27 September 2026  
**Trigger:** Continue #157, mandatory Pass 3 audit cycle 51.  
**Scope:** shared fixed-geometry framework validation only; no comparative performance execution or new physics.

## Disposition

**FAIL / CORRECTION REQUIRED / FRAMEWORK FREEZE WITHHELD.**

Controlled matched-resource annual candidate comparisons are **not yet authorized**.

## Evidence inspected

- Rust framework evidence run **36288726695**, commit `e4fd213ff3664315f9c5ba2c82397201a640ab90`: failed during Rust compilation; retained artifact `10921896515`, SHA-256 `44f0a3c71ba5549c97f17e2908e4b4fef46c742f101a572f7db003db318a2bf4`.
- End-to-end run **36288726639**, same commit: failed at `cargo test --all-targets`; no e2e artifact.
- `src/fixed_geometry.rs`, `src/annual_irradiance.rs`, canonical `src/resource_geometry.rs`, and `src/visibility.rs`.

## Architecture audit

The intended separation is sound: fixed-geometry generators own topology; resource normalization/accounting is delegated to `resource_geometry`; annual irradiance is geometry-agnostic and uses the common SPA, direct visibility/self-shadowing, sky-view and albedo paths. Equal-land/equal-PV contracts call the canonical normalizers; matched-packing validates intrinsic packing before equal-land scaling. No comparative headline result or geometry ranking has been promoted.

## Exact blocker

Both CI lineages fail before framework tests execute because `src/annual_irradiance.rs` leaves the type of `e.iter().sum()` ambiguous. This is a compile-time framework defect, not a numerical/physics failure. Commit `f6e46aafc0c799ba69274faffec725abc9340d96` corrects it to an explicit `f64` sum.

Because the inspected commit does not compile, geometry analytical/limiting tests, orientation/resource invariants and common-pipeline execution cannot be released despite being present in source.

## Release decision

The shared fixed-geometry comparison framework is **NOT FROZEN**. The next pass must inspect fresh whole-crate and e2e CI from `f6e46aafc0c799ba69274faffec725abc9340d96` or a descendant containing only necessary framework corrections. No controlled candidate comparison results may be generated or promoted until that evidence passes.

The previously frozen paraboloid DEVELOPMENT_NOT_SERIS benchmark is unaffected.
