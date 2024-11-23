use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};

struct MainState {
    player_pos: [f32; 2],
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState { player_pos: [400.0, 300.0] };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Ruch gracza na podstawie wciśniętych klawiszy
        let speed = 5.0;
        if ggez::input::keyboard::is_key_pressed(ctx, event::KeyCode::W) {
            self.player_pos[1] -= speed;
        }
        if ggez::input::keyboard::is_key_pressed(ctx, event::KeyCode::S) {
            self.player_pos[1] += speed;
        }
        if ggez::input::keyboard::is_key_pressed(ctx, event::KeyCode::A) {
            self.player_pos[0] -= speed;
        }
        if ggez::input::keyboard::is_key_pressed(ctx, event::KeyCode::D) {
            self.player_pos[0] += speed;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));
        
        let player = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [self.player_pos[0], self.player_pos[1]],
            20.0,
            2.0,
            Color::from_rgb(0, 255, 0),
        )?;
        graphics::draw(ctx, &player, graphics::DrawParam::default())?;
        
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("bullet_hell", "YourName")
        .build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state);
    
}
