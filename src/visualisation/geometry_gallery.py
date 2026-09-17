"""Render equal-packing candidate geometries using the project visual language.

Output is conceptual/model geometry, not measured performance.  The script uses
Matplotlib 3-D only because spatial geometry is the subject.
"""
from pathlib import Path
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d.art3d import Poly3DCollection
from src.models.meshes import paraboloid_mesh, cone_mesh, accordion_mesh

OUT=Path("plots"); OUT.mkdir(exist_ok=True)
PALETTE={"mushroom":"#009E73","cone":"#7B61A8","accordion":"#0072B2",
         "sun":"#E69F00","structure":"#566573"}

def add_mesh(ax, triangles, color):
    verts=[t.vertices for t in triangles]
    pc=Poly3DCollection(verts, facecolor=color, edgecolor="white",
                        linewidth=0.25, alpha=0.92)
    ax.add_collection3d(pc)
    xyz=[v for t in triangles for v in t.vertices]
    xs=[p[0] for p in xyz]; ys=[p[1] for p in xyz]; zs=[p[2] for p in xyz]
    ax.set_xlim(min(xs),max(xs)); ax.set_ylim(min(ys),max(ys)); ax.set_zlim(0,max(zs+[0.1]))
    ax.set_box_aspect((1,1,0.75)); ax.view_init(elev=24, azim=-55)
    ax.set_xlabel("East–west [m]"); ax.set_ylabel("North–south [m]"); ax.set_zlabel("Height [m]")
    ax.grid(False)


def main(pi=2.0):
    configs=[("Paraboloidal mushroom",paraboloid_mesh(pi),PALETTE["mushroom"]),
             ("Cone",cone_mesh(pi),PALETTE["cone"]),
             ("Accordion",accordion_mesh(pi),PALETTE["accordion"])]
    fig=plt.figure(figsize=(12,4.4),layout="constrained")
    for i,(title,mesh,color) in enumerate(configs,1):
        ax=fig.add_subplot(1,3,i,projection="3d")
        add_mesh(ax,mesh,color)
        ax.set_title(f"{title}\n$\\Pi={pi:g}$",fontsize=11,fontweight="bold")
    fig.suptitle("Equal-packing candidate geometries",fontsize=15,fontweight="bold")
    fig.text(0.5,0.01,"Conceptual triangulated geometry; equal PV-area/footprint ratio, not an energy ranking.",ha="center",fontsize=9)
    fig.savefig(OUT/f"geometry_gallery_pi_{pi:g}.png",dpi=220,bbox_inches="tight")
    plt.close(fig)

if __name__=="__main__": main()
