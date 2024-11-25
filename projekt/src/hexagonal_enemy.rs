use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam};
use nalgebra as na;
use crate::bullet::Bullet;
use crate::player::Player;
use crate::enemy::Enemy;

impl HexagonEnemy {
    pub fn new(pos: na::Point2<f32>) -> Self {
        HexagonEnemy {
            pos,
            hp: 10,
            speed: 1.0,
            damage: 2,
            bullet_speed: 8.0,
            last_shot_time: 0.0,
            shoot_cooldown: 2.0,
        }
    }

    fn get_pos(&self) -> &na::Point2<f32> {
        &self.pos
    }

    fn move_towards_player(&mut self, player_pos: &na::Point2<f32>) {
        let direction = (player_pos - self.pos).normalize();
        self.pos += direction * self.speed;
    }

    fn shoot(&self, ctx: &mut Context, target: na::Point2<f32>) -> GameResult<Vec<Bullet>> {
        let mut bullets = Vec::new();
        let bullet = Bullet::new(self.pos, target, self.bullet_speed, self.damage);
        bullets.push(bullet);
        Ok(bullets)
    }    
}

impl Enemy for HexagonEnemy {
    fn update(&mut self, player: &Player, ctx: &mut Context, game_bullets: &mut Vec<Bullet>) {
        self.move_towards_player(&player.player_pos);
        
        let current_time = ggez::timer::time_since_start(ctx).as_secs_f32();
        let time_between_shots = self.shoot_cooldown; // Shoot every 2 seconds

        if current_time - self.last_shot_time > time_between_shots {
            if let Ok(bullets) = self.shoot(ctx, player.player_pos) {
                game_bullets.extend(bullets); // Add bullets to the game's bullets list
            }
            self.last_shot_time = current_time; // Update the last shot time
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let hexagon = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[
                [self.pos.x, self.pos.y],
                [self.pos.x + 20.0, self.pos.y + 10.0],
                [self.pos.x + 20.0, self.pos.y + 30.0],
                [self.pos.x, self.pos.y + 40.0],
                [self.pos.x - 20.0, self.pos.y + 30.0],
                [self.pos.x - 20.0, self.pos.y + 10.0],
            ],
            Color::from_rgb(0, 0, 255),
        )?;
        graphics::draw(ctx, &hexagon, DrawParam::default())?;
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

    fn take_damage(&self, damage: i32) {
        self.hp -= damage;
        if self.hp < 0 {
            self.hp = 0;
        }
    }
}
