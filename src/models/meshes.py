"""Triangulated geometry generators for equal-resource optical comparisons.

Coordinate convention: x east, y north, z up. Triangle vertex ordering is
chosen to give outward/upward normals on exposed PV faces.
"""
from __future__ import annotations
import math
import numpy as np
from .facets import Triangle
from .candidate_geometries import paraboloid_k_from_packing


def _tri(a,b,c):
    return Triangle(np.array([a,b,c], dtype=float))


def paraboloid_mesh(packing_ratio: float, footprint_area: float=1.0,
                     n_r: int=8, n_phi: int=32):
    """Triangulate z=h(1-r^2/R^2) over a circular footprint."""
    R=math.sqrt(footprint_area/math.pi)
    k=paraboloid_k_from_packing(packing_ratio)
    h=k*R
    tris=[]
    # centre fan
    r1=R/n_r
    z0=h
    z1=h*(1-(r1/R)**2)
    c=np.array([0.,0.,z0])
    for j in range(n_phi):
        p0=2*math.pi*j/n_phi; p1=2*math.pi*(j+1)/n_phi
        a=[r1*math.cos(p0),r1*math.sin(p0),z1]
        b=[r1*math.cos(p1),r1*math.sin(p1),z1]
        tris.append(_tri(c,a,b))
    # annuli
    for ir in range(1,n_r):
        ra=R*ir/n_r; rb=R*(ir+1)/n_r
        za=h*(1-(ra/R)**2); zb=h*(1-(rb/R)**2)
        for j in range(n_phi):
            p0=2*math.pi*j/n_phi; p1=2*math.pi*(j+1)/n_phi
            a=[ra*math.cos(p0),ra*math.sin(p0),za]
            b=[rb*math.cos(p0),rb*math.sin(p0),zb]
            c2=[rb*math.cos(p1),rb*math.sin(p1),zb]
            d=[ra*math.cos(p1),ra*math.sin(p1),za]
            tris.extend([_tri(a,b,c2),_tri(a,c2,d)])
    return tris


def cone_mesh(packing_ratio: float, footprint_area: float=1.0,
              n_r: int=8, n_phi: int=32):
    """Triangulate a convex cone with equal horizontal circular footprint."""
    R=math.sqrt(footprint_area/math.pi)
    h=R*math.sqrt(max(0.,packing_ratio**2-1.))
    tris=[]
    apex=np.array([0.,0.,h])
    r1=R/n_r; z1=h*(1-r1/R)
    for j in range(n_phi):
        p0=2*math.pi*j/n_phi; p1=2*math.pi*(j+1)/n_phi
        a=[r1*math.cos(p0),r1*math.sin(p0),z1]
        b=[r1*math.cos(p1),r1*math.sin(p1),z1]
        tris.append(_tri(apex,a,b))
    for ir in range(1,n_r):
        ra=R*ir/n_r; rb=R*(ir+1)/n_r
        za=h*(1-ra/R); zb=h*(1-rb/R)
        for j in range(n_phi):
            p0=2*math.pi*j/n_phi; p1=2*math.pi*(j+1)/n_phi
            a=[ra*math.cos(p0),ra*math.sin(p0),za]
            b=[rb*math.cos(p0),rb*math.sin(p0),zb]
            c=[rb*math.cos(p1),rb*math.sin(p1),zb]
            d=[ra*math.cos(p1),ra*math.sin(p1),za]
            tris.extend([_tri(a,b,c),_tri(a,c,d)])
    return tris


def accordion_mesh(packing_ratio: float, footprint_width: float=1.0,
                    footprint_length: float=1.0, folds: int=4):
    """Symmetric triangular-wave roof spanning a rectangular footprint.

    The true surface/footprint ratio is sec(beta)=packing_ratio. Ridges run
    north-south; slopes alternate east/west. No vertical end walls are PV.
    """
    if packing_ratio < 1:
        raise ValueError("packing ratio must be >= 1")
    beta=math.acos(1/packing_ratio)
    dx=footprint_width/(2*folds)
    rise=dx*math.tan(beta)
    xs=np.linspace(-footprint_width/2,footprint_width/2,2*folds+1)
    zs=np.array([rise if i%2 else 0. for i in range(len(xs))])
    y0=-footprint_length/2; y1=footprint_length/2
    tris=[]
    for i in range(len(xs)-1):
        a=[xs[i],y0,zs[i]]; b=[xs[i+1],y0,zs[i+1]]
        c=[xs[i+1],y1,zs[i+1]]; d=[xs[i],y1,zs[i]]
        # Choose ordering based on slope so normals point upward.
        t1=_tri(a,b,c); t2=_tri(a,c,d)
        if t1.normal[2] < 0: t1=_tri(a,c,b)
        if t2.normal[2] < 0: t2=_tri(a,d,c)
        tris.extend([t1,t2])
    return tris


def mesh_area(triangles):
    return sum(t.area for t in triangles)
