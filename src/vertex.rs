use std::fmt;

use crate::vec::Vec3;
use crate::error::Error;
use crate::error::Result;
use crate::limits;

#[derive(Debug)]
pub struct Vertex {
    // A vertex is a point in space. It wraps a Vec3 and gives stronger guarantees (non-NAN, in bounds, etc.)
    point: Vec3
}

impl Vertex {
    fn check_nan(a: Vec3) -> Result<()> {
        match a.is_finite() {
            false => Err(Error::NotANumber),
            true => Ok(())
        }
    }

    fn check_bounds(a: Vec3) -> Result<()> {
        match a.x.abs() < limits::WORKSPACE_SIZE
           && a.y.abs() < limits::WORKSPACE_SIZE 
           && a.z.abs() < limits::WORKSPACE_SIZE {
            false => Err(Error::OutOfBounds),
            true => Ok(())
        }
    }

    pub fn new(point: Vec3) -> Result<Vertex> {
        Vertex::check_nan(point)?;
        Vertex::check_bounds(point)?;

        Ok(Vertex {
            point: point
        })
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn is_coincident(&self, other: Vertex) -> bool {
        (self.point - other.point).is_within(limits::EPSILON_VERTEX_COINCIDENT)
    }

    pub fn check_vertex_separation(&self, other: &Vertex) -> Result<()> {
        match (self.point - other.point).is_within(limits::MINIMUM_VERTEX_SEPARATION) {
            true => Err(Error::VerticesTooClose),
            false => Ok(())
        }
    }

    // Also checks vertex separation
    pub fn check_colinear(&self, other1: &Vertex, other2: &Vertex) -> Result<()> {
        self.check_vertex_separation(other1)?;
        self.check_vertex_separation(other2)?;
        other1.check_vertex_separation(other2)?;

        let v1 = self.point - other1.point;
        let v2 = self.point - other2.point;
        match v1.cross(v2).is_within(limits::MINIMUM_CROSS_PRODUCT_NON_COLINEAR * v1.length() * v2.length()) {
            true => Err(Error::VerticesColinear),
            false => Ok(())
        }
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.point.fmt(f)
    }
}

// TESTS
#[test]
fn vertex_construction() {
    assert!(
        Vertex::new(limits::WORKSPACE_SIZE * Vec3::new(0.5, 0.9, -0.3))
        .is_ok());
    assert_eq!(
        Vertex::new(limits::WORKSPACE_SIZE * Vec3::new(0.5, 0.9, -1.3)).unwrap_err(),
        Error::OutOfBounds);
    assert_eq!(
        Vertex::new(Vec3::new(55., std::f64::NAN, -23.)).unwrap_err(),
        Error::NotANumber);

    let v = Vec3::new(0.5, 0.9, -0.3) * limits::WORKSPACE_SIZE;
    assert!(Vertex::new(v).unwrap().is_coincident(
        Vertex::new(v + Vec3::new(0.3, -0.9, -0.7) * limits::EPSILON_VERTEX_COINCIDENT).unwrap()));

    assert!(!Vertex::new(v).unwrap().is_coincident(
        Vertex::new(v + Vec3::new(1.3, -0.9, -1.7) * limits::EPSILON_VERTEX_COINCIDENT).unwrap()));
}
