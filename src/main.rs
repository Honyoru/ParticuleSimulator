use macroquad::prelude::*;

mod grid;
mod particle;
mod world;

use world::World;

const MOUSE_RADIUS: f32 = 100.0;

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
    show_mouse(false);
    loop {
        let dt = get_frame_time(); // seconds since the last frame

        clear_background(BLACK);
        let (x, y) = mouse_position();
        world.update(dt);
        world.draw();
        draw_circle_lines(x, y, MOUSE_RADIUS, 3.0, Color::new(1.0, 1.0, 1.0, 0.3));
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 24.0, 28.0, WHITE);

        next_frame().await;
    }
}
