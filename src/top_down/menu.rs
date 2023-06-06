use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::{Texture, Canvas};
use sdl2::mouse::MouseState;

pub struct Menu<'a>
{
    tile_size: u32,
    texture: &'a Texture<'a>,
    play_src: Rect,
    play_dst: Rect,
    logo_src: Rect,
    logo_dst: Rect,
    tile_src: Rect,
}

impl Menu<'_>
{
    pub fn new<'a>( tile_size: u32, multiplier: u32, texture: &'a Texture<'a>, play_src: Rect, play_dst: Rect, logo_src: Rect, logo_dst: Rect, tile_src: Rect ) -> Menu<'a>
    {
        Menu { tile_size: tile_size * multiplier, texture, play_src, play_dst, logo_src, logo_dst, tile_src}
    }

    pub fn draw( &self, canvas: &mut Canvas<Window> ) -> Result<(), String>
    {
        for x in 0..100
        {
            for y in 0..100
            {
                canvas.copy(self.texture, self.tile_src, Rect::new(x * self.tile_size as i32, y * self.tile_size as i32, self.tile_size, self.tile_size))?;
            }
        }
        canvas.copy(self.texture, self.play_src, self.play_dst)?;
        canvas.copy(self.texture, self.logo_src, self.logo_dst)?;
        Ok(())
    }

    pub fn get_input( &mut self, mouse_state: MouseState ) -> bool
    {
        if mouse_state.x() >= self.play_dst.x + self.play_dst.width() as i32 ||
           mouse_state.x() <= self.play_dst.x
        {
            return false;
        }
        if mouse_state.y() >= self.play_dst.y + self.play_dst.height() as i32 ||
           mouse_state.y() <= self.play_dst.y
        {
            return false;
        }

        if mouse_state.left()
        {
            return true;
        }
        return false;
    }
}
