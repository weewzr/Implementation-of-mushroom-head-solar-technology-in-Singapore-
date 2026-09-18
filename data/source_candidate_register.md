# Singapore Weather/Irradiance Source Candidate Register

**Status:** evidence-control foundation. A candidate listed here is not automatically licence-cleared or suitable for validated annual simulation.

## SERIS Singapore irradiance network

### Verified public capability

The project currently treats the Solar Energy Research Institute of Singapore (SERIS) monitoring network as the preferred primary measured-irradiance candidate because the public SERIS monitoring material documents a Singapore network with 25 GHI stations and 10 more fully equipped stations measuring diffuse horizontal irradiance and meteorological quantities including ambient temperature/relative humidity, wind speed/direction and air pressure. The monitoring system is described as operating at 1-second temporal resolution with precise time synchronisation and automated data handling.

### Evidence boundary

The public monitoring description establishes measurement capability; it does **not**, by itself, establish that the historical station time series may be bulk-downloaded or redistributed in this public GitHub repository. Until explicit access and permission terms are obtained, historical measurements remain external/unacquired and must not be committed or described as open data.

### Candidate-state fields

| Field | Current state |
|---|---|
| Provider | SERIS, National University of Singapore |
| Product | Singapore irradiance monitoring network / National Solar Repository context |
| Candidate state | candidate source; public capability verified, historical access/licence not cleared |
| GHI | publicly documented network capability |
| DHI | publicly documented at fully equipped stations |
| DNI | not assumed measured; canonical status unresolved until actual dataset metadata is obtained |
| Ambient temperature/RH | publicly documented at fully equipped stations |
| Wind speed/direction | publicly documented at fully equipped stations |
| Air pressure | publicly documented at fully equipped stations |
| Native monitoring resolution | 1 s capability stated publicly; actual delivered historical dataset interval must be recorded separately |
| Station identifier | unresolved until authorised dataset/station is selected |
| Station coordinates | unresolved until authoritative station metadata is obtained |
| Timestamp convention | unresolved for historical delivery; must not be inferred from live-display behaviour |
| Historical access route | contact/request route; exact delivery mechanism unresolved |
| Licence/permission | unresolved |
| Raw redistribution | **not permitted by project governance unless explicit permission is established** |
| Coordinate tolerance | unresolved; derive from authoritative coordinate precision/metadata semantics, not a convenient default |

### Acceptance actions

Before this candidate can become canonical simulation input:
1. obtain an authorised historical dataset or documented access route;
2. retain the provider's licence/permission terms;
3. identify the selected station and authoritative coordinates;
4. record timestamp semantics and actual delivered sampling interval;
5. resolve measured/derived/unavailable status for every canonical variable;
6. bind those facts into the TOML acquisition manifest;
7. run the Rust ingestion/QC path and retain execution evidence;
8. generate a dataset-specific QC report before annual modelling.

## Supplementary NEA/data.gov.sg meteorology

Official Singapore open-data meteorological datasets may be used as supplementary environmental inputs where their catalogue explicitly states the applicable licence and variables. They are not substitutes for measured GHI/DHI merely because they are open data. Each selected dataset requires its own manifest and variable-level provenance.

## Anti-fabrication rule

Unknown fields remain explicitly unresolved. In particular, this register does not invent station coordinates, station IDs, historical time coverage, DNI measurement status, coordinate tolerance, licence terms or redistribution permission.
