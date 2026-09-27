# First Electrical Rust Kernel Specification

Status: specification only at Continue #184 rotation boundary. Do not treat as implemented.

## Scope
A geometry-agnostic electrical conversion kernel consumes time-step plane-of-array irradiance and environmental/design-basis inputs. It does not alter the frozen irradiance solver.

Inputs per timestep: `G_poa_W_m2`, ambient/weather inputs required by the selected module-temperature model, timestep duration, and a validated `ElectricalDesignBasis`.

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
- auxiliary load uses explicit net-import semantics rather than silently clipping physical accounting;
- time-integration consistency for constant-power fixtures;
- units documented at every public field/function boundary.

## Architecture
The kernel must remain geometry-agnostic. Frozen flat/paraboloid/hemisphere/faceted/folded irradiance outputs and later validated origami meshes may consume the same electrical API. No geometry-specific efficiency fudge factor is allowed.

## Evidence gate
Implementation may begin only after one rigid c-Si module/design basis is provisionally selected with traceable manufacturer data and the thermal-model choice is explicit. Whole-crate tests and an early audit are required before any annual electrical comparison is promoted.
