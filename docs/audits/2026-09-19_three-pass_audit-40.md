# Mandatory Audit 40 — Continue #126

Date: 2026-09-19
Scope: artifacts and execution evidence actually produced during Continues #124-125.

## Evidence inspected

- Canonical SERIS acquisition documentation and pending manifest.
- Fail-closed SERIS raw-data/QC runner.
- Rust weather ingestion/QC foundation.
- NREL SPA Appendix A.5 reference fixture and 0.001 deg implementation acceptance tolerance.
- GHI/DHI/DNI closure and isotropic POA Rust module/tests.
- GitHub Actions Rust execution evidence for commit 957ea3f1baaaae045cf69441b9b687a074fdd6fa.
- Current LaTeX-report workflow state.

## Findings

### PASS — execution integrity
Rust execution evidence run 35421371938 completed successfully. Canonical tests, release executable, final repository-state capture and evidence upload all passed.

### PASS — irradiance arithmetic foundation
The implemented closure diagnostic preserves the physical identity GHI = DHI + DNI cos(theta_z) above the horizon without silently repairing measurements. Synthetic exact fixtures exercise zenith, 60-degree zenith and horizon cases.

The isotropic POA baseline separately computes direct, sky-diffuse and ground-reflected components, clips rear-facing beam incidence, and has exact constructed horizontal/vertical fixtures. It is correctly labelled a baseline rather than a claim that Singapore diffuse irradiance is isotropic.

### PASS — SPA reference definition, NOT implementation validation
The NREL SPA Appendix A.5 published output is pinned with an explicit 0.001 deg project acceptance tolerance. This establishes a validation target and convention adapter.

However, the repository does not yet contain an SPA-equivalent civil-time solar-position implementation that computes the Appendix A.5 case and demonstrates agreement within tolerance. A test that merely checks the stored published constants is not algorithm validation.

### BLOCKED — canonical Singapore annual data
SERIS is selected as the preferred canonical measured irradiance source and the repository correctly refuses to invent historical access/licence metadata. No authorised historical SERIS annual time series has been acquired. Therefore ingestion/QC has not run on the canonical measured dataset and no measured annual Singapore baseline can yet be generated.

### NON-BLOCKING FOR SCIENCE — report compiler
The latest LaTeX workflow still fails during compilation. This remains a documentation-build defect, but it does not invalidate the Rust validation evidence above and must not consume baseline-science passes unless it prevents release of a required report.

## Baseline Model Phase authorization decision

**NOT YET AUTHORIZED for trustworthy annual Singapore results.**

Two release gates remain:
1. acquire an authorised annual Singapore irradiance dataset with resolved provenance/licence/timestamps/site metadata and pass it through Rust ingestion/QC;
2. implement or bind an authoritative SPA/reference solar-position computation and demonstrate the Appendix A.5 benchmark within the documented angular tolerance.

Once both gates pass, Baseline Model Phase is authorized without another broad foundation rewrite. The first baseline should use the validated solar-position path, measured/canonical irradiance inputs, closure QC, and the tested POA transformation with assumptions explicitly labelled.

No annual-yield number is promoted by this audit.
