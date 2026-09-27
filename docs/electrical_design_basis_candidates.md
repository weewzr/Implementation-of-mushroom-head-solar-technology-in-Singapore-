# Electrical Design-Basis Candidate Register

Status: source-selection work for Section 15. No module is yet selected as the universal or final project basis.

## Parameter contract
| Parameter | Symbol | Unit | Physical meaning | Required source | Selected value | Uncertainty/sensitivity | Geometry dependence | Status |
|---|---|---:|---|---|---|---|---|---|
| Reference PV technology/module | -- | -- | Technology and product basis used to convert irradiance to electricity | Current manufacturer datasheet plus independent/authoritative context | unset | compare at least rigid c-Si vs credible non-rigid alternative if used | technology choice can differ by architecture | OPEN |
| STC efficiency | eta_STC | -- | Rated conversion efficiency at STC | Manufacturer datasheet/certified basis | unset | sensitivity required | no for a fixed selected technology | OPEN |
| Module active/physical area | A_m | m2 | Area associated with module rating and layout | Manufacturer dimensions/datasheet | unset | low uncertainty; definition must be explicit | no | OPEN |
| Maximum-power temperature coefficient | gamma_P | 1/degC | Fractional Pmax change with module temperature | Manufacturer datasheet | unset | sensitivity required in Singapore climate | no | OPEN |
| Reference temperature | T_ref | degC | Temperature at which rated coefficient is referenced | STC/datasheet | unset | normally definitional | no | OPEN |
| NOCT/NMOT or thermal basis | theta_T | model-specific | Parameters mapping POA/weather to module temperature | Datasheet or validated thermal literature | unset | thermal-model sensitivity required | mounting/geometry can affect thermal state | OPEN |
| Ambient temperature | T_a(t) | degC | Environmental thermal input | Time-correlated weather dataset | NASA POWER path available; electrical use not yet released | weather/model uncertainty | no, common weather | EVIDENCE AVAILABLE |
| Wind speed | u(t) | m/s | Convective thermal-model input if chosen model requires it | Time-correlated weather dataset | NASA POWER WS10 available; use depends on thermal model | sensitivity/model-form uncertainty | local flow can become geometry-dependent later | EVIDENCE AVAILABLE |
| DC mismatch loss | L_mis | -- | Loss from nonuniform module/string operating points | authoritative performance model/literature + declared assumption | unset | sensitivity required | potentially geometry-dependent through shading | OPEN |
| DC wiring loss | L_wire,dc | -- | Resistive DC collection loss | engineering design/model source | unset | sensitivity if material | architecture-dependent routing later | OPEN |
| Soiling loss | L_soil | -- | Optical loss from deposited dirt | Singapore-relevant evidence if included | unset | sensitivity required if included | potentially orientation-dependent | OPEN |
| Availability loss | L_avail | -- | Downtime/operational availability | explicit reliability/system basis | unset | scenario sensitivity | architecture-dependent later | OPEN |
| Inverter conversion | eta_inv(P) | -- | DC-to-AC conversion efficiency/curve | selected inverter datasheet or authoritative model | unset | sensitivity/part-load treatment | common if same inverter basis | OPEN |
| Other AC/system losses | L_sys | -- | Explicit residual BOS/system losses not double-counted above | component-by-component sourced basis | unset | sensitivity required | declare case-by-case | OPEN |
| Auxiliary/deployment/tracking energy | E_aux | kWh/yr | Electricity consumed by movement/control/auxiliaries | mechanism-specific calculation | zero only for genuinely fixed cases; otherwise unset | required | yes | OPEN |

## Candidate source set

### Rigid high-efficiency crystalline-silicon baseline candidate
Canadian Solar TOPHiKu6 is a current manufacturer family suitable for screening a conventional rigid c-Si design basis. The manufacturer's current product page reports module efficiency up to 23.0% and Pmax temperature coefficient -0.29%/degC. These are family-level screening values, not yet the selected project values; a specific module datasheet must be chosen before implementation.
Source: https://www.canadiansolar.com/na/tophiku6/

### Thin-film comparison candidate
First Solar Series 6 Plus provides a credible commercial CdTe thin-film comparison basis, but it is a large framed utility module and must not be described as a flexible origami sheet. The current manufacturer page reports up to 480 W and up to 19.0% efficiency; the Series 6 Plus user guide reports Pmax temperature coefficient -0.32%/degC for the referenced product generation. First Solar also publishes South-East-Asia-specific technical material. This candidate is useful for technology sensitivity, not proof of foldability.
Sources:
- https://www.firstsolar.com/en/Products/Series-6
- https://www.firstsolar.com/-/media/First-Solar/Technical-Documents/User-Guides/Series-6-User-Guide.pdf?dl=1
- https://www.firstsolar.com/en-AU/lp/SEA

### System-model source
NREL PVWatts is an authoritative baseline reference for grid-connected PV performance-model architecture and system-loss/inverter treatment. Its published model documentation separates module type, system losses, DC/AC ratio and inverter efficiency and performs time-step irradiance -> temperature-adjusted DC -> losses -> inverter AC calculations. Project numerical defaults will not be copied automatically; each adopted parameter must be declared and justified.
Sources:
- https://pvwatts.nrel.gov/
- https://pvwatts.nrel.gov/downloads/pvwattsv5.pdf
- https://sam.nrel.gov/photovoltaic/pv-publications.html

## Selection rule for Continue #184+
Select one specific rigid c-Si module datasheet as the first fixed-geometry electrical basis. Keep CdTe as a technology-sensitivity candidate unless a specific report question requires it. Do not select a flexible/deployable PV product until authoritative bend-radius/flex-cycle/electrical data exist. Do not use the historical 23% placeholder merely because a current family happens to advertise a similar maximum efficiency.
