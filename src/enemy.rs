use macroquad::prelude::*;
use crate::player::Player;

pub struct Enemy {
    pub(crate) position: Vec2,
    pub(crate) size: f32,
    pub(crate) speed: f32,
}

impl Enemy {

    pub fn new(position_enemy: Vec2, size_enemy: f32, speed_enemy: f32) -> Self{
        Self{
            position: position_enemy,
            size: size_enemy,
            speed: speed_enemy,
        }
    }

    pub fn update(&mut self, player: &Player,dt: f32){
        let mut direction = player.position - self.position;
        direction = direction.normalize_or_zero();
        self.position += direction * dt * self.speed;
    }

    pub fn draw(&self){
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size,
            self.size,
            GREEN,
        )
    }
}