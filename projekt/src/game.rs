use ggez::{Context, GameResult};
use crate::player::Player;
use crate::enemy::{Enemy, TriangleEnemy, HexagonEnemy, Boss};

pub struct Game {
    pub player: Player,
    pub enemies: Vec<Box<dyn Enemy>>,
}

impl Game {
    pub fn new() -> GameResult<Game> {
        let player = Player::new()?;
        let enemies: Vec<Box<dyn Enemy>> = Vec::new(); // You can add some enemies here
        Ok(Game { player, enemies })
    }
}

impl ggez::event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.player.update(ctx)?;

        for enemy in &mut self.enemies {
            enemy.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.player.draw(ctx)?;

        for enemy in &self.enemies {
            enemy.draw(ctx)?;
        }

        Ok(())
    }
}
