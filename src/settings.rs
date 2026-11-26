use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

/// Tuning values of the simulation, editable live from the settings panel.
pub struct Settings {
    pub mode: Mode,
    pub gravity: f32,
    pub damping: f32,
    pub mouse_radius: f32,
    pub mouse_strength: f32,
    /// Particle life: distance (px) beyond which particles ignore each other.
    pub life_radius: f32,
    /// Particle life: strength of the forces (px/s²).
    pub life_strength: f32,
    /// Particle life: friction, the time (s) for the speed to halve.
    pub life_half_life: f32,
    /// Set by the "Randomize rules" button; `World` makes new rules and resets it.
    pub randomize_rules: bool,
}

/// Which simulation runs.
#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Classic,
    ParticuleLife,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            mode: Mode::Classic,
            gravity: 500.0,
            damping: 0.7,
            mouse_radius: 100.0,
            mouse_strength: 3500.0,
            life_radius: 80.0,
            life_strength: 800.0,
            life_half_life: 0.04,
            randomize_rules: false,
        }
    }
}

/// Where the "Settings" reopen button is drawn when the panel is closed.
/// `World` also uses it to ignore clicks on the button.
pub fn reopen_button_rect() -> Rect {
    Rect::new(10.0, 40.0, 90.0, 26.0)
}

/// Draws the "Settings" button shown while the panel is closed.
/// Returns `true` on the frame it is clicked.
pub fn draw_reopen_button() -> bool {
    let r = reopen_button_rect();
    widgets::Button::new("Settings")
        .position(vec2(r.x, r.y))
        .size(vec2(r.w, r.h))
        .ui(&mut root_ui())
}

impl Settings {
    /// Draws the settings panel and lets the user edit the values.
    /// Call it after drawing the world, so the panel is on top.
    /// Returns `false` if the user clicked the close button (×).
    pub fn draw_ui(&mut self) -> bool {
        widgets::Window::new(hash!(), vec2(10.0, 40.0), vec2(320.0, 230.0))
            .label("Settings")
            .close_button(true)
            .ui(&mut root_ui(), |ui| {
                // Mode selector.
                let old_mode = self.mode;
                let mut index = match self.mode {
                    Mode::Classic => 0,
                    Mode::ParticuleLife => 1,
                };
                ui.combo_box(hash!(), "mode", &["Classic", "Particle life"], &mut index);
                self.mode = match index {
                    0 => Mode::Classic,
                    _ => Mode::ParticuleLife,
                };
                // Mode just changed: load its default gravity.
                if self.mode != old_mode {
                    self.gravity = match self.mode {
                        Mode::Classic => 500.0,
                        Mode::ParticuleLife => 0.0,
                    };
                }

                // Settings of the current mode only.
                match self.mode {
                    Mode::Classic => {
                        ui.slider(hash!(), "gravity", -1000.0..1000.0, &mut self.gravity);
                        ui.slider(hash!(), "damping", 0.0..1.0, &mut self.damping);
                    }
                    Mode::ParticuleLife => {
                        ui.slider(hash!(), "range", 20.0..200.0, &mut self.life_radius);
                        ui.slider(hash!(), "force", 100.0..3000.0, &mut self.life_strength);
                        ui.slider(hash!(), "friction", 0.005..0.5, &mut self.life_half_life);
                        if ui.button(None, "Randomize rules") {
                            self.randomize_rules = true;
                        }
                    }
                }

                // Mouse: every mode.
                ui.slider(hash!(), "mouse radius", 1.0..500.0, &mut self.mouse_radius);
                ui.slider(
                    hash!(),
                    "mouse force",
                    1.0..10000.0,
                    &mut self.mouse_strength,
                );
            })
    }
}
