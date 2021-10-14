# Arcade roadmap

## 3D PGA
- [X] Projective Geometric Algebra module

## Parametric representations

- [ ] Parametric curve union
  - [ ] Line
  - [ ] Circle
  - [ ] Ellipse
  - [ ] Hyperbola
  - [ ] Parabola
  - [ ] Intersection curve
  - [ ] NURBS curve
  - [ ] Surface-embedded curve
  - [ ] Offset curve
  - [ ] Rolling ball blend edge (maybe)
- [ ] Parametric surface union
  - [ ] Plane
  - [ ] Cylinder
  - [ ] Cone
  - [ ] Sphere
  - [ ] Torus
  - [ ] Linear extrusion (swept surface)
  - [ ] Surface of revolution (spun surface)
  - [ ] NURBS surface
  - [ ] Offset surface
  - [ ] Rolling ball blend surface

## CAD Kernel primitives

- [ ] Vertex
- [ ] Edge (basis curve, endpoint parameters & vertices (optional))
- [ ] Wire (sequence of edges connected by vertices, open or closed)
- [ ] Face (basis surface, bounding wires (optional))
- [ ] Shell (set of faces connected by edges, open or closed)
- [ ] Solid (region of space bounded by oriented shells)

## Rendering / export
- [ ] Convert edge into segments
- [ ] Convert wire into segments
- [ ] Convert face into triangles + normals
- [ ] Convert shell into triangles + normals
- [ ] Convert solid into triangles + normals

## CAD Operations

- [ ] Transform (reflect, translate, rotate)
- [ ] Extrude
- [ ] Revolve
- [ ] Boolean (union, intersection, subtract, invert?)
- TBD!
