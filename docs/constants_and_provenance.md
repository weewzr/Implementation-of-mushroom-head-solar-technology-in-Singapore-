# Numerical Constants and Provenance Register

Every numerical coefficient used by the project must appear here or in a more specific model-data record before it is treated as a validated model input.

## Solar-position constants

### Cooper (1969) declination approximation

$$
\delta(n)=23.45^\circ\sin\left[\frac{360^\circ}{365}(284+n)\right].
$$

where:
- $\delta$ is solar declination: angular position of the Sun relative to Earth's equatorial plane (degrees, $^\circ$);
- $n$ is ordinal day of a non-leap year, with $n=1$ on 1 January (dimensionless day index; often described in days);
- $23.45^\circ$ is the approximate amplitude of solar declination used in the Cooper approximation (degrees); it corresponds approximately to Earth's obliquity used by this historical engineering approximation;
- $360^\circ$ is one complete angular cycle (exact angular convention, degrees);
- $365$ is the non-leap-year period assumed by this approximation (days/year); leap years and sub-daily orbital detail are neglected;
- $284$ is the empirical/phase-offset constant in the Cooper (1969) sinusoidal approximation (days in the equation's phase convention), chosen so the simple sinusoid approximates the seasonal phase of solar declination.

**Category:** astronomical/geometrical approximation.  
**Provenance:** the relation is documented by pvlib as a Duffie & Beckman solar-declination expression attributed to Cooper (1969). Duffie & Beckman, *Solar Engineering of Thermal Processes*, 4th ed. (2013), is now included in the bibliography with publisher DOI. The original 1969 Cooper paper metadata has not yet been independently established to the project's bibliography standard, so the project does not invent a direct Cooper BibTeX entry.  
**Use in this project:** educational and preliminary geometry calculations only. A validated yield model should use a higher-accuracy, timestamp-aware solar-position algorithm (e.g. a validated pvlib/SPA implementation) rather than relying solely on this sinusoid.

### Solar hour angle

$$
H=15^\circ\,\mathrm{h}^{-1}\left(t_{\mathrm{solar}}-12\,\mathrm h\right).
$$

where:
- $H$ is solar hour angle (degrees, $^\circ$), negative before apparent solar noon and positive afterward under the adopted convention;
- $t_{\mathrm{solar}}$ is local apparent solar time (hours, h);
- $15^\circ\,\mathrm{h}^{-1}$ is $360^\circ/24\,\mathrm h$, the mean conversion from solar time to hour angle;
- $12\,\mathrm h$ is apparent solar noon in the solar-time coordinate, where $H=0^\circ$.

**Category:** exact angular/time conversion within the mean-solar-time convention, combined with the definition of solar noon.  
**Important:** civil Singapore Standard Time is not automatically equal to local apparent solar time. Longitude and equation-of-time corrections are required when timestamps are civil clock time.

### Solar elevation relation

$$
\sin\alpha=\sin\phi\sin\delta+\cos\phi\cos\delta\cos H.
$$

where:
- $\alpha$ is solar elevation/altitude above the local horizontal (degrees or radians, provided all angular functions use one consistent unit convention);
- $\phi$ is geographic latitude, positive north (degrees or radians);
- $\delta$ is solar declination (same angular unit as $\phi$ and $H$);
- $H$ is solar hour angle (same angular unit as $\phi$ and $\delta$).

**Category:** spherical solar-geometry relation; there are no fitted numerical constants in this equation. The implementation must convert all angles consistently before trigonometric evaluation.

## Project design assumptions

### Canonical design cell
The exploratory canonical experiment uses $A_{foot}=1\,\mathrm{m^2}$, $A_{PV}=2\,\mathrm{m^2}$ and $H_{max}=2\,\mathrm m$. These are **engineering design assumptions**, not Singapore regulations or measured optima. They were introduced in the foundational discussion to create a finite, comparable topology-optimisation problem. Sensitivity studies must vary them.

## Numerical settings
Mesh counts, ray counts, timestep spacing, optimiser population sizes and convergence tolerances are **computational settings**, not physical constants. They must be accompanied by convergence/sensitivity checks before final quantitative conclusions.

## Sources
- pvlib documentation, `declination_cooper69`, attributing the approximation to Cooper (1969) via Duffie & Beckman.
- J. A. Duffie and W. A. Beckman, *Solar Engineering of Thermal Processes*, 4th ed., Wiley, 2013, DOI 10.1002/9781118671603.
- NREL SPA references in `references/references.bib` are the validation target for production solar position.
- The original Cooper (1969) paper metadata remains **Requires verification** before a direct primary-source bibliography entry is added.
