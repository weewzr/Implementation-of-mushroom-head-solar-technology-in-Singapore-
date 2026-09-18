# NREL SPA validation target

## Purpose

This record freezes the authoritative validation target for the project's future production solar-position path. It does **not** claim that the current Rust solar implementation is SPA-equivalent.

## Primary reference

Ibrahim Reda and Afshin Andreas, *Solar Position Algorithm for Solar Radiation Applications*, NREL/TP-560-34302, revised January 2008.

Authoritative NREL PDF:

https://www.nrel.gov/docs/fy08osti/34302.pdf

The NREL publication describes the Solar Position Algorithm (SPA) and is the project's primary reference for high-fidelity solar-position validation.

## Project use

The current `src/solar.rs` layer deliberately separates:

1. civil standard clock time;
2. longitude/time-zone correction to mean local solar time;
3. the still-missing apparent-solar-time / higher-fidelity astronomical correction;
4. solar elevation and azimuth geometry.

The current Cooper-declination and spherical relations remain preliminary. They must not be promoted to the validated timestamped annual-yield path merely because analytical unit tests pass.

## Validation gate

Before annual Singapore yield claims use the production solar-position path:

- implement or integrate a high-fidelity algorithm whose conventions are explicitly mapped to the project's ENU / clockwise-from-North convention;
- construct benchmark fixtures from the NREL SPA reference/example material with all required inputs recorded;
- compare Rust outputs against the reference outputs using an explicitly justified numerical tolerance;
- retain qualifying whole-crate Rust execution evidence;
- document any atmospheric/refraction choices and distinguish geometric from apparent/topocentric quantities.

No benchmark number is copied into this record until it has been independently extracted and checked from the authoritative reference. This prevents a remembered or secondary-source value from becoming a false validation fixture.

## Status

**Authoritative reference selected; benchmark fixture and execution evidence still open.**
