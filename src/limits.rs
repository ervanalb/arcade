pub const WORKSPACE_SIZE: f64 = 1e6; // The largest acceptable coordinate to avoid floating point imprecision
pub const MINIMUM_VERTEX_SEPARATION: f64 = 1e-6; // Distinct vertices must not be closer than this value
pub const EPSILON_VERTEX_COINCIDENT: f64 = 1e-9; // Coincident vertices must not be farther than this value
pub const MINIMUM_CROSS_PRODUCT_NON_COLINEAR: f64 = 1e-6; // Non-collinear lines must have a cross product larger than this value
pub const EPSILON_CROSS_PRODUCT: f64 = 1e-0; // Collinear lines must have a cross product less than this value
pub const MINIMUM_PARAMETER_SEPARATION: f64 = 1e-6; // The smallest useful parameter sweep
pub const EPSILON_PARAMETER: f64 = 1e-9; // Two parameter values are considered equal if they are within this value
