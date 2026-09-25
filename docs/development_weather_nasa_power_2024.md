# Singapore Development Weather — NASA POWER 2024

**Status:** development dataset acquired and Rust-QC accepted. **Not SERIS measured validation data.**

## Purpose and boundary
This dataset exists so the annual baseline pipeline can be developed reproducibly without waiting for SERIS access. It must not be described as measured Singapore validation weather or used to validate a final annual performance claim.

## Source and reproducible acquisition
- Provider: NASA Langley Research Center POWER.
- Product: POWER Hourly Point API, Renewable Energy community.
- Query point: 1.3521° N, 103.8198° E (project development reference point in Singapore).
- Source grid elevation returned by POWER: 25.8 m.
- Requested period: 2024-01-01 through 2024-12-31 inclusive.
- Requested time standard: UTC.
- Acquisition workflow: `.github/workflows/acquire-development-weather.yml`.
- Successful acquisition + full-year Rust QC: GitHub Actions run **36179970264**.
- Raw CSV SHA-256: `f3442ca0c336011c5c61fe434e00e1f7984f9d34afa27d1431fa939f5eac204c`.
- Acquired raw-file size at run: about 494 KiB; 8,801 physical lines including POWER metadata/header and 8,784 hourly records.

## Time semantics
POWER's Hourly API was explicitly requested with `time-standard=UTC`. POWER documents hourly timestamps as the **start of the hour for the whole hour** and the hourly service as providing hourly average values. 2024 is a leap year, so complete hourly coverage is 366 × 24 = **8,784 records**.

## Variables and status
All values are NASA POWER products rather than local SERIS instrument measurements.

| POWER field | Canonical use | Source status/unit |
|---|---|---|
| `ALLSKY_SFC_SW_DWN` | GHI | CERES-derived all-sky surface shortwave downward irradiance, Wh/m² for each hour |
| `ALLSKY_SFC_SW_DIFF` | DHI | CERES-derived diffuse irradiance, Wh/m² for each hour |
| `ALLSKY_SFC_SW_DNI` | DNI | CERES-derived/calculated direct-normal irradiance, Wh/m² for each hour |
| `T2M` | ambient temperature | MERRA-2, °C |
| `WS10M` | wind speed | MERRA-2, m/s at 10 m |
| `WD10M` | wind direction | MERRA-2, degrees |
| `RH2M` | relative humidity | MERRA-2, % |
| `PS` | surface pressure | MERRA-2, kPa; Rust ingestion converts ×1000 to Pa |

For a one-hour interval, Wh/m² over the hour is numerically equal to the interval-average W/m² used by the canonical hourly integration. This equivalence must not be reused after changing interval duration without an explicit energy-preserving conversion.

## Missing-data and QC policy
POWER declares `-999` as its unavailable/missing sentinel. The Rust adapter rejects a row containing a POWER fill sentinel rather than silently imputing it. No imputation was applied to the acquired 2024 file.

Full-year Rust QC result:
- parsed records: **8,784 / 8,784 expected**;
- QC issues: **0**;
- duplicate timestamps: **0**;
- non-monotonic timestamps: **0**;
- negative irradiance values: **0**;
- implied missing hourly samples: **0**;
- longest observed interval: **3,600 s**.

The first ingestion CI run is intentionally retained: it caught an incorrect kPa→Pa test conversion before acceptance. The corrected conversion and full-year QC subsequently passed.

## Licence / redistribution
NASA Earthdata's data-use guidance states that NASA-led Earth-science data are generally openly reusable and, absent a marked restriction, are CC0; NASA should be acknowledged and any specifically marked restriction must still be respected. The repository therefore records the source/query/checksum and may retain or redistribute the acquired NASA data subject to that policy. This does not change the separate licensing status of SERIS data.

## Acceptance state
This source has reached **development simulation input** status: reproducibly acquired, provenance-bound, licence-recorded, parsed by canonical Rust and QC-reviewed. It has **not** reached the project's `validation-supporting input` state and does not replace SERIS.
