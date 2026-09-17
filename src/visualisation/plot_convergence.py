"""Plot numerical convergence using the project visual design system.

Input: data/model_generated/convergence_study.csv
Output: plots/convergence_study.png

The figure is a numerical-method diagnostic, not a solar-yield result.
"""
from pathlib import Path
import csv
import matplotlib.pyplot as plt

INPUT = Path("data/model_generated/convergence_study.csv")
OUTPUT = Path("plots/convergence_study.png")

LABELS = {
    "n_r": r"Radial divisions $N_r$ (count)",
    "n_phi": r"Azimuthal divisions $N_\phi$ (count)",
    "day_step_days": r"Day sampling $\Delta n$ (days)",
    "time_step_hours": r"Time sampling $\Delta t$ (h)",
}


def load():
    rows = []
    with INPUT.open() as f:
        for r in csv.DictReader(f):
            r["value"] = float(r["value"])
            r["direct_beam_geometry_ratio"] = float(r["direct_beam_geometry_ratio"])
            rows.append(r)
    return rows


def main():
    rows = load()
    fig, axes = plt.subplots(2, 2, figsize=(11, 8), constrained_layout=True)
    axes = axes.ravel()
    for ax, parameter in zip(axes, LABELS):
        subset = [r for r in rows if r["parameter"] == parameter]
        x = [r["value"] for r in subset]
        y = [r["direct_beam_geometry_ratio"] for r in subset]
        ax.plot(x, y, marker="o", linewidth=2)
        ax.set_xlabel(LABELS[parameter])
        ax.set_ylabel(r"Geometric direct-beam ratio $R_{\rm beam,geom}$ (-)")
        ax.grid(alpha=0.2)
        if parameter in ("day_step_days", "time_step_hours"):
            ax.invert_xaxis()  # finer sampling appears to the right visually
    fig.suptitle("Numerical convergence diagnostic — paraboloidal mushroom at $\\Pi=2$", fontsize=15)
    fig.text(0.5, 0.005,
             "Model-generated numerical diagnostic; no measured irradiance and no annual-energy claim.",
             ha="center", fontsize=9)
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(OUTPUT, dpi=220, bbox_inches="tight")


if __name__ == "__main__":
    main()
