use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam};
use nalgebra as na;

pub struct Bullet {
    pub pos: na::Point2<f32>,
    pub vel: na::Vector2<f32>,
    pub speed: f32,
}

impl Bullet {
    pub fn new(pos: na::Point2<f32>, target: na::Point2<f32>, speed: f32) -> Bullet {
        let direction = (target - pos).normalize();
        let sp = speed;
        Bullet {
            pos,
            vel: direction * sp,
            speed,
        }
    }

    pub fn update(&mut self) {
        self.pos += self.vel;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let square = graphics::Rect::new(self.pos.x, self.pos.y, 10.0, 10.0);
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
}
