use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use sdl2::rect::Rect;

pub struct Lives<'a>
{
    texture: &'a Texture<'a>,
    amount_of_lives: u32,
    max_lives: u32,
    live_heart: Rect,
    dead_heart: Rect,
    tile_size: u32,
}

impl Lives<'_>
{
    pub fn new<'a>( texture: &'a Texture<'a>, amount_of_lives: u32, live_heart: Rect, dead_heart: Rect, tile_size: u32, multiplier: u32  ) -> Lives<'a>
    {
        Lives { texture, amount_of_lives, max_lives: amount_of_lives, live_heart, dead_heart, tile_size: tile_size * multiplier}
    }

    pub fn get_amount_of_lives( &self ) -> u32
    {
        self.amount_of_lives
    }
    
    pub fn draw( &mut self, canvas: &mut Canvas<Window>  ) -> Result<(), String>
    {
        for live_heart in 0..self.amount_of_lives
        {
            canvas.copy(&self.texture, self.live_heart, Rect::new((live_heart * self.tile_size) as i32, 0, self.tile_size, self.tile_size))?;
        }
        for dead_heart in self.amount_of_lives..self.max_lives
        {
            canvas.copy(&self.texture, self.dead_heart, Rect::new((dead_heart * self.tile_size) as i32, 0, self.tile_size, self.tile_size))?;
        }
        Ok(())
    }
    pub fn take_damage( &mut self, amount: u32 )
    {
        if self.amount_of_lives == 0
        {
            return;
        }
        self.amount_of_lives -= amount;
    }
}
