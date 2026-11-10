use macroquad::prelude::*;
use crate::player::Player;
use crate::enemy::Enemy;
pub struct Game{
    pub(crate) player: Player,
    enemies: Vec<Enemy>,
    pub(crate) status: bool
}

impl Game{
    pub fn new() -> Self {
        let mut enemies = Vec::new();

        for _ in 0..3{
            enemies.push(Enemy::new(
                vec2(rand::gen_range(0.0, screen_width()-50.0), rand::gen_range(0.0, screen_height()-50.0)),
                50.0,
                50.0,
            ));
        }

        Self {
            player: Player::new(),
            enemies: enemies,
            status: true
        }
    }

    pub fn update(&mut self){
        let dt = get_frame_time();
        self.player.update(dt, &mut self.status);
        for enemy in &mut self.enemies{
            enemy.update(&self.player, dt);
        }

        self.player.update_invulnerable(dt);

        for enemy in &mut self.enemies {
            if self.player.collides_with(enemy){
                self.player.take_damage();

            }
        }
    }

    pub fn draw(&self){
        self.player.draw();

        for enemy in &self.enemies{
            enemy.draw();
        }

        draw_text(
            &format!("HP: {}/100", self.player.health),
            10.0,
            15.0,
            20.0,
            WHITE,
        );

        draw_rectangle(
            10.0,
            30.0,
            200.0,
            20.0,
            RED,
        );

        draw_rectangle(
            10.0,
            30.0,
            self.player.health*2.0,
            20.0,
            GREEN,
        )
    }
}