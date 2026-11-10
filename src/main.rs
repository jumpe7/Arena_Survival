mod player;
mod game;
mod enemy;

use macroquad::prelude::*;

fn window_conf() -> Conf{
    Conf{
        window_title: String::from("Arena Survival"),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut game = game::Game::new();
    while game.status == true {
        clear_background(BLACK);
        // draw_text(
        //     &format!("health: {}", game.player.health),
        //     10.0,
        //     10.0,
        //     20.0,
        //     WHITE,
        // );


        game.update();
        game.draw();

        next_frame().await
    }
}
