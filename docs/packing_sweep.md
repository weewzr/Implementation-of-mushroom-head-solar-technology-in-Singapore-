# Equal-Packing Geometry Sweep

## Purpose
The next stage compares candidate geometries at the **same PV packing ratio** rather than comparing arbitrary shapes. This avoids confusing a geometry benefit with simply using more PV material.

Define

$$\Pi=A_{PV}/A_{land}.$$

The sweep uses

$$\Pi\in\{1,1.25,1.5,2,2.5,3,4\}.$$

## Analytical mappings
### Accordion
For a symmetric folded surface with facet tilt $\beta$,

$$\Pi=\sec\beta,$$

so

$$\beta=\cos^{-1}(1/\Pi).$$

At $\Pi=2$, $\beta=60^\circ$.

### Cone
For cone radius $R$ and height $h$,

$$\Pi=\frac{\pi R\sqrt{R^2+h^2}}{\pi R^2}=\sqrt{1+(h/R)^2}.$$

Thus

$$h/R=\sqrt{\Pi^2-1},$$

and the cone slope angle also satisfies $\beta=\cos^{-1}(1/\Pi)$.

### Paraboloid
For

$$z=h(1-r^2/R^2),\qquad k=h/R,$$

$$\Pi=\frac{(1+4k^2)^{3/2}-1}{6k^2}.$$

This relation is monotonic and is inverted numerically by bisection in `candidate_geometries.py`.

## Why this stage does not calculate energy yet
A geometry-only sweep is deliberately separated from annual yield. A defensible energy comparison requires time-correlated DNI/DHI or GHI/DHI, anisotropic diffuse transposition and 3-D self-shadowing. Applying a single annual diffuse fraction to all timesteps would create false precision.

## Required next model
For every geometry and timestep:

$$G_{beam,i}=DNI(t)V_i(t)[\mathbf n_i\cdot\mathbf s(t)]_+,$$

then add anisotropic sky diffuse and rear irradiance. Integrate temperature-corrected electrical output over the year. Only then calculate

$$M_L(\Pi)=E_{geometry}(\Pi)/E_{baseline}$$

and

$$\eta_{pack}(\Pi)=M_L(\Pi)/\Pi.$$

## Reproduction
```bash
PYTHONPATH=. python src/analysis/packing_sweep.py
pytest -q
```

The generated CSV belongs in `data/generated/packing_geometry_sweep.csv` and is model output, not measured data.
