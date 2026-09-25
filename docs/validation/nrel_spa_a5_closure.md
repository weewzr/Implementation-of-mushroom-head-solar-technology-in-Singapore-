# NREL SPA Appendix A.5 closure — Continue #139

## Scope
This note records the physical-validation closure of the solar-position gate only. V1–V5 numerical-method verification is not revisited.

## Authoritative benchmark
NREL Reda–Andreas Appendix A.5 specifies 17 October 2003, 12:30:30 LST, UTC−7, longitude −105.1786°, latitude 39.742476°, elevation 1830.14 m, pressure 820 mbar, temperature 11 °C and ΔT=67 s. The published outputs include zenith 50.11162° and azimuth 194.34024°. The project acceptance tolerance remains 0.001° for each angle.

## Term-by-term diagnosis of the reduced implementation

| SPA chain term | Reduced implementation before #139 | Authoritative SPA requirement | Disposition |
|---|---|---|---|
| Julian day / ephemeris time | Present | JD → JC/JDE/JCE/JME | Retained |
| Earth heliocentric longitude L | Low-order mean longitude + equation-of-centre approximation | Six VSOP87 longitude series L0…L5 with 64/34/20/7/3/1 terms | Replaced |
| Earth heliocentric latitude B | Forced to zero | Two VSOP87 latitude series B0/B1 | Replaced |
| Earth radius vector R | Low-order eccentric-orbit expression; earlier denominator defect corrected in Audit 44 | Five VSOP87 radius series R0…R4 | Replaced |
| Geocentric longitude/latitude | Reduced solar-longitude convention; earlier 180° convention defect corrected in Audit 44 | θ=L+180°, β=−B after heliocentric Earth series | Replaced with SPA chain |
| Nutation Δψ, Δε | Single low-order Ω correction | Five fundamental arguments plus all 63 Y/PE periodic terms | Replaced |
| Mean/true obliquity | Short cubic approximation | SPA tenth-order polynomial in U plus Δε | Replaced |
| Aberration | Fixed low-order longitude correction | −20.4898/(3600R) | Replaced |
| Apparent longitude λ | Approximate | θ+Δψ+Δτ | Replaced |
| Sidereal time | Mean sidereal time plus approximate nutation | ν0 plus Δψ cos ε | Replaced |
| Right ascension / declination | Formula present but fed reduced λ, β, ε | SPA geocentric α, δ | Retained formula with full upstream terms |
| Parallax/topocentric correction | Present | SPA ξ, Δα, δ′ and H′=H−Δα | Corrected/retained; H′ sign follows SPA |
| Refraction | Present with simplified horizon condition | SPA pressure/temperature correction and solar-radius + atmospheric-refraction horizon gate | Corrected |
| Zenith / azimuth | Same terminal geometry | SPA topocentric outputs | Retained |

The approximately 0.00648° A.5 azimuth error observed at Audit 44 was therefore not attributable to a defensible single tolerance-scale term. It was the residual of a reduced astronomical chain whose missing VSOP87 latitude/longitude/radius, full nutation and obliquity terms were material at the project acceptance level. The correct repair was replacement with the full SPA terms, not tolerance relaxation.

## Rust implementation
`src/spa.rs` is now a Rust-only implementation of the SPA zenith/azimuth path using the published L/B/R periodic tables and 63-term nutation table. No C/FFI runtime dependency is introduced.

## Execution evidence
Commit `3ee7593f98993c4da8f140d04dcf690c7d912a32`, GitHub Actions Rust execution evidence run **36177517373**:
- canonical `cargo test`: PASS;
- NREL Appendix A.5 test: PASS at unchanged 0.001° zenith/azimuth tolerance;
- canonical release executable `cargo run --release --bin mushroom-solar`: PASS;
- tracked working tree after execution: clean;
- evidence artifact upload: PASS.

The preceding commit `3ba2f68f642395941cc766d2a94c41bf05244f44` already demonstrated 49/49 library tests passing, including the A.5 fixture; its workflow failed only because an unqualified `cargo run --release` was ambiguous among four binaries. That evidence-workflow defect was corrected without changing SPA acceptance criteria.

## Gate decision
**NREL SPA Appendix A.5 physical-validation gate: PASS / CLOSED.**

This closure validates the implemented solar-position algorithm against the pinned A.5 fixture. It does not by itself validate Singapore weather, POA irradiance, visibility, diffuse-sky treatment, thermal/electrical conversion or annual yield.
