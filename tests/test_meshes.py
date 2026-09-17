import math
from src.models.meshes import paraboloid_mesh, cone_mesh, accordion_mesh, mesh_area


def relerr(a,b):
    return abs(a-b)/max(abs(b),1e-12)


def test_accordion_equal_packing():
    for pi in [1.0,1.25,1.5,2.0,3.0]:
        assert relerr(mesh_area(accordion_mesh(pi,folds=5)),pi) < 1e-10


def test_cone_mesh_converges_to_target_area():
    for pi in [1.25,2.0,3.0]:
        # polygonal circumference causes a small discretisation error
        assert relerr(mesh_area(cone_mesh(pi,n_r=10,n_phi=128)),pi) < 2e-3


def test_paraboloid_mesh_converges_to_target_area():
    for pi in [1.25,2.0,3.0]:
        assert relerr(mesh_area(paraboloid_mesh(pi,n_r=32,n_phi=128)),pi) < 3e-3


def test_all_exposed_normals_are_upward():
    for mesh in [accordion_mesh(2),cone_mesh(2),paraboloid_mesh(2)]:
        assert min(t.normal[2] for t in mesh) >= -1e-12
