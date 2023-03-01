use std::f64;

pub type Float = f64;

pub const FLOAT_DIVISION_EPSILON: Float = 1e-9;
pub const PI: Float = f64::consts::PI;
pub const EPSILON_VERTEX_COINCIDENT: f64 = 1e-9; // Coincident vertices must not be farther than this value
