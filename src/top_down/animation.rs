use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use sdl2::rect::Rect;

pub struct Animation<'a>
{
    texture: &'a Texture<'a>,
    frame_locations: Vec<Rect>,
    length: u32,
    frame_delay: u32, 
    frame: u32,
}

impl Animation<'_>
{
    pub fn new<'a>(texture: &'a Texture<'a>, first_frame_x: i32, first_frame_y: i32, tile_size: u32, length: u32, frame_delay: u32) -> Animation
    {
        let mut frame_locations: Vec<Rect> = vec![];
        for frame in 0..length 
        {
            frame_locations.push(Rect::new(( tile_size * frame ) as i32 + first_frame_x,
                                           first_frame_y,
                                           tile_size,
                                           tile_size));
        }
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
