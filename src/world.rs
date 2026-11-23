use crate::particle::Particle;
use macroquad::prelude::*;

pub struct World {
    particles: Vec<Particle>,
}

impl World {
    /// Creates a world with `count` random particles.
    pub fn new(count: usize) -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    /// Updates every particle (later: collisions too).
    pub fn update(&mut self, dt: f32) {}

    /// Draws every particle.
    pub fn draw(&self) {
        
    }
}
