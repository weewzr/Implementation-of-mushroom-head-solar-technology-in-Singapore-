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


## Independently cross-checked Appendix A.5 fixture

The Appendix A.5 example has now been cross-checked against the indexed NREL report and independent implementations that explicitly identify the values as the SPA paper reference case. This record still does not claim that the project's current preliminary Rust equations reproduce SPA.

### Inputs

| Quantity | Reference value |
|---|---:|
| Date | 2003-10-17 |
| Local standard time | 12:30:30 |
| Time-zone offset | -7 h |
| Longitude | -105.1786 deg (east-positive project convention) |
| Latitude | 39.742476 deg |
| Elevation | 1830.14 m |
| Pressure | 820 mbar |
| Temperature | 11 deg C |
| Surface slope | 30 deg |
| Surface azimuth rotation | -10 deg |
| Delta T | 67 s |

The corresponding UTC instant is 2003-10-17 19:30:30 UTC.

### Reference outputs selected for the first project benchmark

| Quantity | NREL SPA reference value |
|---|---:|
| Topocentric zenith angle | 50.11162 deg |
| Topocentric azimuth angle, clockwise from North | 194.34024 deg |
| Surface incidence angle | 25.18700 deg |

The first Rust benchmark should use zenith and azimuth. Surface incidence belongs to a later cross-check once the SPA surface-orientation convention has been mapped explicitly to the project's facet-normal convention.

### Convention mapping

The project uses longitude positive east of Greenwich and azimuth clockwise from North. The Appendix case longitude is therefore represented as -105.1786 deg. The selected SPA azimuth output is already the north-clockwise form needed by the project's ENU constructor. Zenith converts to geometric elevation for the downstream ENU representation as `elevation = 90 deg - zenith`, but atmospheric/topocentric semantics must remain explicit rather than being silently equated with the project's preliminary geometric elevation.

### Evidence status

The fixture values are now suitable as **reference data for a future SPA-equivalent implementation test**. They are not a test of `cooper_declination`, `mean_local_solar_time_h`, `solar_elevation`, or `solar_azimuth_from_north`, because those functions intentionally implement a lower-fidelity preliminary model.
