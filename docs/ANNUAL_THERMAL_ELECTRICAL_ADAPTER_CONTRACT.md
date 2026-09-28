# Annual Thermal-Electrical Adapter Contract — prepared at Continue #191

Status: preparation contract only. No annual geometry electrical result is authorized.

## Canonical input row
Future annual adapter must bind, by exact canonical UTC timestamp:
- timestamp_utc_s;
- geometry_id and resource_contract;
- accepted/converged upstream POA irradiance [W/m2];
- NASA POWER DEVELOPMENT_NOT_SERIS ambient T2M [degC];
- explicit dt_hours;
- NMOT scenario [39,42,45 degC];
- provisional rigid-PV electrical design-basis identifier;
- explicit auxiliary-power scenario.

## Join rules
Exact equality on canonical absolute UTC timestamp. No nearest-neighbour matching, row-index joining, implicit UTC/local conversion or silent row dropping. Duplicate timestamps, missing irradiance, missing ambient temperature, non-finite fields or inconsistent timestep must fail the annual adapter.

NASA POWER hourly timestamps are start-of-hour whole-hour averages. Frozen irradiance outputs used for annual coupling must represent the same canonical interval semantics. #192 must prove this alignment before any annual integration.

## Output schema
At minimum: timestamp_utc_s, geometry_id, resource_contract, nmot_c, poa_w_m2, ambient_temp_c, module_temp_c, efficiency, ideal_dc_w, delivered_dc_w, ac_w, auxiliary_power_w, net_w, dt_hours, ac_energy_wh, net_energy_wh, status, provenance identifiers.

## Energy units
Power remains W. Timestep duration is hours. Energy is Wh by explicit multiplication. Conversion to kWh is a reporting transform only and must be declared.

## Night handling
Accepted G_POA=0 is retained as a row. Gross DC/AC generation is zero; auxiliary semantics remain explicit and net power may be negative. Night rows are not dropped.

## NMOT sensitivity
Run identical accepted inputs/design basis under 39, 42 and 45 degC NMOT scenarios. Scenario differences are model sensitivity, not confidence intervals.

## Electrical basis
Bind one canonical provisional Canadian Solar CS6.2-48TM-460H design-basis object. Do not duplicate eta_ref, area, gamma_P or T_ref in adapter constants. Production loss/inverter/auxiliary assumptions must be separately sourced before headline annual net-energy promotion.

## Artifact requirements for #192
Create adapter-level deterministic evidence proving exact timestamp joins, rejection of duplicate/missing/misaligned rows, one-input-to-one-output preservation, units/schema, NMOT sensitivity propagation and provenance fields. Do not yet emit headline annual geometry kWh.
