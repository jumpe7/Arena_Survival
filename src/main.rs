use macroquad::prelude::*;

struct Player {
    position: Vec2,
    size: f32,
    speed: f32,
}

struct Enemy {
    position: Vec2,
    size: f32,
    speed: f32,
}

#[macroquad::main("Arena Survival")]
async fn main() {
    let mut player = Player {
        position: vec2(100.0, 200.0),
        size: 30.0,
        speed: 100.0,
    };

    let mut enemies: Vec<Enemy> = Vec::new();
    for _ in 0..3{
        let max_width = screen_width();
        let max_height = screen_height();
        enemies.push(Enemy{
            position:vec2(rand::gen_range(0.0, max_width-50.0), rand::gen_range(0.0, max_height-50.0)),
            size:50.0,
            speed:100.0
        });
    }

    loop {
        let dt = get_frame_time();
        let mut direction = vec2(0.0, 0.0);

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            direction.y += 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            direction.x += 1.0;
        }

        if is_key_down(KeyCode::Escape){
            break;
        }

        direction = direction.normalize_or_zero();
        player.position += direction*player.speed*dt;


        player.position.x = player.position.x.clamp(0.0, screen_width() - player.size);
        player.position.y = player.position.y.clamp(0.0, screen_height() - player.size);

        clear_background(BLACK);

        draw_rectangle(
            player.position.x,
            player.position.y,
            player.size,
            player.size,
            BLUE,
        );

        for enemy in &enemies{
            draw_rectangle(
                enemy.position.x,
                enemy.position.y,
                enemy.size,
                enemy.size,
                GREEN,
            )
        }

        next_frame().await
    }
}
