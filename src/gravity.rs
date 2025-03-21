use crate::types::{Acceleration, Position};

pub struct Gravity {
    points: Vec<Position>,
    strength: f64,
}

impl Gravity {
    pub fn new(points: Vec<Position>, strength: f64) -> Self {
        Self { points, strength }
    }

    pub fn acceleration(&self, position: &Position) -> Acceleration {
        let mut acceleration = Acceleration::new(0.0, 0.0);
        for point in self.points.iter() {
            let direction = point - position;
            let distance = direction.norm();
            let point_acceleration = direction.normalize() * (self.strength / distance.powf(2.));

            acceleration += point_acceleration;
        }

        acceleration
    }
}
