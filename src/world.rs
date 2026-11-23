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
    pub fn update(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.update(dt);
        }
        let n = self.particles.len();
        for i in 0..n {
            for j in i + 1..n {
                let pos_a = self.particles[i].pos;
                let pos_b = self.particles[j].pos;
                let r_a = self.particles[i].radius;
                let r_b = self.particles[j].radius;
                if pos_a.distance(pos_b) < r_a + r_b {
                    self.particles[i].color = WHITE;
                    self.particles[j].color = WHITE;
                }
            }
        }
    }

    /// Draws every particle.
    pub fn draw(&self) {
        for particle in &self.particles {
            particle.draw();
        }
    }
}
