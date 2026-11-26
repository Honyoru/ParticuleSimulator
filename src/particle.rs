use macroquad::prelude::*;

use crate::settings::*;

pub const MAX_RADIUS: f32 = 4.0;
pub const MIN_RADIUS: f32 = 2.0;
/// One colour per particle type (particle life).
pub const COLORS: [Color; 4] = [RED, GREEN, BLUE, YELLOW];

/// A single particle: a disc that moves and bounces off the window edges.
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub color: Color,
    /// Type of the particle: index into `COLORS` and the rules table.
    pub kind: usize,
}

impl Particle {
    /// Creates a particle with a random position, velocity, radius and type
    /// (its colour comes from the type).
    pub fn new() -> Self {
        let kind = rand::gen_range(0, COLORS.len());
        let radius = rand::gen_range(MIN_RADIUS, MAX_RADIUS);
        let x = rand::gen_range(radius, screen_width() - radius);
        let y = rand::gen_range(radius, screen_height() - radius);
        let color = COLORS[kind];
        /*
        let color = Color::new(
            rand::gen_range(0.0, 1.0),
            rand::gen_range(0.0, 1.0),
            rand::gen_range(0.0, 1.0),
            1.0,
        );
        */
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(
                rand::gen_range(-200.0, 200.0),
                rand::gen_range(-200.0, 200.0),
            ),
            radius,
            color,
            kind,
        }
    }

    /// Moves the particle: gravity, movement, bouncing off the edges.
    pub fn update(&mut self, dt: f32, settings: &Settings) {
        self.vel.y += settings.gravity * dt;
        self.pos += self.vel * dt;
        if self.pos.x + self.radius > screen_width() {
            self.pos.x = screen_width() - self.radius;
            self.vel.x = -self.vel.x.abs() * settings.damping;
        } else if self.pos.x - self.radius < 0.0 {
            self.pos.x = self.radius;
            self.vel.x = self.vel.x.abs() * settings.damping;
        }
        if self.pos.y + self.radius > screen_height() {
            self.pos.y = screen_height() - self.radius;
            self.vel.y = -self.vel.y.abs() * settings.damping;
        } else if self.pos.y - self.radius < 0.0 {
            self.pos.y = self.radius;
            self.vel.y = self.vel.y.abs() * settings.damping;
        }
    }

    /// Draws the particle as a circle.
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
    }
}
