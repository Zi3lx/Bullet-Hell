use crate::player::Player;
use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};


pub struct Shop {
    pub health_upgrade_cost: i32,
    pub damage_upgrade_cost: i32,
    pub speed_upgrade_cost: i32,
    pub fire_rate_cost: i32,

    pub health_lvl: i32,
    pub damage_lvl: i32,
    pub speed_lvl: i32,
    pub fire_rate_lvl: i32,
}

impl Shop {
    pub fn new() -> GameResult<Shop> {
        let s = Shop {
            health_upgrade_cost: 50,
            damage_upgrade_cost: 500,
            speed_upgrade_cost: 200,
            fire_rate_cost: 400,

            health_lvl: 1,
            damage_lvl: 1,
            speed_lvl: 1,
            fire_rate_lvl: 1,
        };
        Ok(s)
    }

    pub fn display(&self, ctx: &mut Context, player: &mut Player) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));
        let text = format!(
            "Health Upgrade: {} Coins \nDamage Upgrade: {} Coins \nSpeed Upgrade: {} \nFire Rate Upgrade: {} Coins \nCoins: {}",
            self.health_upgrade_cost, self.damage_upgrade_cost, self.speed_upgrade_cost, self.fire_rate_cost, player.coins
        );
        let display_text = graphics::Text::new((text, graphics::Font::default(), 30.0));
        graphics::draw(ctx, &display_text, DrawParam::default())?;
        Ok(())
    }

    pub fn try_buy_health_upgrade(&mut self, player: &mut Player) {
        if player.coins >= self.health_upgrade_cost {
            player.coins -= self.health_upgrade_cost;
            player.hp += 5; // Upgrade health
            self.health_upgrade_cost = self.health_upgrade_cost / self.health_lvl * (self.health_lvl + 1);
            self.health_lvl += 1;
            println!("Health upgraded! New HP: {}", player.hp);
        } else {
            println!("Not enough coins for health upgrade.");
        }
    }

    pub fn try_buy_damage_upgrade(&mut self, player: &mut Player) {
        if player.coins >= self.damage_upgrade_cost {
            player.coins -= self.damage_upgrade_cost;
            player.damage += 1; // Upgrade damage
            self.damage_upgrade_cost = self.damage_upgrade_cost / self.damage_lvl * (self.damage_lvl + 1);
            self.damage_lvl += 1;
            println!("Damage upgraded! New Damage: {}", player.damage);
        } else {
            println!("Not enough coins for damage upgrade.");
        }
    }

    pub fn try_buy_speed_upgrade(&mut self, player: &mut Player) {
        if player.coins >= self.speed_upgrade_cost {
            player.coins -= self.speed_upgrade_cost;
            player.speed += 1.0; // Upgrade speed
            self.speed_upgrade_cost = self.speed_upgrade_cost / self.speed_lvl * (self.speed_lvl + 1);
            self.speed_lvl += 1;
            println!("Speed upgraded! New Speed: {}", player.speed);
        } else {
            println!("Not enough coins for speed upgrade.");
        }
    }

    pub fn try_buy_fire_rate_upgrade(&mut self, player: &mut Player) {
        if player.coins >= self.fire_rate_cost {
            player.coins -= self.fire_rate_cost;
            player.fire_rate -= 0.05; // Upgrade fire rate
            self.fire_rate_cost = self.fire_rate_cost / self.fire_rate_lvl * (self.fire_rate_lvl + 1);
            self.fire_rate_lvl += 1;
            println!("Fire rate upgraded! New Fire Rate: {}", player.fire_rate);
        } else {
            println!("Not enough coins for fire rate upgrade.");
        }
    }
}
