use macroquad::prelude::*;

mod particle;
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

    let mut world = World::new(500);

    loop {
        let dt = get_frame_time(); // seconds since the last frame

        clear_background(BLACK);

        world.update(dt);
        world.draw();

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 24.0, 28.0, WHITE);

        next_frame().await;
    }
}