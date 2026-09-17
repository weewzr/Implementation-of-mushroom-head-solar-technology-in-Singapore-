"""Generate analytical/simulated fixed-paraboloid diagnostic figures.

No measured Singapore data are used here. Figure colours follow the repository
semantic design system: baseline=slate/deep blue; mushroom candidates=teal/green;
uncertainty/convergence=grey. Line styles and markers provide greyscale redundancy.
"""
from __future__ import annotations

from pathlib import Path
import math
import sys

import matplotlib.pyplot as plt
import numpy as np

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
from src.models.paraboloid_analytic import absolute_gain, directional_response, horizontal_response

OUT = ROOT / "figures" / "paraboloid_analytic"
OUT.mkdir(parents=True, exist_ok=True)

# Category-4 design samples for visual exploration only; not claimed optima.
H_VALUES = (0.0, 0.25, 0.5, 1.0)
# Category-5 plotting grid. 181 points gives 0.5-degree spacing over 0..90 deg.
THETA_DEG = np.linspace(0.0, 90.0, 181)
THETA_RAD = np.deg2rad(THETA_DEG)
# Semantic colours specified by project design requirements.
BASELINE = "#334155"
CANDIDATES = ("#0f766e", "#059669", "#65a30d")
STYLES = ("--", "-.", ":")


def save(fig, name: str) -> None:
    fig.tight_layout()
    fig.savefig(OUT / f"{name}.png", dpi=200, bbox_inches="tight")
    fig.savefig(OUT / f"{name}.pdf", bbox_inches="tight")
    plt.close(fig)


def plot_cross_sections() -> None:
    rho = np.linspace(-1.0, 1.0, 401)
    fig, ax = plt.subplots(figsize=(7.2, 4.5))
    ax.axhline(0.0, color=BASELINE, linewidth=1.2, label="horizontal baseline")
    for h, colour, style in zip(H_VALUES[1:], CANDIDATES, STYLES):
        z = h * (1.0 - rho * rho)
        ax.plot(rho, z, color=colour, linestyle=style, linewidth=2.0, label=fr"$H/R={h:g}$")
    ax.set_xlabel(r"dimensionless radius $x/R$")
    ax.set_ylabel(r"dimensionless elevation $z/R$")
    ax.set_title("Paraboloidal candidate cross-sections (conceptual geometry)")
    ax.legend()
    ax.grid(alpha=0.25)
    save(fig, "cross_sections")


def plot_response() -> None:
    fig, ax = plt.subplots(figsize=(7.2, 4.5))
    baseline = [horizontal_response(t) for t in THETA_RAD]
    ax.plot(THETA_DEG, baseline, color=BASELINE, linewidth=2.2, label="horizontal fixed")
    for h, colour, style in zip(H_VALUES[1:], CANDIDATES, STYLES):
        values = [directional_response(t, h) for t in THETA_RAD]
        ax.plot(THETA_DEG, values, color=colour, linestyle=style, linewidth=2.0, label=fr"paraboloid $H/R={h:g}$")
    ax.set_xlabel(r"solar zenith angle $\theta_z$ (deg)")
    ax.set_ylabel(r"directional response $C=A_{\rm eff}/(\pi R^2)$ (-)")
    ax.set_title("Fixed-geometry direct-beam directional response")
    ax.legend()
    ax.grid(alpha=0.25)
    save(fig, "directional_response")


def plot_gain() -> None:
    fig, ax = plt.subplots(figsize=(7.2, 4.5))
    ax.axhline(0.0, color=BASELINE, linewidth=1.4, label="equal to horizontal")
    for h, colour, style in zip(H_VALUES[1:], CANDIDATES, STYLES):
        values = [absolute_gain(t, h) for t in THETA_RAD]
        ax.plot(THETA_DEG, values, color=colour, linestyle=style, linewidth=2.0, label=fr"$H/R={h:g}$")
    ax.set_xlabel(r"solar zenith angle $\theta_z$ (deg)")
    ax.set_ylabel(r"absolute response difference $\Delta C$ (-)")
    ax.set_title("Absolute direct-beam difference from horizontal baseline")
    ax.legend()
    ax.grid(alpha=0.25)
    save(fig, "absolute_gain")


def plot_convergence() -> None:
    # Category-5 convergence sequence: powers of two suit composite Simpson quadrature.
    ns = np.array([32, 64, 128, 256, 512, 1024, 2048], dtype=int)
    theta = math.radians(70.0)  # category-4 diagnostic case, chosen in partial-illumination regime.
    h = 0.5  # category-4 diagnostic geometry, not an optimum.
    reference = directional_response(theta, h, int(ns[-1]))
    errors = np.array([abs(directional_response(theta, h, int(n)) - reference) for n in ns[:-1]])
    fig, ax = plt.subplots(figsize=(7.2, 4.5))
    ax.loglog(ns[:-1], errors, marker="o", color="#6b7280", linewidth=1.8)
    ax.set_xlabel("radial Simpson intervals, $N_r$ (-)")
    ax.set_ylabel(r"absolute difference from $N_r=2048$ reference (-)")
    ax.set_title(r"Quadrature convergence diagnostic: $H/R=0.5$, $\theta_z=70^\circ$")
    ax.grid(True, which="both", alpha=0.25)
    save(fig, "quadrature_convergence")


if __name__ == "__main__":
    plot_cross_sections()
    plot_response()
    plot_gain()
    plot_convergence()
