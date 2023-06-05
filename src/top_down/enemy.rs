use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{Rect, Point};

use crate::camera::Camera;

use super::animation::Animation;
use super::path_finder::find_path;
use super::tile_map::TileMap;

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

    pub fn go_to( &mut self, to: Point, tile_map: &TileMap, solid_tiles: &[u32]) 
    {
        let path = find_path(Point::new(self.location.x, self.location.y), to, tile_map, solid_tiles, 32);

        if path.len() != 0
        {
            match path[0] {
                1 => self.location.y -= 32 * 2,
                2 => self.location.x += 32 * 2,
                3 => self.location.y += 32 * 2,
                4 => self.location.x -= 32 * 2,
                _ => (),
            }
        }

    }
}


