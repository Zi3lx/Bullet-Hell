use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam, Image};
use nalgebra as na;
use crate::player::Player;
use crate::bullet::Bullet;
use crate::enemy::Enemy;

pub struct HexagonEnemy {
    pub size: f32,
    pub pos: na::Point2<f32>,
    pub hp: i32,
    pub speed: f32,
    pub damage: i32,
    pub bullet_speed: f32,
    pub last_shot_time: f32,
    pub shoot_cooldown: f32,
    pub coins: i32,
    pub points: i32,
    pub sprite: Image
}

impl HexagonEnemy {
    pub fn new(pos: na::Point2<f32>, level: i32, image: Image) -> Self {
        HexagonEnemy {
            size: 62.5,
            pos,
            hp: 3 * level,
            speed: 3.0 * level as f32 / 2.0 ,
            damage: 2 * level,
            bullet_speed: 4.0 * level as f32,
            last_shot_time: 0.0,
            shoot_cooldown: 3.5,
            coins: 100 * level,
            points: 50 * level,
            sprite: image
        }
    }

    fn move_towards_player(&mut self, player_pos: &na::Point2<f32>) {
        let direction = (player_pos - self.pos).normalize();
        self.pos += direction * self.speed;
    }

    // Shooting one bullet at a time towards player
    fn shoot(&self, _ctx: &mut Context, target: na::Point2<f32>) -> GameResult<Vec<Bullet>> {
        let mut bullets = Vec::new();
        let bullet = Bullet::new(self.pos, target, self.bullet_speed, self.damage, 10.0);
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
            // Destructing match result to get Ok(bullets) shoot retursns ok => then bullets else Error
            if let Ok(bullets) = self.shoot(ctx, player.player_pos) {
                game_bullets.extend(bullets); // Add bullets to the game's bullets list
            }
            self.last_shot_time = current_time; // Update the last shot time
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.sprite, DrawParam::default().dest([self.pos.x, self.pos.y]).scale([2.5, 2.5]))?;
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

    fn take_damage(&mut self, damage: i32) -> i32 {
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