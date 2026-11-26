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
