use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use sdl2::rect::Rect;

pub struct Animation<'a>
{
    texture: Texture<'a>,
    frame_locations: Vec<Rect>,
    length: u32,
    frame_delay: u32, 
    frame: u32,
}

impl Animation<'_>
{
    pub fn new<'a>(texture: Texture<'a>, frame_locations: Vec<Rect>, length: u32, frame_delay: u32) -> Animation
    {
        Animation { texture, frame_locations, length: length - 1, frame_delay, frame: 0 }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, location: Rect) -> Result<(), String>
    {
        canvas.copy(&self.texture, self.frame_locations[(self.frame / self.frame_delay) as usize], location)?;

        self.frame += 1;

        if self.frame / self.frame_delay > self.length
        {
            self.frame = 0;
        }

        Ok(())
    }
}
