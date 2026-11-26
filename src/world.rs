use crate::grid::*;
use crate::particle::*;
use crate::settings::*;
use macroquad::prelude::*;
use macroquad::ui::root_ui;

/// Particle life: below this fraction of `life_radius`, particles always repel.
const BETA: f32 = 0.3;

pub struct World {
    particles: Vec<Particle>,
    grid: Grid,
    /// Window size the grid was built for; the grid is rebuilt when it
    /// (or the cell size needed by the mode) changes.
    grid_size: Vec2,
    /// Mouse action this frame: 1.0 = attract, -1.0 = repel, 0.0 = none.
    mouse_factor: f32,
    /// True if the current click started on the UI (it then belongs to the UI until released).
    click_on_ui: bool,
    pub settings: Settings,
    /// Attraction of type `a` toward type `b`: `rules[a][b]` in -1..1 (> 0 attract, < 0 repel).
    rules: [[f32; 4]; 4],
}

impl World {
    /// Creates a world with `count` random particles.
    pub fn new(count: usize) -> Self {
        let rules = Self::random_rules();
        let mut particles = Vec::new();
        let grid_size = vec2(screen_width(), screen_height());
        let grid = Grid::new(grid_size.x, grid_size.y, 2.0 * MAX_RADIUS);
        for _ in 0..count {
            particles.push(Particle::new());
        }
        Self {
            particles,
            grid,
            grid_size,
            mouse_factor: 0.0,
            click_on_ui: false,
            settings: Settings::default(),
            rules,
        }
    }

    /// Fills a rules table with random values in -1..1.
    fn random_rules() -> [[f32; 4]; 4] {
        let mut rules = [[0.0; 4]; 4];
        for k1 in 0..COLORS.len() {
            for k2 in 0..COLORS.len() {
                rules[k1][k2] = rand::gen_range(-1.0, 1.0);
            }
        }
        rules
    }

    /// Pulls (left click) or pushes (right click) the particles inside the mouse ring.
    /// Used by every mode.
    fn mouse_force(&mut self, dt: f32, mouse: Vec2) {
        if self.mouse_factor == 0.0 {
            return; // no click: nothing to do
        }
        for particle in &mut self.particles {
            if particle.pos.distance(mouse) < self.settings.mouse_radius {
                let dir = (mouse - particle.pos).normalize_or_zero();
                particle.vel += dir * self.mouse_factor * self.settings.mouse_strength * dt;
            }
        }
    }

    /// Classic mode: overlapping particles are pushed apart and swap velocities.
    fn classic(&mut self) {
        let n = self.particles.len();
        for i in 0..n {
            let neighbours = self.grid.neighbours(self.particles[i].pos);
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

    /// Particle life force between two particles.
    /// `r` = distance / life_radius (0..1), `rule` = rules[a][b] in -1..1.
    /// Returns > 0 for attraction, < 0 for repulsion.
    fn life_force(r: f32, rule: f32) -> f32 {
        if r < BETA {
            r / BETA - 1.0 // too close: always repel (-1 at contact, 0 at BETA)
        } else if r < 1.0 {
            rule * (1.0 - (2.0 * r - 1.0 - BETA).abs() / (1.0 - BETA)) // triangle peak
        } else {
            0.0 // too far: no effect
        }
    }

    /// Particle life mode: each pair of nearby particles attracts or repels
    /// according to `rules[a.kind][b.kind]`, plus friction.
    fn particles_life(&mut self, dt: f32) {
        // "Randomize rules" button: a new universe.
        if self.settings.randomize_rules {
            self.rules = Self::random_rules();
            self.settings.randomize_rules = false;
        }

        let radius = self.settings.life_radius;
        let strength = self.settings.life_strength;
        let n = self.particles.len();
        for i in 0..n {
            let neighbours = self.grid.neighbours(self.particles[i].pos);
            for j in neighbours {
                if j <= i {
                    continue;
                }
                let (left, right) = self.particles.split_at_mut(j);
                let delta = right[0].pos - left[i].pos;
                let distance = delta.length();
                if distance == 0.0 || distance >= radius {
                    continue;
                }
                let dir = delta / distance; // c. unit vector a → b
                let r = distance / radius; // d. 0..1
                // e. each particle reacts with its own rule
                let kind_a = left[i].kind;
                let kind_b = right[0].kind;
                let force_a = Self::life_force(r, self.rules[kind_a][kind_b]);
                let force_b = Self::life_force(r, self.rules[kind_b][kind_a]);
                left[i].vel += dir * force_a * strength * dt;
                right[0].vel -= dir * force_b * strength * dt;
            }
        }
        // f. friction: the speed halves every `life_half_life` seconds, whatever the FPS.
        let friction = 0.5_f32.powf(dt / self.settings.life_half_life);
        for particle in &mut self.particles {
            particle.vel *= friction;
        }
    }

    /// Advances the simulation by `dt` seconds: moves the particles, rebuilds the grid,
    /// applies the mouse force, then the current mode's interactions.
    pub fn update(&mut self, dt: f32) {
        // Rebuild the grid if the window size or the mode's cell size changed
        // (fullscreen, resize, mode switch, range slider). The cell must cover the interaction distance.
        let screen = vec2(screen_width(), screen_height());
        let cell_size = match self.settings.mode {
            Mode::Classic => 2.0 * MAX_RADIUS, // collisions: up to one diameter
            Mode::ParticuleLife => self.settings.life_radius, // forces: up to life_radius
        };
        if screen != self.grid_size || cell_size != self.grid.cell_size {
            self.grid = Grid::new(screen.x, screen.y, cell_size);
            self.grid_size = screen;
        }
        let mouse = Vec2::from(mouse_position());
        let left_click = is_mouse_button_down(MouseButton::Left);
        let right_click = is_mouse_button_down(MouseButton::Right);

        // A click belongs to whatever it started on: the UI or the particles.
        // Decided once, on the frame the button goes down.
        if is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_pressed(MouseButton::Right)
        {
            // Over the settings panel, or over the "Settings" reopen button.
            self.click_on_ui =
                root_ui().is_mouse_over(mouse) || reopen_button_rect().contains(mouse);
        }

        self.mouse_factor = if self.click_on_ui {
            0.0
        } else if left_click && !right_click {
            1.0
        } else if right_click && !left_click {
            -1.0
        } else {
            0.0
        };

        // Update the particles.
        for particle in &mut self.particles {
            particle.update(dt, &self.settings);
        }

        // Update the grid.
        self.grid.clear();
        for (i, particle) in self.particles.iter().enumerate() {
            self.grid.insert(i, particle.pos);
        }

        // Mouse force: every mode.
        self.mouse_force(dt, mouse);

        // Mode-specific interactions.
        match self.settings.mode {
            Mode::Classic => self.classic(),
            Mode::ParticuleLife => self.particles_life(dt),
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
        let color = if self.mouse_factor > 0.0 {
            Color::new(0.2, 0.3, 1.0, 0.6) // attract: blue
        } else if self.mouse_factor < 0.0 {
            Color::new(1.0, 0.3, 0.2, 0.6) // repel: red
        } else {
            Color::new(1.0, 1.0, 1.0, 0.3) // idle: white
        };
        let thickness = if self.mouse_factor != 0.0 { 4.0 } else { 3.0 };
        draw_circle_lines(x, y, self.settings.mouse_radius, thickness, color);
    }
}
