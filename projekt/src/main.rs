mod player;
mod enemy;
mod bullet;
mod game;

use ggez::{ContextBuilder, GameResult};
use ggez::event;
use game::Game;

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("bullet_hell", "BoomBoom")
        .window_setup(ggez::conf::WindowSetup::default().title("Bullet Hell"))
        .window_mode(ggez::conf::WindowMode {
            width: 1500.0,
            height: 1000.0,
            ..Default::default()
        })
        .build()?;
    
    let game = Game::new()?;
    
    event::run(ctx, event_loop, game)
}
