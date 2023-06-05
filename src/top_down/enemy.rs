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
    moving_to: Rect,
    multiplier: u32,
    is_moving: bool,
    animation: Animation<'a>,
}

impl Enemy<'_>
{
    pub fn new<'a>( location: Rect, animation: Animation<'a>, multiplier: u32 ) -> Enemy
    {
        Enemy { multiplier, location, moving_to: location, animation, is_moving: false }
    }

    pub fn draw( &mut self, camera: &Camera, canvas: &mut Canvas<Window> ) -> Result<(), String>
    {
        self.animation.draw(canvas, Rect::new(self.location.x - camera.get_x(), self.location.y - camera.get_y(), self.location.width() * self.multiplier, self.location.height() * self.multiplier))?;
        Ok(())
    }

    pub fn go_to( &mut self, to: Point, tile_map: &TileMap, solid_tiles: &[u32]) 
    {
        if self.is_moving 
        {
            return;
        }

        self.is_moving = true;

        let path = find_path(Point::new(self.location.x, self.location.y), to, tile_map, solid_tiles, 64);

        if path.len() != 0
        {
            match path[0] {
                1 => self.moving_to.y -= 32 * 2,
                2 => self.moving_to.x += 32 * 2,
                3 => self.moving_to.y += 32 * 2,
                4 => self.moving_to.x -= 32 * 2,
                _ => (),
            }
        }
    }
    
    pub fn move_enemy( &mut self, speed: i32 )
    {
        // checking if at the right location
        if self.location.x == self.moving_to.x && self.location.y == self.moving_to.y
        {
            self.is_moving = false;
            return;
        }

        // moving x to the right location
        if self.location.x > self.moving_to.x
        {
            self.location.x -= speed;
        }
        else if self.location.x < self.moving_to.x {
            self.location.x += speed;
        }

        // moving y to the right location
        if self.location.y > self.moving_to.y
        {
            self.location.y -= speed;
        }
        else if self.location.y < self.moving_to.y {
            self.location.y += speed;
        }
    }
}

