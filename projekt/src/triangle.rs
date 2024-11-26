use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam};
use ggez::mint::Point2;
use nalgebra as na;
use std::f32::consts::PI;
use crate::player::Player;
use crate::bullet::Bullet;
use crate::enemy::Enemy;
use std::time::{Duration, Instant};

pub struct TriangleEnemy {
    pub pos: na::Point2<f32>,
    pub hp: i32,
    pub speed: f32,
    pub damage: i32,
    pub coins: i32,
    pub points: i32,
}

impl TriangleEnemy {
    pub fn new(pos: na::Point2<f32>, level: i32) -> Self {
        TriangleEnemy {
            pos,
            hp: 1 * level,
            speed: 6.0 * level as f32 / 2.0,
            damage: 1 * level,
            coins: 50 * level,
            points: 10 * level,
        }
    }

    fn move_towards_player(&mut self, player_pos: &na::Point2<f32>) {
        let direction = (player_pos - self.pos).normalize();
        self.pos += direction * self.speed;
    }
}

impl Enemy for TriangleEnemy {
    fn update(&mut self, player: &Player, ctx: &mut Context, game_bullets: &mut Vec<Bullet>) {
        self.move_towards_player(&player.player_pos);
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let vertices = vec![
            Point2 { x: self.pos.x, y: self.pos.y - 15.0 },
            Point2 { x: self.pos.x - 15.0, y: self.pos.y + 15.0 },
            Point2 { x: self.pos.x + 15.0, y: self.pos.y + 15.0 },
        ];
        let triangle = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &vertices,
            Color::from_rgb(255, 0, 0),
        )?;
        graphics::draw(ctx, &triangle, DrawParam::default())?;
        Ok(())
    }

    fn check_collision(&self, player: &Player) -> bool {
        let enemy_rect = graphics::Rect::new(self.pos.x - 10.0, self.pos.y - 10.0, 20.0, 20.0);
        let player_rect = graphics::Rect::new(player.player_pos.x - 10.0, player.player_pos.y - 10.0, 20.0, 20.0);
        enemy_rect.overlaps(&player_rect)
    }

    fn apply_damage(&self, player: &mut Player) {
        player.take_damage(self.damage);
    }

    /*fn shoot(&self, ctx: &mut Context, target: na::Point2<f32>) -> GameResult<Vec<Bullet>> {
        let mut bullets = Vec::new();
        let bullet = Bullet::new(self.pos, target, self.bullet_speed);
        bullets.push(bullet);
        Ok(bullets)
    }*/

    fn take_damage(&mut self, damage: i32) -> i32{
        self.hp -= damage;
        if self.hp < 0 {
            self.hp = 0;
        }
        self.hp
    }

    fn give_coins(&self, coins: i32, player: &mut Player) {
        player.coins += self.coins;
    }

    fn get_pos(&self) -> &na::Point2<f32> {
        &self.pos
    }

    fn get_hp(&self) -> i32 {
        self.hp
    }

    fn get_coins(&self) -> i32 {
        self.coins
    }

    fn get_points(&self) -> i32 {
        self.points
    }
}