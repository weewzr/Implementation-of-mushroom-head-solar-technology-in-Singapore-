"""Geometry-only packing sweep.

This stage deliberately does NOT claim annual Singapore yield. It maps equal packing
ratios to the geometry required by accordion, cone and paraboloid families. These
outputs become inputs to the later time-resolved irradiance/ray-tracing model.
"""
import csv
from pathlib import Path
from src.models.candidate_geometries import (
    accordion_from_packing, cone_from_packing, paraboloid_from_packing
)

PACKING = [1.0, 1.25, 1.5, 2.0, 2.5, 3.0, 4.0]

def rows():
    out=[]
    for pi in PACKING:
        for fn in (accordion_from_packing, cone_from_packing, paraboloid_from_packing):
            g=fn(pi)
            out.append({
                "geometry":g.name,
                "packing_ratio":g.packing_ratio,
                "characteristic_tilt_deg":g.characteristic_tilt_deg,
                "height_over_width":g.height_over_width,
                "notes":g.notes,
            })
    return out

def main():
    outdir=Path("data/generated")
    outdir.mkdir(parents=True, exist_ok=True)
    path=outdir/"packing_geometry_sweep.csv"
    r=rows()
    with path.open("w", newline="") as f:
        w=csv.DictWriter(f, fieldnames=r[0].keys())
        w.writeheader(); w.writerows(r)
    print(f"wrote {path}")

if __name__ == "__main__":
    main()
