use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam};
use ggez::mint::Point2;
use nalgebra as na;
use nalgebra::{Vector2};
use crate::player::Player;
use crate::bullet::Bullet;


pub trait Shop {
    fn update(&mut self, player: &Player, ctx: &mut Context, game_bullets: &mut Vec<Bullet>);
    fn draw(&self, ctx: &mut Context) -> GameResult;
    fn upgrade_damage();
    fn upgrade_speed();
    fn upgrade_bullet_speed();
    fn upgrade_fire_rate();
}