use macroquad::prelude::*;
use crate::enemy::Enemy;

pub struct Player {
    pub(crate) position: Vec2,
    size: f32,
    speed: f32,
    pub(crate) health: f32,
    is_invulnerable: bool,
    invulnerable_timer: f32,
}

impl Player{

    pub fn new() -> Self {
        Self{
            position:Vec2::new(40.0,60.0),
            size: 30.0,
            speed: 100.0,
            health: 100.0,
            is_invulnerable: false,
            invulnerable_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt:f32, status: &mut bool){
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
            *status = false;
        }

        direction = direction.normalize_or_zero();
        self.position += direction*self.speed*dt;


        self.position.x = self.position.x.clamp(0.0, screen_width() - self.size);
        self.position.y = self.position.y.clamp(0.0, screen_height() - self.size);
    }

    pub fn collides_with(&self, enemy: &Enemy) -> bool {
        (self.position.x < enemy.position.x + enemy.size) &&
            (self.position.x + self.size > enemy.position.x) &&
            (self.position.y < enemy.position.y + enemy.size) &&
            (self.position.y + self.size > enemy.position.y)
    }

    pub fn take_damage(&mut self){
        if self.is_invulnerable {
            return;
        }

        if self.health > 0.0{
            self.health = (self.health - 10.0).max(0.0);
            self.is_invulnerable = true;
            self.invulnerable_timer = 1.0;
        }
    }

    pub fn update_invulnerable(&mut self, dt: f32){
        if self.is_invulnerable {
            self.invulnerable_timer -= dt;
            if self.invulnerable_timer <= 0.0{
                self.invulnerable_timer = 0.0;
                self.is_invulnerable = false;
            }
        }
    }

    pub fn draw(&self){
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size,
            self.size,
            BLUE
        )
    }
}
