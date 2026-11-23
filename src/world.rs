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

    /// Moves every particle, then resolves particle-particle collisions:
    /// overlapping pairs are pushed apart and swap velocities.
    pub fn update(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.update(dt);
        }
        let n = self.particles.len();
        for i in 0..n {
            for j in i + 1..n {
                let (left, right) = self.particles.split_at_mut(j);
                let pos_a = left[i].pos;
                let pos_b = right[0].pos;
                let r_a = left[i].radius;
                let r_b = right[0].radius;
                let distance = pos_a.distance(pos_b);
                if distance < r_a + r_b {
                    let overlap = (r_a + r_b) - distance;
                    let dir = (pos_b - pos_a).normalize_or_zero();

                    left[i].pos -= dir * (overlap / 2.0);
                    right[0].pos += dir * (overlap / 2.0);
                    std::mem::swap(&mut left[i].vel, &mut right[0].vel)
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
