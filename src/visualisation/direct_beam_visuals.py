"""Human-centred visualisations for the direct-beam ray-tracing phase."""
from __future__ import annotations
from pathlib import Path
import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d.art3d import Poly3DCollection
from src.models.solar_geometry import solar_vector
from src.models.meshes import paraboloid_mesh, cone_mesh, accordion_mesh
from src.models.facets import direct_visibility

PALETTE={"mushroom":"#14866D","cone":"#6657A8","accordion":"#3D6F9E",
         "sun":"#E9A23B","shade":"#68717A","lit":"#F1C453"}

def _facet_value(tri,s,i,mesh):
    mu=max(0.,float(np.dot(tri.normal,s)))
    vis=direct_visibility(i,mesh,s) if mu>0 else 0.
    return mu*vis

def _draw(ax,mesh,s,title):
    vals=np.array([_facet_value(t,s,i,mesh) for i,t in enumerate(mesh)])
    vmax=max(vals.max(),1e-9)
    cmap=plt.colormaps["YlGn"]
    colors=[cmap(0.15+0.8*v/vmax) if v>0 else PALETTE["shade"] for v in vals]
    poly=Poly3DCollection([t.vertices for t in mesh],facecolors=colors,
                          edgecolors="white",linewidths=0.18)
    ax.add_collection3d(poly)
    pts=np.vstack([t.vertices for t in mesh]); span=np.ptp(pts,axis=0); m=max(span.max(),1)
    ctr=(pts.min(axis=0)+pts.max(axis=0))/2
    for setter,c in zip((ax.set_xlim,ax.set_ylim,ax.set_zlim),ctr): setter(c-m*.62,c+m*.62)
    # solar-direction arrow, pointing toward structure
    p=ctr+s*m*.9
    ax.quiver(*p,*(-s),length=m*.55,normalize=True,color=PALETTE["sun"],linewidth=2)
    ax.set_title(title,fontsize=10,fontweight="bold")
    ax.set_xlabel("East–West"); ax.set_ylabel("North–South"); ax.set_zlabel("Height")
    ax.view_init(elev=24,azim=-58)

def make_snapshot(day=80,packing=2.0):
    Path("plots/direct_beam").mkdir(parents=True,exist_ok=True)
    meshes={"Mushroom":paraboloid_mesh(packing,n_r=5,n_phi=24),
            "Cone":cone_mesh(packing,n_r=5,n_phi=24),
            "Accordion":accordion_mesh(packing,folds=4)}
    for hour,label in [(8.0,"Morning"),(12.0,"Noon"),(16.0,"Afternoon")]:
        s=solar_vector(day,hour)
        fig=plt.figure(figsize=(12,4.2),constrained_layout=True)
        for j,(name,mesh) in enumerate(meshes.items(),1):
            ax=fig.add_subplot(1,3,j,projection="3d")
            _draw(ax,mesh,s,f"{name} — {label}")
        fig.suptitle(f"Direct-beam exposure at equal packing ratio Π={packing:g}\n"
                     "Colour intensity = illuminated cosine factor; grey = back-facing or shadowed",
                     fontsize=13,fontweight="bold")
        fig.savefig(f"plots/direct_beam/equal_packing_{label.lower()}.png",dpi=220,bbox_inches="tight")
        plt.close(fig)

if __name__=="__main__": make_snapshot()
