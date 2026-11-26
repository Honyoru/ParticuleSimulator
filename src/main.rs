use macroquad::prelude::*;
use macroquad::ui::root_ui;

mod grid;
mod particle;
mod settings;
mod world;

use world::World;

fn window_conf() -> Conf {
    Conf {
        window_title: "Particule".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Seed the random generator, otherwise you get the same "random" particles every run
    rand::srand(miniquad::date::now() as u64);

    let mut world = World::new(100);
    let mut show_settings = true;
    loop {
        let dt = get_frame_time(); // seconds since the last frame

        clear_background(BLACK);

        world.update(dt);
        world.draw();

        // Settings panel: Tab toggles it, × closes it, the button reopens it.
        if is_key_pressed(KeyCode::Tab) {
            show_settings = !show_settings;
        }
        if show_settings {
            show_settings = world.settings.draw_ui();
        } else if root_ui().button(vec2(10.0, 40.0), "Settings") {
            show_settings = true;
        }

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 24.0, 28.0, WHITE);

        next_frame().await;
    }
}
