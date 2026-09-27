# First Electrical Rust Kernel Specification

Status: Audit 61 reviewed specification; implementation is authorized for Continue #186 after provisional rigid-PV design-basis selection. Do not treat as implemented.

## Scope
A geometry-agnostic electrical conversion kernel consumes time-step plane-of-array irradiance and environmental/design-basis inputs. It does not alter the frozen irradiance solver.

Inputs per timestep: `G_poa_W_m2`, ambient/weather inputs required by the selected module-temperature model, timestep duration, and a validated `ElectricalDesignBasis`. The initial thermal basis is the selected module's declared NMOT basis; thermal-model choice must remain explicit and replaceable.

Chain:
1. POA irradiance -> module temperature `T_module`.
2. `T_module` -> temperature-adjusted efficiency.
3. irradiance x active PV area x efficiency -> ideal DC power.
4. apply explicit mismatch and DC wiring losses.
5. inverter conversion -> AC power.
6. apply explicitly separated residual AC/system losses.
7. subtract auxiliary/tracking/deployment power using explicit net-import semantics.
8. integrate AC and net power over time to annual energy.

Definitions:
`E_AC,annual = sum_t P_AC(t) * dt`.
`E_net,annual = E_AC,annual - E_aux,annual`.

## Required API validation
- zero irradiance -> zero generated DC/AC power;
- negative irradiance rejected explicitly;
- reference temperature -> reference efficiency;
- negative Pmax temperature coefficient + hotter module -> lower efficiency/power;
- efficiency finite and nonnegative;
- all fractional losses bounded in [0,1);
- zero losses recover ideal DC limit;
- inverter output finite and nonnegative;
- auxiliary load uses explicit net-import semantics: generated AC power remains nonnegative, while net power/energy may be negative when auxiliary demand exceeds generation; do not silently clip net import;
- time-integration consistency for constant-power fixtures;
- units documented at every public field/function boundary.

## Architecture
The kernel must remain geometry-agnostic. Frozen flat/paraboloid/hemisphere/faceted/folded irradiance outputs and later validated origami meshes may consume the same electrical API. No geometry-specific efficiency fudge factor is allowed.

## Evidence gate
Implementation may begin only after one rigid c-Si module/design basis is provisionally selected with traceable manufacturer data and the thermal-model choice is explicit. Whole-crate tests and an early audit are required before any annual electrical comparison is promoted.


## Audit 61 implementation authorization checks
PASS as a specification:
- geometry-agnostic chain remains POA -> module temperature -> temperature-adjusted efficiency -> DC -> declared DC losses -> inverter -> residual AC/system losses -> auxiliary -> net -> integration;
- parameter validation explicitly covers negative irradiance, finite/nonnegative efficiency, bounded loss fractions, inverter output and units;
- zero irradiance, reference-temperature, hot-module/negative-gamma, ideal-zero-loss and constant-power integration fixtures are required;
- net-import semantics are explicit: net power may be negative while generated DC/AC power may not;
- the frozen irradiance solver is an upstream consumer contract and must not be modified by the electrical kernel.

Continue #186 is authorized to implement this first tested Rust kernel against the provisional rigid-PV basis. Annual geometry comparisons remain unreleased until whole-crate tests and the relevant evidence/audit gate pass.
