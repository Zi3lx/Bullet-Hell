use ggez::{Context, GameResult};
use ggez::event::{EventHandler};
use crate::player::Player;
use crate::enemy::{Enemy, TriangleEnemy, HexagonEnemy};
use crate::bullet::Bullet;
use crate::shop::Shop;

use rand::Rng;
use nalgebra as na;
use ggez::graphics;

pub struct Game {
    pub player: Player,
    pub shop: Shop,
    pub enemies: Vec<Box<dyn Enemy>>,
    pub bullets: Vec<Bullet>
}

impl Game {
    pub fn new() -> GameResult<Game> {
        let player = Player::new()?;
        let shop = Shop::new()?;
        let enemies = Vec::new(); // Start with no enemies
        let bullets = Vec::new();
        Ok(Game { player, shop, enemies, bullets })
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

        let enemy_type = rng.gen_range(0..2); // Randomly choose between Triangle and Hexagon
    
        println!("Spawning enemy at ({}, {})", x_pos, y_pos); // Debugging spawn location
        
        match enemy_type {
            0 => { // Spawn TriangleEnemy
                let enemy = TriangleEnemy::new(na::Point2::new(x_pos, y_pos));
                self.enemies.push(Box::new(enemy)); // Adding to vector
            }
            1 => { // Spawn HexagonEnemy
                let enemy = HexagonEnemy::new(na::Point2::new(x_pos, y_pos));
                self.enemies.push(Box::new(enemy)); // Adding to vector
            }
            _ => {}
        }

        //println!("Enemies count: {}", self.enemies.len());
        //println!("Bullets count: {}", self.bullets.len());
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ggez::input::keyboard::is_key_pressed(ctx, ggez::event::KeyCode::Key1) {
            self.shop.try_buy_health_upgrade(&mut self.player);
        }
        if ggez::input::keyboard::is_key_pressed(ctx, ggez::event::KeyCode::Key2) {
            self.shop.try_buy_damage_upgrade(&mut self.player);
        }
        if ggez::input::keyboard::is_key_pressed(ctx, ggez::event::KeyCode::Key3) {
            self.shop.try_buy_speed_upgrade(&mut self.player);
        }
        if ggez::input::keyboard::is_key_pressed(ctx, ggez::event::KeyCode::Key4) {
            self.shop.try_buy_fire_rate_upgrade(&mut self.player);
        }

        self.player.update(ctx)?;

        // Update all enemies
        let mut enemies_to_remove = Vec::new();
        let mut player_bullets_to_remove = Vec::new();

        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            enemy.update(&self.player, ctx, &mut self.bullets);

            if enemy.check_collision(&self.player) {
                enemy.apply_damage(&mut self.player); // Apply damage to player
                enemies_to_remove.push(i); // Store index of enemy to remove
                self.player.coins += enemy.get_coins();
                println!("Enemy touched player! Player HP: {}", self.player.hp);
            }

            for (j, bullet) in self.player.bullets.iter_mut().enumerate() {
                if bullet.check_collision_with_enemy(&**enemy) {
                    if bullet.apply_damage_to_enemy(&mut **enemy) == 0 {    
                        enemies_to_remove.push(i);
                        self.player.coins += enemy.get_coins();
                    }
                    player_bullets_to_remove.push(j);
                    println!("Bullet touched enemy! Enemy HP: {}", enemy.get_hp());
                }

                if bullet.is_off_screen() {
                    if player_bullets_to_remove.contains(&j) {
                        continue;
                    }
                    player_bullets_to_remove.push(j);
                }
            }
        }



        let mut bullets_to_remove = Vec::new();
        for (i, bullet) in self.bullets.iter_mut().enumerate() {
            bullet.update();
            if bullet.check_collision_with_player(&self.player) == true {
                bullet.apply_damage(&mut self.player);
                bullets_to_remove.push(i);
            }
            if bullet.is_off_screen() {
                if bullets_to_remove.contains(&i) {
                    continue;
                }
                bullets_to_remove.push(i);
            }
        }

        //println!{"Player bullets {}, enemies {}, bullets {}", self.player.bullets.len(), self.enemies.len(), self.bullets.len()};

        for &index in player_bullets_to_remove.iter().rev() {
            if index < self.player.bullets.len() {
                self.player.bullets.remove(index);
                println!("Bullet removed from player's bullets at index {}", index);
            }
        }
        
        for i in bullets_to_remove.iter().rev() {
            self.bullets.remove(*i);
            println!("Bullet removed {}", i);
        }

        for i in enemies_to_remove.iter().rev() {
            self.enemies.remove(*i);
            println!("Enemy removed {}", i);
        }

        // Spawn enemies randomly 0.02 = 2%
        if rand::random::<f32>() < 0.01 { 
            self.spawn_enemy();
        }

        println!("Player hp: {}, damage: {}, speed: {}, coins: {}", self.player.hp, self.player.damage, self.player.speed, self.player.coins);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));

        for enemy in &self.enemies {
            enemy.draw(ctx)?;
        }
    
        for bullet in &self.bullets {
            bullet.draw(ctx)?
        }

        self.player.draw(ctx)?; 
        self.shop.display(ctx, &mut self.player)?;

        // Present the drawn content
        graphics::present(ctx)?;

        Ok(())
    }
}
