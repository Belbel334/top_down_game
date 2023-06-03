use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use crate::camera::Camera;

use super::animation::Animation;

pub struct Enemy<'a>
{
    location: Rect,
    multiplier: u32,
    animation: Animation<'a>,
}

impl Enemy<'_>
{
    pub fn new<'a>( location: Rect, animation: Animation<'a>, multiplier: u32 ) -> Enemy
    {
        Enemy { multiplier, location, animation }
    }

    pub fn draw( &mut self, camera: &Camera, canvas: &mut Canvas<Window> ) -> Result<(), String>
    {
        self.animation.draw(canvas, Rect::new(self.location.x - camera.get_x(), self.location.y - camera.get_y(), self.location.width() * self.multiplier, self.location.height() * self.multiplier))?;
        Ok(())
    }
}
