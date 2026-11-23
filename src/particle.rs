use macroquad::prelude::*;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub color: Color,
}

impl Particle {
    /// Creates a particle with a random position, velocity, radius and color.
    pub fn new() -> Self {
        todo!()
    }

    /// Moves the particle: gravity, movement, bouncing off the edges.
    pub fn update(&mut self, dt: f32) {
        todo!()
    }

    /// Draws the particle as a circle.
    pub fn draw(&self) {
        todo!()
    }
}
