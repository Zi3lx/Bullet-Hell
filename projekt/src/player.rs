use ggez::{Context, GameResult};
use ggez::input::{keyboard, mouse};
use nalgebra as na;
use std::time::{Duration, Instant};
use crate::bullet::Bullet;
use ggez::graphics::{self, DrawParam, Color, Mesh, Image};


pub struct Player {
    pub hp: i32,
    pub speed: f32,
    pub damage: i32,
    pub player_pos: na::Point2<f32>,
    pub bullets: Vec<Bullet>,
    pub last_shot_time: Instant,
    pub fire_rate: f32,
    pub player_bullet_speed: f32,
    pub coins: i32,
    pub points: i32,
    pub player_image: Image,
}

impl Player {
    pub fn new(image: Image) -> GameResult<Player> {
        let s = Player {
            hp: 100,
            damage: 1,
            speed: 5.0,
            player_pos: na::Point2::new(400.0, 300.0),
            bullets: Vec::new(),
            last_shot_time: Instant::now(),
            fire_rate: 0.6,
            player_bullet_speed: 15.0,
            coins: 1000,
            points: 0,
            player_image: image
        };
        Ok(s)
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp < 0 {
            self.hp = 0;
        }
    }

    pub fn check_possition(&mut self) {
        if self.player_pos.x < 0.0 {
            self.player_pos.x = 0.0;
        } else if self.player_pos.x > 1570.0 {
            self.player_pos.x = 1570.0;
        }
        if self.player_pos.y < 0.0 {
            self.player_pos.y = 0.0;
        } else if self.player_pos.y > 1000.0 {
            self.player_pos.y = 1000.0;
        }
    }

    pub fn fire(&mut self, ctx: &mut Context) {
        if self.last_shot_time.elapsed() >= Duration::from_secs_f32(self.fire_rate) {
            let mouse_pos = mouse::position(ctx);
            let bullet = Bullet::new(self.player_pos, na::Point2::new(mouse_pos.x, mouse_pos.y), self.player_bullet_speed, self.damage, 10.0);
            self.bullets.push(bullet);
            self.last_shot_time = Instant::now();  // Update the shot time
        }
    }

    fn draw_ui(&self, ctx: &mut Context) -> ggez::GameResult {
        let text = format!("HP: {} \nPoints: {} \nDamage: {} \nFire Rate: {} \nSpeed: {} \nCoins: {}",
            self.hp, self.points, self.damage, self.fire_rate, self.speed, self.coins);

        let display_text = graphics::Text::new((text, graphics::Font::default(), 30.0));
        graphics::draw(ctx, &display_text, DrawParam::default().dest([10.0, 800.0]))?;
        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Player movement
        let speed = self.speed;
        if keyboard::is_key_pressed(ctx, ggez::event::KeyCode::W) {
            self.player_pos.y -= speed;
        }
        if keyboard::is_key_pressed(ctx, ggez::event::KeyCode::S) {
            self.player_pos.y += speed;
        }
        if keyboard::is_key_pressed(ctx, ggez::event::KeyCode::A) {
            self.player_pos.x -= speed;
        }
        if keyboard::is_key_pressed(ctx, ggez::event::KeyCode::D) {
            self.player_pos.x += speed;
        }

        self.check_possition();

        // Exit the game when ESC is pressed
        if keyboard::is_key_pressed(ctx, ggez::event::KeyCode::Escape) {
            ggez::event::quit(ctx);
        }

        // Fire when the spacebar is pressed
        if keyboard::is_key_pressed(ctx, ggez::event::KeyCode::Space) || mouse::button_pressed(ctx, ggez::input::mouse::MouseButton::Left) {
            self.fire(ctx);
        }

        // Update bullets
        for bullet in &mut self.bullets {
            bullet.update();
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_ui(ctx)?;

        let mouse_pos = mouse::position(ctx);
        let dx = mouse_pos.x - self.player_pos.x;
        let dy = mouse_pos.y - self.player_pos.y;
        let angle = dy.atan2(dx);
    
        graphics::draw(ctx, &self.player_image, DrawParam::default()
            .dest([self.player_pos.x, self.player_pos.y])
            .rotation(angle)
            .offset([0.5, 0.5])
        )?;
    
        let health_bar_width = 50.0;
        let health_bar_height = 5.0;
        let health_percentage = self.hp as f32 / 100.0;
        let health_bar = graphics::Rect::new(
            self.player_pos.x - 25.0,
            self.player_pos.y - 50.0, // Above the player
            health_bar_width * health_percentage,
            health_bar_height,
        );
        let health_mesh = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            health_bar,
            Color::from_rgb(255, 0, 0),
        )?;
        graphics::draw(ctx, &health_mesh, DrawParam::default())?;
    
        // Draw bullets
        for bullet in &self.bullets {
            bullet.draw(ctx)?;
        }
        graphics::present(ctx)?;
        Ok(())
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}
