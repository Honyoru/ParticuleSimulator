use crate::grid::*;
use crate::particle::*;
use macroquad::prelude::*;

const MOUSE_RADIUS: f32 = 100.0;
const MOUSE_STRENGTH: f32 = 2000.0;

pub struct World {
    particles: Vec<Particle>,
    grid: Grid,
}

impl World {
    /// Creates a world with `count` random particles.
    pub fn new(count: usize) -> Self {
        let mut particles = Vec::new();
        let grid = Grid::new(screen_width(), screen_height(), 2.0 * MAX_RADIUS);
        for _ in 0..count {
            particles.push(Particle::new());
        }
        Self { particles, grid }
    }

    /// Moves every particle, then resolves particle-particle collisions:
    /// overlapping pairs are pushed apart and swap velocities.
    /// Mouse can attrack and repel the particles.
    pub fn update(&mut self, dt: f32) {
        let mouse = Vec2::from(mouse_position());
        let left_click = is_mouse_button_down(MouseButton::Left);
        let right_click = is_mouse_button_down(MouseButton::Right);
        let factor = if left_click && !right_click {
            1.0
        } else if right_click && !left_click {
            -1.0
        } else {
            0.0
        };
        // Update the particles.
        for particle in &mut self.particles {
            particle.update(dt);
        }

        // Update the grid.
        self.grid.clear();
        for (i, particle) in self.particles.iter().enumerate() {
            self.grid.insert(i, particle.pos);
        }

        // Collision and attraction.
        let n = self.particles.len();
        for i in 0..n {
            let p = &mut self.particles[i];
            let neighbours = self.grid.neighbours(p.pos);
            if factor != 0.0 && p.pos.distance(mouse) < MOUSE_RADIUS {
                let dir = (mouse - p.pos).normalize_or_zero();
                self.particles[i].vel += dir * factor * MOUSE_STRENGTH * dt
            }

            // Collision.
            for j in neighbours {
                if j <= i {
                    continue;
                }
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
    /// Draw everything.
    pub fn draw(&self) {
        // Draw particles.
        for particle in &self.particles {
            particle.draw();
        }

        // Draw the mouse circle.
        let (x, y) = mouse_position();
        draw_circle_lines(x, y, MOUSE_RADIUS, 3.0, Color::new(1.0, 1.0, 1.0, 0.3));
    }
}
