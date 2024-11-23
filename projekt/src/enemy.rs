use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam};
use nalgebra as na;
use crate::player::Player;

pub trait Enemy {
    fn update(&mut self);
    fn draw(&self, ctx: &mut Context) -> GameResult;
    fn check_collision(&self, player: &Player) -> bool;
    fn damage(&mut self, amount: i32);
}

pub struct TriangleEnemy {
    pub pos: na::Point2<f32>,
    pub hp: i32,
}

impl Enemy for TriangleEnemy {
    fn update(&mut self) {
        self.pos.x -= 2.0;
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let triangle = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[
                [self.pos.x, self.pos.y],
                [self.pos.x + 10.0, self.pos.y + 20.0],
                [self.pos.x - 10.0, self.pos.y + 20.0],
            ],
            Color::from_rgb(0, 255, 0), // Green
        )?;
        graphics::draw(ctx, &triangle, DrawParam::default())?;
        Ok(())
    }

    fn check_collision(&self, player: &Player) -> bool {
        self.pos.coords.metric_distance(&player.player_pos.coords) < 20.0
    }

    fn damage(&mut self, amount: i32) {
        self.hp -= amount;
    }
}

pub struct HexagonEnemy {
    pub pos: na::Point2<f32>,
    pub hp: i32,
}

impl Enemy for HexagonEnemy {
    fn update(&mut self) {
        self.pos.x -= 1.0;
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
            Color::from_rgb(0, 0, 255), // Blue
        )?;
        graphics::draw(ctx, &hexagon, DrawParam::default())?;
        Ok(())
    }

    fn check_collision(&self, player: &Player) -> bool {
        self.pos.coords.metric_distance(&player.player_pos.coords) < 20.0
    }

    fn damage(&mut self, amount: i32) {
        self.hp -= amount;
    }
}

pub struct Boss {
    pub pos: na::Point2<f32>,
    pub hp: i32,
}

impl Enemy for Boss {
    fn update(&mut self) {
        // Update logic for the boss
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let boss = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.pos.x, self.pos.y, 40.0, 40.0),
            Color::from_rgb(255, 0, 0), // Red color
        )?;
        graphics::draw(ctx, &boss, DrawParam::default())?;
        Ok(())
    }

    fn check_collision(&self, player: &Player) -> bool {
        self.pos.coords.metric_distance(&player.player_pos.coords) < 40.0
    }

    fn damage(&mut self, amount: i32) {
        self.hp -= amount;
    }
}
