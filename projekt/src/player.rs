use ggez::{Context, GameResult};
use ggez::input::{keyboard, mouse};
use nalgebra as na;
use std::time::{Duration, Instant};
use crate::bullet::Bullet;

pub struct Player {
    pub hp: i32,
    pub speed: f32,
    pub damage: i32,
    pub player_pos: na::Point2<f32>,
    pub bullets: Vec<Bullet>,
    pub last_shot_time: Instant, // Time of the last shot
}

impl Player {
    pub fn new() -> GameResult<Player> {
        let s = Player {
            hp: 10,
            damage: 1,
            speed: 5.0,
            player_pos: na::Point2::new(400.0, 300.0),
            bullets: Vec::new(),
            last_shot_time: Instant::now(),
        };
        Ok(s)
    }

    pub fn fire(&mut self, ctx: &mut Context) {
        if self.last_shot_time.elapsed() >= Duration::from_secs_f32(0.5) {
            let mouse_pos = mouse::position(ctx);
            let bullet = Bullet::new(self.player_pos, na::Point2::new(mouse_pos.x, mouse_pos.y));
            self.bullets.push(bullet);
            self.last_shot_time = Instant::now();  // Update the shot time
        }
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

        // Exit the game when ESC is pressed
        if keyboard::is_key_pressed(ctx, ggez::event::KeyCode::Escape) {
            ggez::event::quit(ctx);
        }

        // Fire when the spacebar is pressed
        if keyboard::is_key_pressed(ctx, ggez::event::KeyCode::Space) {
            self.fire(ctx);
        }

        // Update bullets
        for bullet in &mut self.bullets {
            bullet.update();
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        use ggez::graphics::{self, Mesh, DrawParam, Color};

        graphics::clear(ctx, Color::from_rgb(0, 0, 0));

        // Draw player
        let player = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [self.player_pos.x, self.player_pos.y],
            20.0,
            2.0,
            Color::from_rgb(0, 255, 0),
        )?;
        graphics::draw(ctx, &player, DrawParam::default())?;

        // Draw bullets
        for bullet in &self.bullets {
            bullet.draw(ctx)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
