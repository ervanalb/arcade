use std::f64;

pub type Float = f64;

pub const FLOAT_DIVISION_EPSILON: Float = 1e-9;
pub const PI: Float = f64::consts::PI;
pub const EPSILON_COINCIDENT_DISTANCE: f64 = 1e-9; // Distance between two coincident objects (like vertices or a vertex and a plane) must not be farther than this value
