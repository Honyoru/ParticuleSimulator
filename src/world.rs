use crate::particle::Particle;
use macroquad::prelude::*;

pub struct World {
    particles: Vec<Particle>,
}

impl World {
    /// Creates a world with `count` random particles.
    pub fn new(count: usize) -> Self {
        let mut particles = Vec::new();
        for _ in 0..count {
            particles.push(Particle::new());
        }
        Self { particles }
    }

    /// Updates every particle (later: collisions too).
    pub fn update(&mut self, dt: f32) {}

    /// Draws every particle.
    pub fn draw(&self) {
        for particle in &self.particles {
            particle.draw();
        }
    }
}
