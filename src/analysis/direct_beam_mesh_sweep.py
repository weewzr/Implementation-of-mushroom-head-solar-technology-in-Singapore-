"""Self-shadowing-adjusted direct-beam geometry sweep.

This is deliberately a GEOMETRIC solar-path experiment, not a Singapore
annual-energy prediction. It integrates projected illuminated area over a
sampled annual solar path with equal unit weighting. Weather-weighted DNI is a
later layer. Results are normalized against a 1 m^2 horizontal reference.
"""
from __future__ import annotations
import csv
from pathlib import Path
import numpy as np
from src.models.solar_geometry import solar_vector
from src.models.meshes import paraboloid_mesh, cone_mesh, accordion_mesh
from src.models.facets import array_direct_power_factor

PACKING=(1.0,1.25,1.5,2.0,2.5,3.0,4.0)
DAYS=range(1,366,10)
HOURS=np.arange(6.25,17.76,0.5)


def annual_geometric_score(mesh):
    score=0.0
    flat=0.0
    samples=0
    for day in DAYS:
        for hour in HOURS:
            s=solar_vector(day,hour)
            if s[2] <= 0:
                continue
            score += array_direct_power_factor(mesh,s)
            flat += s[2]  # 1 m^2 horizontal reference
            samples += 1
    return score/flat, samples


def run():
    rows=[]
    for pi in PACKING:
        geometries={
            "paraboloid": paraboloid_mesh(pi,n_r=5,n_phi=20),
            "cone": cone_mesh(pi,n_r=5,n_phi=20),
            "accordion": accordion_mesh(pi,folds=4),
        }
        for name,mesh in geometries.items():
            score,n=annual_geometric_score(mesh)
            rows.append({"geometry":name,"packing_ratio":pi,
                         "direct_beam_geometry_ratio":score,
                         "samples":n})
    out=Path("data/model_generated/direct_beam_mesh_sweep.csv")
    out.parent.mkdir(parents=True,exist_ok=True)
    with out.open("w",newline="") as f:
        w=csv.DictWriter(f,fieldnames=rows[0].keys()); w.writeheader(); w.writerows(rows)
    return rows

if __name__ == "__main__":
    for r in run(): print(r)
