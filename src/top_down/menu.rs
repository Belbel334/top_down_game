use std::collections::TryReserveError;

use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::{Texture, Canvas};
use sdl2::mouse::MouseState;

pub struct Menu<'a>
{
    texture: &'a Texture<'a>,
    play_src: Rect,
    play_dst: Rect,
    logo_src: Rect,
    logo_dst: Rect,
    bg_color: Color,
}

impl Menu<'_>
{
    pub fn new<'a>( texture: &'a Texture<'a>, play_src: Rect, play_dst: Rect, logo_src: Rect, logo_dst: Rect, bg_color: Color ) -> Menu<'a>
    {
        Menu { texture, play_src, play_dst, logo_src, logo_dst, bg_color}
    }

    pub fn draw( &self, canvas: &mut Canvas<Window> ) -> Result<(), String>
    {
        canvas.set_draw_color(self.bg_color);
        canvas.clear();
        canvas.copy(self.texture, self.play_src, self.play_dst)?;
        canvas.copy(self.texture, self.logo_src, self.logo_dst)?;
        Ok(())
    }

    pub fn get_input( &mut self, mouse_state: MouseState ) -> bool
    {
        if mouse_state.left()
        {
            return true;
        }
        return false;
    }
}
