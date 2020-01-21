use std::fmt;

use crate::vec::Vec3;
use crate::error::Error;
use crate::error::Result;
use crate::limits;

#[derive(Debug, PartialEq)]
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
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.point.fmt(f)
    }
}
