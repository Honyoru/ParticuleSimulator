use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

/// Tuning values of the simulation, editable live from the settings panel.
pub struct Settings {
    pub gravity: f32,
    pub damping: f32,
    pub mouse_radius: f32,
    pub mouse_strength: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            gravity: 500.0,
            damping: 0.7,
            mouse_radius: 100.0,
            mouse_strength: 3500.0,
        }
    }
}

impl Settings {
    /// Draws the settings panel and lets the user edit the values.
    /// Call it after drawing the world, so the panel is on top.
    /// Returns `false` if the user clicked the close button (×).
    pub fn draw_ui(&mut self) -> bool {
        widgets::Window::new(hash!(), vec2(10.0, 40.0), vec2(320.0, 140.0))
            .label("Settings")
            .close_button(true)
            .ui(&mut root_ui(), |ui| {
                ui.slider(hash!(), "gravity", -1000.0..1000.0, &mut self.gravity);
                ui.slider(hash!(), "damping", 0.0..1.0, &mut self.damping);
                ui.slider(hash!(), "radius", 1.0..500.0, &mut self.mouse_radius);
                ui.slider(hash!(), "strength", 1.0..10000.0, &mut self.mouse_strength);
            })
    }
}
