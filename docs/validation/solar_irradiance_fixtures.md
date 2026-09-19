# Solar-position and irradiance validation fixtures

## Authoritative solar-position fixture

Primary reference: Reda & Andreas, NREL/TP-560-34302 (revised 2008), Appendix A.5.

Input:
- local standard date/time: 2003-10-17 12:30:30
- timezone: UTC-7
- longitude: -105.1786 deg (east-positive convention)
- latitude: 39.742476 deg
- elevation: 1830.14 m
- pressure: 820 mbar
- temperature: 11 degC
- delta-T: 67 s
- surface slope: 30 deg
- surface azimuth rotation: -10 deg

Published output:
- apparent topocentric zenith: 50.11162 deg
- topocentric azimuth, clockwise from north: 194.34024 deg
- incidence angle: 25.18700 deg

NREL reports SPA solar-position uncertainty of +/-0.0003 deg over its stated validity range. The project acceptance tolerance is deliberately looser at **0.001 deg** for zenith and azimuth when validating an independent double-precision implementation against this rounded published fixture. Incidence-angle acceptance is also **0.001 deg**. These are implementation acceptance tolerances, not claims that the reference itself is uncertain by 0.001 deg.

## Irradiance closure fixtures

For sun above the horizon:
GHI = DHI + DNI cos(theta_z).

Exact synthetic fixtures are used because they isolate convention/arithmetic defects:
- zenith 0 deg, DNI 800, DHI 100 -> GHI 900 W/m2
- zenith 60 deg, DNI 800, DHI 100 -> GHI 500 W/m2
- zenith 90 deg -> direct-horizontal contribution is zero.

Arithmetic acceptance tolerance for these constructed cases: **1e-9 W/m2**.

Measured-data QC will not use this machine-precision tolerance. Its closure acceptance threshold must be selected from the acquired dataset's sensor uncertainty/quality specification; until SERIS delivery metadata is available that threshold remains unresolved.

## Plane-of-array fixtures

The baseline transposition validation uses the isotropic-sky model, independently cross-checkable against pvlib's `get_total_irradiance(..., model="isotropic")` component definition:
POA_global = POA_direct + POA_sky_diffuse + POA_ground_diffuse.

Definitions:
- POA_direct = DNI max(cos(AOI), 0)
- POA_sky_diffuse = DHI (1 + cos(beta))/2
- POA_ground_diffuse = GHI rho_g (1 - cos(beta))/2

Constructed fixtures:
1. horizontal surface beta=0: POA_sky=DHI, POA_ground=0 and, for consistent horizontal irradiance, POA_global=GHI;
2. vertical surface beta=90 with sun normal to surface: POA_direct=DNI, POA_sky=DHI/2, POA_ground=GHI*rho_g/2;
3. rear-facing direct beam: POA_direct=0 by clipping negative cosine of incidence.

Arithmetic acceptance tolerance: **1e-9 W/m2** for synthetic fixtures and **1e-10** for dimensionless direction cosines.

The isotropic model is a validation baseline, not the final claim that Singapore diffuse irradiance is isotropic. Higher-fidelity Perez/Hay-Davies comparisons require their additional inputs and separate validation.
