use nalgebra::SVector;

pub type Position = SVector<f64, 2>;
pub type Line = (Position, Position);
pub type Velocity = SVector<f64, 2>;
pub type Acceleration = SVector<f64, 2>;
