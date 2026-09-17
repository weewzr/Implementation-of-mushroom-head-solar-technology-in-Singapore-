"""Numerical convergence study for the self-shadowing direct-beam model.

This study varies mesh resolution and temporal sampling independently before
geometry rankings are interpreted. It reports a geometric direct-beam ratio,
not annual Singapore electricity yield.

All numerical values below are computational settings (class N in
`docs/code_parameters.md`), not physical constants. Their adequacy is tested
by refinement rather than assumed.
"""
from __future__ import annotations
import csv
from pathlib import Path
import numpy as np
from src.models.solar_geometry import solar_vector
from src.models.meshes import paraboloid_mesh
from src.models.facets import array_direct_power_factor

# Canonical convergence geometry. Pi=2 is an engineering test condition (E),
# selected because it doubles PV area per footprint and was central to the
# founding 1 m2 footprint / 2 m2 PV experiment. It is not an optimum.
PACKING_RATIO = 2.0

# Resolution ladders. Each sequence is deliberately refinement-oriented.
RADIAL_LEVELS = (3, 5, 8, 12)
AZIMUTH_LEVELS = (12, 20, 32, 48)
DAY_STEPS = (20, 10, 5, 2)
TIME_STEPS_H = (1.0, 0.5, 0.25, 0.125)


def geometric_score(n_r: int, n_phi: int, day_step: int, time_step_h: float):
    """Return direct-beam geometric ratio relative to 1 m2 horizontal plane.

    Parameters
    ----------
    n_r : int
        Number of radial mesh divisions (dimensionless count).
    n_phi : int
        Number of azimuthal mesh divisions (dimensionless count).
    day_step : int
        Day-of-year sampling interval (days).
    time_step_h : float
        Intraday apparent-solar-time sampling interval (hours).
    """
    mesh = paraboloid_mesh(PACKING_RATIO, n_r=n_r, n_phi=n_phi)
    score = 0.0
    flat = 0.0
    samples = 0
    for day in range(1, 366, day_step):
        # 06:00--18:00 apparent solar time is a deliberately broad preliminary
        # integration window. Samples below the horizon are discarded by s_z.
        for hour in np.arange(6.0, 18.0 + 0.5*time_step_h, time_step_h):
            s = solar_vector(day, float(hour))
            if s[2] <= 0.0:
                continue
            score += array_direct_power_factor(mesh, s)
            flat += s[2]  # projected area of 1 m2 horizontal reference
            samples += 1
    return score / flat, samples, len(mesh)


def relative_change(current: float, previous: float) -> float:
    """Absolute fractional change between successive refinement levels."""
    return abs(current - previous) / abs(current)


def run_ladder(name, values, fixed, setter):
    rows = []
    previous = None
    for value in values:
        params = dict(fixed)
        setter(params, value)
        score, samples, triangles = geometric_score(**params)
        change = None if previous is None else relative_change(score, previous)
        rows.append({
            "parameter": name,
            "value": value,
            "direct_beam_geometry_ratio": score,
            "relative_change_from_previous": change,
            "solar_samples": samples,
            "triangles": triangles,
        })
        previous = score
    return rows


def run():
    # Fixed values are intermediate settings chosen only to isolate each ladder.
    base = dict(n_r=8, n_phi=32, day_step=5, time_step_h=0.25)
    rows = []
    rows += run_ladder("n_r", RADIAL_LEVELS, base, lambda p, v: p.update(n_r=v))
    rows += run_ladder("n_phi", AZIMUTH_LEVELS, base, lambda p, v: p.update(n_phi=v))
    rows += run_ladder("day_step_days", DAY_STEPS, base, lambda p, v: p.update(day_step=v))
    rows += run_ladder("time_step_hours", TIME_STEPS_H, base, lambda p, v: p.update(time_step_h=v))

    out = Path("data/model_generated/convergence_study.csv")
    out.parent.mkdir(parents=True, exist_ok=True)
    with out.open("w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=rows[0].keys())
        writer.writeheader()
        writer.writerows(rows)
    return rows


if __name__ == "__main__":
    for row in run():
        print(row)
