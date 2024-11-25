use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam};
use nalgebra as na;
use crate::bullet::Bullet;
use crate::player::Player;
use crate::enemy::Enemy;

pub struct TriangleEnemy {
    pub pos: na::Point2<f32>,
    pub hp: i32,
    pub speed: f32,
    pub damage: i32,
}

impl TriangleEnemy {
    pub fn new(pos: na::Point2<f32>) -> Self {
        TriangleEnemy {
            pos,
            hp: 1,
            speed: 6.0,
            damage: 1,
        }
    }

    fn get_pos(&self) -> &na::Point2<f32> {
        &self.pos
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

    fn take_damage(&self, damage: i32) {
        self.hp -= damage;
    }
}

pub struct HexagonEnemy {
    pub pos: na::Point2<f32>,
    pub hp: i32,
    pub speed: f32,
    pub damage: i32,
    pub bullet_speed: f32,
    pub last_shot_time: f32,
    pub shoot_cooldown: f32
}