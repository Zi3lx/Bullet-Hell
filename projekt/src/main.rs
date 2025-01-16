mod player;
mod enemy;
mod bullet;
mod game;
mod shop;
mod triangle;
mod hexagonal;
mod boss;
mod menu;

use ggez::{ContextBuilder, GameResult};
use ggez::event;
use game::Game;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("bullet_hell", "BoomBoom")
        .window_setup(ggez::conf::WindowSetup::default().title("Bullet Hell"))
        .window_mode(ggez::conf::WindowMode {
            width: 1500.0,
            height: 1000.0,
            ..Default::default()
        })
        .add_resource_path("resources")
        .build()?;
    
    let game = Game::new(&mut ctx)?;
    
    event::run(ctx, event_loop, game)
}
