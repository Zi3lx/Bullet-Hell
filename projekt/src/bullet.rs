use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam};
use nalgebra as na;
use crate::player::Player;
use crate::enemy::Enemy;

pub struct Bullet {
    pub pos: na::Point2<f32>,
    pub vel: na::Vector2<f32>,
    pub damage: i32,
    pub size: f32,
}

impl Bullet {
    pub fn new(pos: na::Point2<f32>, target: na::Point2<f32>, speed: f32, damage: i32, size_of_bullet: f32) -> Bullet {
        let direction = (target - pos).normalize();
        let sp = speed;
        Bullet {
            pos,
            vel: direction * sp,
            damage,
            size: size_of_bullet
        }
    }

    /*pub fn new(pos: na::Point2<f32>, vel: na::Vector2<f32>, speed: f32, damage: i32, sizeOfBullet: f32) -> Bullet {
        let direction = (target - pos).normalize();
        let sp = speed;
        Bullet {
            pos,
            vel: direction * sp,
            speed,
            damage,
            size: sizeOfBullet,
        }
    }*/

    pub fn update(&mut self) {
        self.pos += self.vel;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let square = graphics::Rect::new(self.pos.x, self.pos.y, self.size, self.size);
        let square_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            square,
            Color::from_rgb(255, 0, 0), // Red color
        )?;
        graphics::draw(ctx, &square_mesh, DrawParam::default())?;
        Ok(())
    }

    pub fn is_off_screen(&self) -> bool {
        self.pos.x < 0.0 || self.pos.x > 1600.0 || self.pos.y < 0.0 || self.pos.y > 1100.0
    }

    pub fn check_collision_with_player(&self, player: &Player) -> bool {
        let player_rect = graphics::Rect::new(player.player_pos.x - 10.0, player.player_pos.y - 10.0, 20.0, 20.0);
        let bullet_rect = graphics::Rect::new(self.pos.x - 10.0, self.pos.y - 10.0, 20.0, 20.0);
        bullet_rect.overlaps(&player_rect)
    }

    pub fn apply_damage(&self, player: &mut Player) {
        player.take_damage(self.damage);
    }

    pub fn apply_damage_to_enemy(&self, enemy: &mut dyn Enemy) -> i32{
        enemy.take_damage(self.damage);
        enemy.get_hp()
    }

    pub fn check_collision_with_enemy(&self, enemy: &dyn Enemy) -> bool {
        let enemy_pos = enemy.get_pos();
        let enemy_rect = graphics::Rect::new(enemy_pos.x - 10.0, enemy_pos.y - 10.0, 20.0, 20.0);
        let bullet_rect = graphics::Rect::new(self.pos.x - 10.0, self.pos.y - 10.0, 20.0, 20.0);
        bullet_rect.overlaps(&enemy_rect)
    }

}
