use ggez::{Context, GameResult};
use ggez::event::{EventHandler};
use ggez::graphics::{self, DrawParam};

use crate::player::Player;
use crate::enemy::Enemy;

use crate::triangle::TriangleEnemy;
use crate::hexagonal::HexagonEnemy;
use crate::boss::Boss;

use crate::bullet::Bullet;
use crate::shop::Shop;

use rand::Rng;
use nalgebra as na;

pub struct Game {
    pub player: Player,
    pub shop: Shop,
    pub enemies: Vec<Box<dyn Enemy>>,
    pub bullets: Vec<Bullet>,
    pub is_boss: bool,
    pub level: i32,
    pub killed_enemies: i32,
    pub spawn_rate: f32
}


impl Game {
    pub fn new() -> GameResult<Game> {
        let player = Player::new()?;
        let shop = Shop::new()?;
        let enemies = Vec::new();
        let bullets = Vec::new();
        let is_boss = false;
        let level = 1;
        let killed_enemies = 0;
        let spawn_rate = 0.02;
        Ok(Game { player, shop, enemies, bullets, is_boss, level, killed_enemies, spawn_rate })
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

        let enemy_type = rng.gen_range(0..3); // Randomly choose between Triangle and Hexagon
    
        println!("Spawning enemy at ({}, {})", x_pos, y_pos);
        
        match enemy_type {
            0 => { // Spawn TriangleEnemy
                let enemy = TriangleEnemy::new(na::Point2::new(x_pos, y_pos), self.level);
                self.enemies.push(Box::new(enemy));
            }
            1 => { // Spawn HexagonEnemy
                let enemy = HexagonEnemy::new(na::Point2::new(x_pos, y_pos), self.level);
                self.enemies.push(Box::new(enemy));
            }
            2 => { // Spawn Boss
                if !self.is_boss {
                    let boss_chance = rng.gen_range(0..10); // 10% chance to spawn boss
                    if boss_chance == 0 {
                        let enemy = Boss::new(na::Point2::new(x_pos, y_pos), self.level);
                        self.enemies.push(Box::new(enemy));
                        self.is_boss = true;
                    }
                }
            }
            _ => {}
        }

        //println!("Enemies count: {}", self.enemies.len());
        //println!("Bullets count: {}", self.bullets.len());
    }

    fn handle_shop_buy(&mut self, ctx: &mut Context) {
        // Buy upgrades by pressing keys 1, 2, 3, 4
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
    }

    fn handle_enemy_bullet_logic(&mut self, ctx: &mut Context) -> (Vec<usize>, Vec<usize>) {
        // Creates a vector of indexes to remove (enemy and bullets)
        let mut enemies_to_remove = Vec::new();
        let mut player_bullets_to_remove = Vec::new();

        // Update all enemies
        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            enemy.update(&self.player, ctx, &mut self.bullets); // Udpating

            if enemy.check_collision(&self.player) {
                enemy.apply_damage(&mut self.player); // Apply damage to player
                //println!("Enemy touched player! Player HP: {}", self.player.hp);
            }

            // Check for collisions between player bullets and enemies
            for (j, bullet) in self.player.bullets.iter_mut().enumerate() {
                if bullet.check_collision_with_enemy(&**enemy) {
                    if bullet.apply_damage_to_enemy(&mut **enemy) == 0 { 
                        if !enemies_to_remove.contains(&i) { // Check if the enemy is already marked for removal
                            if enemy.is_boss() { // Controls boss state (1 boss at a time)
                                self.is_boss = false;
                            }

                            enemies_to_remove.push(i);

                            self.killed_enemies += 1;
                            self.player.points += enemy.get_points();
                            self.player.coins += enemy.get_coins();
                        }
                    }
                    player_bullets_to_remove.push(j);
                    //println!("Bullet touched enemy! Enemy HP: {}", enemy.get_hp());
                }

                if bullet.is_off_screen() {
                    if player_bullets_to_remove.contains(&j) {
                        continue;
                    }
                    player_bullets_to_remove.push(j);
                }
            }
        }

        (enemies_to_remove, player_bullets_to_remove)
    }

    fn handle_player_bullet_logic(&mut self) -> Vec<usize> {
        // Creates a vector of indexes to remove
        let mut bullets_to_remove = Vec::new();

        // Update all bullets and check if bullets hit the player
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

        bullets_to_remove   
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
            self.handle_shop_buy(ctx);

            self.player.update(ctx)?;

            let (enemies_to_remove, player_bullets_to_remove) = self.handle_enemy_bullet_logic(ctx);
            
            let bullets_to_remove = self.handle_player_bullet_logic();

            //println!{"Player bullets {}, enemies {}, bullets {}", self.player.bullets.len(), self.enemies.len(), self.bullets.len()};

            for &index in player_bullets_to_remove.iter().rev() {
                if index < self.player.bullets.len() {
                    self.player.bullets.remove(index);
                    //println!("Bullet removed from player's bullets at index {}", index);
                }
            }
            

            // Remove enemies and bullets

            for i in bullets_to_remove.iter().rev() {
                self.bullets.remove(*i);
                //println!("Bullet removed {}", i);
            }

            for i in enemies_to_remove.iter().rev() {
                self.enemies.remove(*i);
                //println!("Enemy removed {}", i);
            }

            // Spawn enemies randomly
            if rand::random::<f32>() < self.spawn_rate {
                self.spawn_enemy();
            }

            // Check if level should be increased
            if self.killed_enemies >= 30 {
                self.level += 1;
                self.killed_enemies = 0;
                self.spawn_rate += 0.01;
            }

            //println!("Level: {}, Enemies killed: {}, Player hp: {}, damage: {}, speed: {}, coins: {}, SpawnRate {}", self.level, self.killed_enemies, self.player.hp, self.player.damage, self.player.speed, self.player.coins, self.spawn_rate);
            //println!("Player hp: {}, damage: {}, speed: {}, coins: {}", self.player.hp, self.player.damage, self.player.speed, self.player.coins);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));

        // Draw End screen if player dies else draw alla content
        if self.player.hp <= 0 {
            let text = format!("SCORE:{}",
            self.player.points);

            let display_text = graphics::Text::new((text, graphics::Font::default(), 40.0));

            // Use the tuple directly in DrawParam::dest()
            graphics::draw(ctx, &display_text, DrawParam::default().dest([650.0, 450.0]))?;
            println!("Game Over!");
        }
        else {
            for enemy in &self.enemies {
                enemy.draw(ctx)?;
            }
        
            for bullet in &self.bullets {
                bullet.draw(ctx)?
            }

            self.player.draw(ctx)?; 
            self.shop.display(ctx, &mut self.player)?;
        }
        // Present the drawn content
        graphics::present(ctx)?;

        Ok(())
    }
}
