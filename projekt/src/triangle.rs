use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Image};
use nalgebra as na;
use crate::player::Player;
use crate::bullet::Bullet;
use crate::enemy::Enemy;
use ggez::mint::Point2;

pub struct TriangleEnemy {
    pub size: f32,
    pub pos: na::Point2<f32>,
    pub hp: i32,
    pub speed: f32,
    pub damage: i32,
    pub coins: i32,
    pub points: i32,
    pub sprite: Image
}

impl TriangleEnemy {
    pub fn new(pos: na::Point2<f32>, level: i32, image: Image) -> Self {
        TriangleEnemy {
            size: 30.0,
            pos,
            hp: 1 * level,
            speed: 6.0 * level as f32 / 2.0,
            damage: 1 * level,
            coins: 50 * level,
            points: 10 * level,
            sprite: image
        }
    }

    fn move_towards_player(&mut self, player_pos: &na::Point2<f32>) {
        let direction = (player_pos - self.pos).normalize();
        self.pos += direction * self.speed;
    }
}

impl Enemy for TriangleEnemy {
    fn update(&mut self, player: &Player, _ctx: &mut Context, _game_bullets: &mut Vec<Bullet>) {
        self.move_towards_player(&player.player_pos);
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.sprite, DrawParam::default().dest([self.pos.x, self.pos.y]).scale([1.5, 1.5]))?;
        Ok(())
    }

    fn check_collision(&self, player: &Player) -> bool {
        let enemy_rect = graphics::Rect::new(self.pos.x - 10.0, self.pos.y - 10.0, self.size, self.size);
        let player_rect = graphics::Rect::new(player.player_pos.x - 10.0, player.player_pos.y - 10.0, 20.0, 20.0);
        enemy_rect.overlaps(&player_rect)
    }

    fn apply_damage(&self, player: &mut Player) {
        player.take_damage(self.damage);
    }

    fn take_damage(&mut self, damage: i32) -> i32{
        self.hp -= damage;
        if self.hp < 0 {
            self.hp = 0;
        }
        self.hp
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

    fn get_size(&self) -> f32 {
        self.size
    }
}