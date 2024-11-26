use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam};
use ggez::mint::Point2;
use nalgebra as na;
use std::f32::consts::PI;
use crate::player::Player;
use crate::bullet::Bullet;
use crate::enemy::Enemy;
use std::time::{Duration, Instant};
use rand::Rng;
use ggez::timer;
use std::fmt;


enum BossState {
    Idle,
    AttackNormal,
    AttackCircle,
    AttackMachine,
}

impl fmt::Debug for BossState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state_str = match self {
            BossState::Idle => "Idle",
            BossState::AttackNormal => "AttackNormal",
            BossState::AttackCircle => "AttackCircle",
            BossState::AttackMachine => "AttackMachine",
        };
        write!(f, "{}", state_str)
    }
}

pub struct Boss {
    pub pos: na::Point2<f32>,
    pub hp: i32,
    pub max_hp: i32,
    pub speed: f32,
    pub damage: i32,
    pub bullet_speed: f32,
    pub last_shot_time: f32,
    pub shoot_cooldown: f32,
    pub attacks_cooldown: f32,
    pub attack_timer: f32,
    pub current_state: BossState,
    pub move_timer: f32,
    pub coins: i32,
    pub points: i32,

    pub circle_bullets_count: usize,
    pub burst_shot_interval: f32,      // Odstęp czasu między strzałami w serii
    pub current_bullet_count: usize,   // Liczba pocisków już wystrzelonych
    pub time_since_last_bullet: f32,
}


impl Boss {
    pub fn new(pos: na::Point2<f32>, level: i32) -> Self {
        Boss {
            pos,
            hp: 100 * level,
            max_hp: 100 * level,
            speed: 1.0,
            damage: 8 * level,
            bullet_speed: 4.0 * level as f32,
            last_shot_time: 3.0,
            shoot_cooldown: 3.0,
            attacks_cooldown: 2.5,
            attack_timer: 0.0,
            current_state: BossState::Idle,
            move_timer: 3.0,
            coins: 1000 * level,
            points: 100 * level,

            circle_bullets_count: 8 * level as usize,
            burst_shot_interval: 0.2,             // Odstęp czasu między strzałami w serii
            current_bullet_count: 0,       // Początkowo brak pocisków wystrzelonych
            time_since_last_bullet: 0.0,
        }
    }

    fn choose_random_move(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let random_move = rng.gen_range(0..3);

        self.current_state = match random_move {
            0 => BossState::AttackNormal,
            1 => BossState::AttackCircle,
            2 => BossState::AttackMachine,
            _ => unreachable!(),
        };
    }

    fn move_towards_player(&mut self, target: &Player) {
        let dir = (target.player_pos - &self.pos).normalize();
        self.pos.x += dir.x * self.speed;
        self.pos.y += dir.y * self.speed;
    }

    fn shoot_pattern(&mut self, ctx: &mut Context) -> GameResult<Vec<Bullet>> {
        let mut bullets = Vec::new();

        // Only shoot when cooldown has passed
        self.attack_timer += timer::delta(ctx).as_secs_f32();
        if self.attack_timer >= self.shoot_cooldown {
            let directions = vec![
                na::Point2::new(self.pos.x, self.pos.y + 1.0), // Down
                na::Point2::new(self.pos.x, self.pos.y - 1.0), // Up
                na::Point2::new(self.pos.x - 1.0, self.pos.y), // Left
                na::Point2::new(self.pos.x + 1.0, self.pos.y), // Right
            ];

            for target in directions {
                let bullet = Bullet::new(self.pos, target, self.bullet_speed, self.damage, 20.0);
                bullets.push(bullet);
            }

            // Reset the attack timer after shooting
            self.attack_timer = 0.0;
        }

        Ok(bullets)
    }

    
    fn shoot_pattern_circle(&mut self, ctx: &mut Context, bullets_in_burst: usize) -> GameResult<Vec<Bullet>> {
        let mut bullets = Vec::new();
        let angle_step = 2.0 * PI / (bullets_in_burst as f32);

        self.attack_timer += timer::delta(ctx).as_secs_f32();

        if self.attack_timer >= self.shoot_cooldown {
            for i in 0..bullets_in_burst {
                let angle = i as f32 * angle_step;
                let dir_x = angle.cos();
                let dir_y = angle.sin();
                let target = na::Point2::new(self.pos.x + dir_x, self.pos.y + dir_y);
                let bullet = Bullet::new(self.pos, target, self.bullet_speed, self.damage, 20.0);
                bullets.push(bullet);
            }

            self.attack_timer = 0.0; // Reset the attack timer after shooting
        }

        Ok(bullets)
    }
    

    fn shoot_burst_towards_player(&mut self, ctx: &mut Context, target: na::Point2<f32>) -> GameResult<Vec<Bullet>> {
        let mut bullets = Vec::new();
    
        // We increment the attack timer as we are in the AttackMachine state
        self.attack_timer += timer::delta(ctx).as_secs_f32();
    
        // If the burst is not finished and enough time has passed between shots
        if self.attack_timer >= self.shoot_cooldown  {
            let bullet = Bullet::new(self.pos, target, self.bullet_speed * 2.0, self.damage, 50.0);
            bullets.push(bullet);
            self.attack_timer = 0.0;  // Reset timer after firing
        }

        Ok(bullets)
    }
    
    

    fn draw_hp(&self, ctx: &mut Context) -> GameResult {
        // Rysowanie tła paska HP
        let background_rect = graphics::Rect::new(self.pos.x - 25.0, self.pos.y - 35.0, 50.0, 5.0);
        let background_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), background_rect, graphics::Color::from_rgb(0, 0, 0))?;
        graphics::draw(ctx, &background_mesh, graphics::DrawParam::default())?;

        // Rysowanie paska HP
        let hp_width = (self.hp as f32 / self.max_hp as f32) * 50.0;
        let hp_rect = graphics::Rect::new(self.pos.x - 25.0, self.pos.y - 35.0, hp_width, 5.0);
        let hp_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), hp_rect, graphics::Color::from_rgb(0, 255, 0))?;
        graphics::draw(ctx, &hp_mesh, graphics::DrawParam::default())?;

        Ok(())
    }
}

impl Enemy for Boss {
    fn update(&mut self, player: &Player, ctx: &mut Context, game_bullets: &mut Vec<Bullet>) {
        // Ruch bossa
        self.move_towards_player(&player);

        //self.last_shot_time += self.shoot_cooldown;

        match self.current_state {
            BossState::AttackNormal => {
                if let Ok(bullets) = self.shoot_pattern(ctx) {
                    game_bullets.extend(bullets);
                }
                self.choose_random_move();
            }
            BossState::AttackCircle => {
                if let Ok(bullets) = self.shoot_pattern_circle(ctx, self.circle_bullets_count) {
                    game_bullets.extend(bullets);
                } 
                self.choose_random_move();
            }
            BossState::AttackMachine => {
                if let Ok(bullets) = self.shoot_burst_towards_player(ctx, player.player_pos) {
                    game_bullets.extend(bullets);
                }
                self.choose_random_move();
            }
            BossState::Idle => {
                self.choose_random_move();
            }
        }
        println!("Boss state: {:?}", self.current_state);
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        // Rysowanie bossa jako prostokąt lub inny kształt
        let rect = graphics::Rect::new(self.pos.x - 25.0, self.pos.y - 25.0, 50.0, 50.0);
        let color = graphics::Color::from_rgb(255, 0, 0); // Kolor czerwony dla bossa

        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        // Opcjonalnie: Rysowanie paska HP bossa
        self.draw_hp(ctx)?;

        Ok(())
    }
    
    fn check_collision(&self, player: &Player) -> bool {
        let enemy_rect = graphics::Rect::new(self.pos.x - 10.0, self.pos.y - 10.0, 50.0, 50.0);
        let player_rect = graphics::Rect::new(player.player_pos.x - 10.0, player.player_pos.y - 10.0, 50.0, 50.0);
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

    fn is_boss(&self) -> bool { true }

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
