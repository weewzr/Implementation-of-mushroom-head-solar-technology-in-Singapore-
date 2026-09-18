# Canonical Singapore Time-Series Input Contract

**Status:** foundation specification. This document defines what a weather/irradiance dataset must satisfy before it can support annual 3-D PV comparisons. It does not assert that a qualifying dataset has been acquired.

## 1. Purpose and validation boundary

Annual geometry comparisons require time-correlated solar and meteorological inputs. A published annual-average irradiance value is useful context but is not a substitute for a time series because orientation, shading, diffuse response, temperature and tracking depend on conditions at each time step.

A dataset becomes a **canonical simulation input** only after its provenance, licence, time basis, variables, units, QC and preprocessing are recorded and accepted under this contract.

## 2. Required fields

| Field | Canonical unit | Requirement | Notes |
|---|---:|---|---|
| timestamp | ISO 8601 | required | Must include an explicit UTC offset or be accompanied by an unambiguous timezone convention. |
| GHI | W m^-2 | required | Measured/source-derived status must be recorded. |
| DHI | W m^-2 | required for validated diffuse modelling | Must not be replaced by a fixed diffuse fraction for validated annual work. |
| DNI | W m^-2 | required | May be measured or derived; derivation method must be named and uncertainty tracked. |
| ambient temperature | degC | required for thermal layer | Source/sensor status required. |
| wind speed | m s^-1 | required for thermal/mechanical layers | Measurement height must be retained when available. |
| wind direction | degree clockwise from north | required for directional wind mechanics when used | Convention must be explicit. |
| relative humidity | % | recommended | Retain when available. |
| air pressure | Pa or hPa | recommended | Original unit must be recorded and conversion traceable. |
| site latitude/longitude | degree | required metadata | Coordinate datum should be recorded when supplied. |
| site elevation | m | recommended metadata | Reference datum should be retained when known. |

Additional variables may be retained, but they must not silently change the minimum model contract.

## 3. Time basis

Singapore civil timestamps must be represented with an explicit time basis. The canonical ingestion layer shall preserve the source timestamp and create a normalized timestamp with timezone information. Daylight-saving adjustments must not be invented.

The acquisition manifest must state:

- source timezone/UTC offset;
- whether timestamps denote interval start, centre or end;
- native sampling interval;
- whether timestamps are regular or event-driven;
- leap-day handling;
- any clock correction supplied by the provider.

Solar position must be calculated from the normalized timestamp and site coordinates through the project's validated solar-position path; civil clock time must not be treated directly as apparent solar time.

## 4. Provenance and licensing gate

Every candidate dataset requires a manifest containing provider, product/dataset name, retrieval date, source identifier/URL, station/site identifier, coordinates, coverage period, native resolution, variables, units, sensor/derived status, licence/permission terms, redistribution status and a checksum or immutable identifier when permitted.

**Public-repository rule:** raw third-party measurements may be committed only when redistribution rights explicitly permit it. Otherwise raw files remain outside Git and the repository stores acquisition instructions, manifests, schemas, preprocessing code and only licence-permitted derived outputs.

The current preferred measured-data route described in `data/README.md` is SERIS, but the repository does not currently establish an open bulk historical-data redistribution licence. Therefore SERIS historical time series remain **required but not acquired** for validation.

## 5. Quality-control gates

Ingestion must detect and report, without silently repairing:

1. missing timestamps and gaps;
2. duplicate timestamps;
3. non-monotonic timestamps;
4. unexpected sampling-interval changes;
5. non-finite values;
6. negative irradiance values requiring source/QC interpretation;
7. physically impossible or provider-flagged values;
8. unit/schema mismatches;
9. inconsistent site metadata;
10. irradiance closure inconsistencies where GHI, DHI and DNI are jointly available.

Provider quality flags must be preserved rather than discarded.

Any imputation or correction is a separate preprocessing operation and must leave the raw/source-derived record traceable.

## 6. Irradiance consistency

Where GHI, DHI and DNI are available, the preprocessing/validation layer shall check consistency against the solar geometry relation

```text
GHI ~= DHI + DNI * cos(theta_z)
```

for sun-above-horizon samples, subject to measurement uncertainty, timing mismatch, sensor response and QC tolerances.

Here `theta_z` is solar zenith angle. This relation is a diagnostic, not permission to overwrite measurements.

If DNI must be derived, the decomposition model, implementation/version, input variables and uncertainty status must be recorded. Derived DNI must never be labelled measured.

## 7. Resampling and aggregation

The native time series should be preserved when permitted. Any resampling must document:

- source and target intervals;
- averaging/integration rule;
- treatment of gaps;
- treatment of instantaneous versus interval-average measurements;
- timezone handling;
- energy-conservation check for irradiance integration;
- later timestep-sensitivity/convergence evidence.

Annual energy calculations must not mix incompatible instantaneous and interval-average conventions.

## 8. Missing-data policy

No single universal imputation rule is assumed. Each gap-treatment method must be explicit and sensitivity-tested when material.

At minimum, the annual-run record must report total expected samples, valid samples, missing samples, longest gap, fraction imputed (if any), and excluded periods.

A dataset with material gaps may be suitable for exploratory testing while remaining unsuitable for a validated annual comparison.

## 9. Acceptance states

A weather dataset progresses through these states:

1. **candidate source** — identified but rights/content not verified;
2. **accessible source** — retrieval route established;
3. **licence-cleared source** — permitted uses and redistribution recorded;
4. **ingested dataset** — schema/time basis parsed reproducibly;
5. **QC-reviewed dataset** — QC report generated and material defects resolved or bounded;
6. **canonical simulation input** — provenance, licence, QC and preprocessing gates passed for the intended model;
7. **validation-supporting input** — additionally accompanied by uncertainty/sensitivity treatment appropriate to the result being claimed.

These states prevent “downloaded” from being treated as synonymous with “validated”.

## 10. Required repository artefacts

Before annual-yield validation, the repository should contain:

- this contract;
- one acquisition manifest per external source;
- a machine-readable schema/configuration used by the Rust ingestion layer;
- Rust ingestion and QC code;
- tests using small synthetic or redistribution-safe fixtures;
- QC summaries for the selected dataset;
- preprocessing provenance;
- licence/redistribution record;
- timestep-sensitivity evidence;
- traceability links from dataset -> model run -> figures/tables -> report claim.

## Current status — 18 September 2026

**Contract defined; canonical measured time series not yet acquired.** No annual Singapore geometry comparison is validated by this document alone.
