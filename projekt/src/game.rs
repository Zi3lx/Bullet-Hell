use ggez::{Context, GameResult};
use ggez::event::{EventHandler};
use crate::player::Player;
use crate::enemy::{Enemy, TriangleEnemy, HexagonEnemy};
use crate::bullet::Bullet;
use rand::Rng;
use nalgebra as na;
use ggez::graphics;

pub struct Game {
    pub player: Player,
    pub enemies: Vec<Box<dyn Enemy>>,
    pub bullets: Vec<Bullet>
}

impl Game {
    pub fn new() -> GameResult<Game> {
        let player = Player::new()?;
        let enemies = Vec::new(); // Start with no enemies
        let bullets = Vec::new();
        Ok(Game { player, enemies, bullets })
    }

    fn spawn_enemy(&mut self) {
        let mut rng = rand::thread_rng();
        let side = rng.gen_range(0..4); // Choose one of the 4 sides: 0=top, 1=right, 2=bottom, 3=left
    
        let x_pos = match side {
            0 => rng.gen_range(0.0..1500.0),   // Top edge (random x, y=0)
            1 => 1500.0,                       // Right edge (x=1500, random y)
            2 => rng.gen_range(0.0..1500.0),   // Bottom edge (random x, y=1000)
            3 => 0.0,                          // Left edge (x=0, random y)
            _ => unreachable!(),
        };

        let y_pos = match side {
            0 => 0.0,                          // Top edge (y=0, random x)
            1 => rng.gen_range(0.0..1000.0),   // Right edge (random y, x=1500)
            2 => 1000.0,                       // Bottom edge (y=1000, random x)
            3 => rng.gen_range(0.0..1000.0),   // Left edge (random y, x=0)
            _ => unreachable!(),
        };

        let enemy_type = rng.gen_range(1..2); // Randomly choose between Triangle and Hexagon
    
        println!("Spawning enemy at ({}, {})", x_pos, y_pos); // Debugging spawn location
        
        match enemy_type {
            0 => { // Spawn TriangleEnemy
                let enemy = TriangleEnemy::new(na::Point2::new(x_pos, y_pos));
                self.enemies.push(Box::new(enemy)); // Adding to vector
                println!("Spawned TriangleEnemy at ({}, {})", x_pos, y_pos);
            }
            1 => { // Spawn HexagonEnemy
                let enemy = HexagonEnemy::new(na::Point2::new(x_pos, y_pos));
                self.enemies.push(Box::new(enemy)); // Adding to vector
                println!("Spawned HexagonEnemy at ({}, {})", x_pos, y_pos);
            }
            _ => {}
        }

        println!("Enemies count: {}", self.enemies.len());
        println!("Bullets count: {}", self.bullets.len());
    }

    fn fire_bullet(&mut self, pos: na::Point2<f32>, target: na::Point2<f32>, speed: f32) {
        let bullet = Bullet::new(pos, target, speed);
        self.bullets.push(bullet);
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.player.update(ctx)?;

        // Update all enemies
        let mut enemies_to_remove = Vec::new();
        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            enemy.update(&self.player, ctx, &mut self.bullets);

            if enemy.check_collision(&self.player) {
                enemy.apply_damage(&mut self.player); // Apply damage to player
                enemies_to_remove.push(i); // Store index of enemy to remove
                println!("Enemy touched player! Player HP: {}", self.player.hp);
            }
        }
        
        // Remove collided enemies
        for i in enemies_to_remove.iter().rev() {
            self.enemies.remove(*i);
        }


        for bullet in self.bullets.iter_mut() {
            bullet.update(); // Funkcja do aktualizacji pozycji pocisku
        }

        self.bullets.retain(|bullet| !bullet.is_off_screen()); // Remove player bullets that are off-screen

        // Spawn enemies randomly
        if rand::random::<f32>() < 0.01 { // Random chance to spawn an enemy
            self.spawn_enemy();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));

        for enemy in &self.enemies {
            enemy.draw(ctx)?; // Call the draw method of each enemy
        }
    
        for bullet in &self.bullets {
            bullet.draw(ctx)?; // Rysowanie pocisków
        }

        self.player.draw(ctx)?;  // This should already be defined in the Player struct

        // Present the drawn content
        graphics::present(ctx)?;

        Ok(())
    }
}
