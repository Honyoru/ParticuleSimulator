use crate::particle::Particle;

pub struct World {
    particles: Vec<Particle>,
}

impl World {
    /// Creates a world with `count` random particles.
    pub fn new(count: usize) -> Self {
        todo!()
    }

    /// Updates every particle (later: collisions too).
    pub fn update(&mut self, dt: f32) {
        todo!()
    }

    /// Draws every particle.
    pub fn draw(&self) {
        todo!()
    }
}