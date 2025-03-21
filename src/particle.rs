use crate::{
    bounds::Bounds,
    intersection,
    types::{Acceleration, Position, Velocity},
};

#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Position,
    pub velocity: Velocity,
    pub ttl: u32,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            position: Position::new(0.0, 0.0),
            velocity: Velocity::identity(),
            ttl: 1,
        }
    }
}

impl Particle {
    pub fn new_trail(position: Position, ttl: u32) -> Self {
        Self {
            position,
            ttl,
            ..Default::default()
        }
    }

    pub fn apply_acceleration(&mut self, acceleration: Acceleration, time_delta: f64) {
        self.velocity += acceleration * time_delta;
    }

    pub fn update_position(&mut self, time_delta: f64, bounds: &Bounds) {
        let mut time_delta = time_delta;

        while time_delta > 0.0000001 {
            let next_position = self.position + self.velocity * time_delta;
            let candidates = bounds.candidates(&next_position);

            if candidates.is_empty() {
                self.position = next_position;
                return;
            }

            let (intersection, distance_quota, bound) =
                intersection::find(self.position, next_position, candidates);
            time_delta -= time_delta * distance_quota;

            self.position = intersection;
            if bound == &bounds.top || bound == &bounds.bottom {
                self.velocity.y *= -0.5;
                self.velocity.x *= 0.7;
            } else {
                self.velocity.x *= -0.5;
                self.velocity.y *= 0.7;
            }
        }
    }
}
