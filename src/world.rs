use crate::{bounds::Bounds, gravity::Gravity, particle::Particle, types::Position};
use colorgrad::{Gradient, LinearGradient};

const TRAIL_LENGTH: u32 = 100;
const NUM_PARTICLES: usize = 10;
const GRAVITY_STRENGTH: f64 = 300000.0;

pub struct World {
    width: f64,
    height: f64,
    bounds: Bounds,
    particles: Vec<Particle>,
    trail_particles: Vec<Particle>,
    speed_colors: LinearGradient,
    trail_colors: LinearGradient,
    gravity: Gravity,
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        let width = width as f64;
        let height = height as f64;
        Self {
            width,
            height,
            bounds: Bounds::new(width, height),
            particles: initialize_particles(NUM_PARTICLES, width as i32, height as i32),
            trail_particles: vec![],
            speed_colors: colorgrad::GradientBuilder::new()
                .html_colors(&["#ff006e", "#ffc882"])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),
            trail_colors: colorgrad::GradientBuilder::new()
                .html_colors(&["#2a2430", "#9e379c", "#ff006e"])
                .build::<colorgrad::LinearGradient>()
                .unwrap(),

            gravity: Gravity::new(
                vec![
                    Position::new(768.0, 576.0),
                    Position::new(768.0, 384.0),
                    Position::new(768.0, 192.0),
                    Position::new(512.0, 576.0),
                    Position::new(512.0, 384.0),
                    Position::new(512.0, 192.0),
                    Position::new(256.0, 576.0),
                    Position::new(256.0, 384.0),
                    Position::new(256.0, 192.0),
                ],
                GRAVITY_STRENGTH,
            ),
        }
    }

    pub fn update(&mut self, time_delta: f64) {
        self.particles.iter_mut().for_each(|particle| {
            self.trail_particles
                .push(Particle::new_trail(particle.position, TRAIL_LENGTH));

            let acceleration = self.gravity.acceleration(&particle.position);
            particle.apply_acceleration(acceleration, time_delta);
            particle.update_position(time_delta, &self.bounds);
        });

        self.trail_particles.retain(|particle| particle.ttl > 0);
        self.trail_particles.iter_mut().for_each(|particle| {
            particle.ttl -= 1;
        });
    }

    pub fn draw(&self, frame: &mut [u8]) {
        // clear frame
        for byte in frame.iter_mut() {
            *byte = 0;
        }

        // draw particles
        self.particles.iter().for_each(|particle| {
            let idx = idx(particle, self.width);
            let speed = particle.velocity.norm();
            let color = self.speed_colors.at((speed / 100.0) as f32).to_rgba8();

            self.draw_dot(idx, &color, frame);
        });

        self.trail_particles.iter().for_each(|particle| {
            let idx = idx(particle, self.width);
            let color = self
                .trail_colors
                .at(particle.ttl as f32 / TRAIL_LENGTH as f32)
                .to_rgba8();

            self.draw_dot(idx, &color, frame);
        });
    }

    fn draw_dot(&self, center_idx: usize, color: &[u8; 4], frame: &mut [u8]) {
        let ids: Vec<usize> = [
            Some(center_idx + (self.width as usize * 4)),
            Some(center_idx + 4),
            Some(center_idx),
            center_idx.checked_sub(4),
            center_idx.checked_sub(self.width as usize * 4),
        ]
        .iter()
        .filter_map(|&x| x)
        .collect();

        ids.iter().for_each(|&id| {
            if (id + 4) < frame.len() {
                frame[id..(id + 4)].copy_from_slice(color);
            }
        });
    }
}

fn initialize_particles(count: usize, width: i32, height: i32) -> Vec<Particle> {
    let mut particles = vec![Particle::default(); count];

    particles.iter_mut().for_each(|particle| {
        particle.position.x = rand::random_range(0..width) as f64;
        particle.position.y = rand::random_range(0..height) as f64;
        particle.velocity.x = rand::random_range(-50..50) as f64;
        particle.velocity.y = rand::random_range(-50..50) as f64;
    });

    particles
}

fn idx(particle: &Particle, width: f64) -> usize {
    (particle.position.x.floor() + particle.position.y.floor() * width) as usize * 4
}
