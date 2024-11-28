use ggez::{Context, GameResult};
use nalgebra as na;
use crate::player::Player;
use crate::bullet::Bullet;

pub trait Enemy {
    fn update(&mut self, player: &Player, ctx: &mut Context, game_bullets: &mut Vec<Bullet>);
    fn draw(&self, ctx: &mut Context) -> GameResult;
    fn check_collision(&self, player: &Player) -> bool;
    fn apply_damage(&self, player: &mut Player);
    fn take_damage(&mut self, damage: i32) -> i32;

    fn is_boss(&self) -> bool {
        false
    }

    fn get_pos(&self) -> &na::Point2<f32>;
    fn get_hp(&self) -> i32;
    fn get_coins(&self) -> i32;
    fn get_points(&self) -> i32;
}