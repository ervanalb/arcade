use nalgebra::{Vector2, Vector3, Vector4, DVector, Matrix, U3, U4, Dynamic, VecStorage};

pub type Vec2 = Vector2<f64>;
pub type Vec3 = Vector3<f64>;
pub type Vec4 = Vector4<f64>;
pub type VecN = DVector<f64>;
pub type Mat3xN = Matrix::<f64, U3, Dynamic, VecStorage<f64, U3, Dynamic>>;
pub type Mat4xN = Matrix::<f64, U4, Dynamic, VecStorage<f64, U4, Dynamic>>;
