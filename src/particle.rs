use macroquad::prelude::*;

const GRAVITY: f32 = 500.0;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub color: Color,
}

impl Particle {
    /// Creates a particle with a random position, velocity, radius and color.
    pub fn new() -> Self {
        let radius = rand::gen_range(2.0, 30.0);
        let x = rand::gen_range(radius, screen_width() - radius);
        let y = rand::gen_range(radius, screen_height() - radius);
        let color = Color::new(
            rand::gen_range(0.0, 1.0),
            rand::gen_range(0.0, 1.0),
            rand::gen_range(0.0, 1.0),
            1.0,
        );
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(
                rand::gen_range(-200.0, 200.0),
                rand::gen_range(-200.0, 200.0),
            ),
            radius,
            color,
        }
    }

    /// Moves the particle: gravity, movement, bouncing off the edges.
    pub fn update(&mut self, dt: f32) {
        self.vel.y += GRAVITY * dt;
        self.pos += self.vel * dt;
    }

    /// Draws the particle as a circle.
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
    }
}
