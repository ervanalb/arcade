use std::f64;
use std::ops::BitXor;

pub type Float = f64;

pub const FLOAT_DIVISION_EPSILON: Float = 1e-9;
pub const PI: Float = f64::consts::PI;
pub const EPSILON_COINCIDENT_DISTANCE: f64 = 1e-9; // Distance between two coincident objects (like vertices or a vertex and a plane) must not be farther than this value

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Reverse,
}

impl BitXor for Direction {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::Forward, Self::Forward) => Self::Forward,
            (Self::Forward, Self::Reverse) => Self::Reverse,
            (Self::Reverse, Self::Forward) => Self::Reverse,
            (Self::Reverse, Self::Reverse) => Self::Forward,
        }
    }
}

