use ggez::{Context, GameResult};
use ggez::graphics::{self, Image, DrawParam};
use std::time::Instant;

pub struct MenuParallax {
    pub sky_image: Image,
    pub mountains_image: Image,
    pub ground_image: Image,
    pub scroll_speed_sky: f32,
    pub scroll_speed_mountains: f32,
    pub scroll_speed_ground: f32,
    pub sky_offset: f32,
    pub mountains_offset: f32,
    pub ground_offset: f32,
    pub last_update: Instant,
}

impl MenuParallax {
    pub fn new(ctx: &mut Context) -> GameResult<MenuParallax> {
        let sky_image = Image::new(ctx, "/sky.png")?;
        let mountains_image = Image::new(ctx, "/mountains.png")?;
        let ground_image = Image::new(ctx, "/trees.png")?;

        Ok(MenuParallax {
            sky_image,
            mountains_image,
            ground_image,
            scroll_speed_sky: 20.0,
            scroll_speed_mountains: 40.0,
            scroll_speed_ground: 80.0,
            sky_offset: 0.0,
            mountains_offset: 0.0,
            ground_offset: 0.0,
            last_update: Instant::now(),
        })
    }

    pub fn update(&mut self) {
        let elapsed_time = self.last_update.elapsed().as_secs_f32();
        self.last_update = Instant::now();

        // Update the offsets for each layer based on elapsed time and their respective speeds
        self.sky_offset -= self.scroll_speed_sky * elapsed_time;
        self.mountains_offset -= self.scroll_speed_mountains * elapsed_time;
        self.ground_offset -= self.scroll_speed_ground * elapsed_time;

        // Loop the offsets to create a seamless scrolling effect
        if self.sky_offset <= -(self.sky_image.width() as f32) {
            self.sky_offset = 0.0;
        }
        if self.mountains_offset <= -(self.mountains_image.width() as f32) {
            self.mountains_offset = 0.0;
        }
        if self.ground_offset <= -(self.ground_image.width() as f32) {
            self.ground_offset = 0.0;
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        // Draw the sky layer
        graphics::draw(ctx, &self.sky_image, DrawParam::default().dest([self.sky_offset, 0.0]).scale([5.0, 4.1]))?;
        graphics::draw(ctx, &self.sky_image, DrawParam::default().dest([self.sky_offset + self.sky_image.width() as f32, 0.0]).scale([5.0, 4.1]))?;

        // Draw the mountains layer
        graphics::draw(ctx, &self.mountains_image, DrawParam::default().dest([self.mountains_offset, 100.0]).scale([5.0, 4.1]))?;
        graphics::draw(ctx, &self.mountains_image, DrawParam::default().dest([self.mountains_offset + self.mountains_image.width() as f32, 100.0]).scale([5.0, 4.1]))?;

        // Draw the ground layer
        graphics::draw(ctx, &self.ground_image, DrawParam::default().dest([self.ground_offset, 300.0]).scale([5.0, 4.1]))?;
        graphics::draw(ctx, &self.ground_image, DrawParam::default().dest([self.ground_offset + self.ground_image.width() as f32, 300.0]).scale([5.0, 4.1]))?;

        Ok(())
    }
}
