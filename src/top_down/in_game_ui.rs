use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use sdl2::rect::Rect;

pub struct Lives<'a>
{
    texture: &'a Texture<'a>,
    amount_of_lives: u32,
    live_heart: Rect,
    dead_heart: Rect,
    tile_size: u32,
}

impl Lives<'_>
{
    pub fn new<'a>( texture: &'a Texture<'a>, amount_of_lives: u32, live_heart: Rect, dead_heart: Rect, tile_size: u32, multiplier: u32  ) -> Lives<'a>
    {
        Lives { texture, amount_of_lives, live_heart, dead_heart, tile_size: tile_size * multiplier}
    }
}
