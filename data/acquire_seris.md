# Canonical Singapore irradiance acquisition

## Canonical source decision

The canonical measured irradiance source for validation is the **SERIS Singapore irradiance monitoring network / National Solar Repository context**.

SERIS publicly documents 25 Singapore GHI stations on an approximately 5 km x 5 km grid. Ten fully equipped stations additionally measure diffuse horizontal irradiance, ambient temperature/relative humidity, wind speed/direction and air pressure, with 1-second monitoring capability.

## Access and licence boundary

As of 2026-09-19, the public SERIS monitoring page documents the measurement system and a contact route, but does not establish a historical bulk-download endpoint or an open licence permitting redistribution of historical station time series.

Therefore:

- the project must not invent a download URL;
- the project must not commit third-party raw SERIS observations without permission;
- canonical status means **selected canonical provider/product, acquisition pending authorisation**, not "data already acquired";
- annual validation remains blocked until an authorised historical delivery is obtained.

Official source:
https://www.seris.nus.edu.sg/services/real-time-monitoring-system-of-irradiance/

Contact published by SERIS:
- SOE Pyae, Team Leader, PV Monitoring
- soepyae@nus.edu.sg
- seris-info@nus.edu.sg
- seris-services@nus.edu.sg

## Acquisition mechanism

1. Request an historical export from a fully equipped SERIS station covering the intended validation period.
2. Request/retain written licence or permission terms, including raw and derived redistribution rights.
3. Save the provider-delivered file outside Git under `data/raw/seris/` when redistribution is restricted.
4. Copy `data/manifests/seris_canonical_pending.toml` and replace unresolved fields only from provider metadata.
5. Convert provider columns to the canonical CSV contract documented in `data/schemas/singapore_weather_v1.toml` without imputation.
6. Run the Rust ingestion/QC pipeline and retain its report/evidence.

## Required delivery fields

Minimum preferred measured fields: timestamp, GHI, DHI, ambient temperature, wind speed and wind direction. DNI may be derived only with a named method and explicit derived-variable provenance if not delivered as a measurement.

## Supplementary open meteorology

NEA/data.gov.sg real-time/historical APIs can supplement ambient meteorology under the Singapore Open Data Licence, but do not replace the canonical SERIS irradiance record. NEA's climate-data page states that other data types, including solar radiation, are available through a separately administered request route.

## No-data behaviour

The acquisition/QC workflow must fail clearly when the authorised raw file is absent. It must never substitute synthetic, monthly-average or unrelated open data merely to make the pipeline pass.
