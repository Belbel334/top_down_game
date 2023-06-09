use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::{Rect, Point};

use crate::camera::Camera;

use super::animation::Animation;
use super::path_finder::find_path;
use super::tile_map::TileMap;

pub struct Enemy<'a>
{
    tile_size: u32,
    multiplier: u32,
    location: Rect,
    moving_to: Rect,
    is_moving: bool,
    allert_range: f32,
    animation: Animation<'a>,
}

impl Enemy<'_>
{
    pub fn new<'a>( location: Rect, animation: Animation<'a>, tile_size: u32, multiplier: u32, allert_range: f32 ) -> Enemy
    {
        Enemy { tile_size: tile_size * multiplier, multiplier, location, moving_to: location, animation, is_moving: false, allert_range }
    }

    pub fn draw( &mut self, camera: &Camera, canvas: &mut Canvas<Window> ) -> Result<(), String>
    {
        self.animation.draw(canvas, Rect::new(self.location.x - camera.get_x(), self.location.y - camera.get_y(), self.location.width() * self.multiplier, self.location.height() * self.multiplier))?;
        Ok(())
    }

    pub fn go_to( &mut self, to: Point, tile_map: &TileMap, solid_tiles: &[u32] ) 
    {
        let distance = (((to.x - self.location.x) * (to.x - self.location.x) + (to.y - self.location.y) * (to.y - self.location.y)) as f32).sqrt();

        if distance > self.allert_range
        {
            return;
        }

        if self.is_moving 
        {
            return;
        }

        self.is_moving = true;

        let path = find_path(Point::new(self.location.x, self.location.y), to, tile_map, solid_tiles, self.tile_size as i32);

        if path.len() != 0
        {
            match path[0] {
                1 => self.moving_to.y -= self.tile_size as i32,
                2 => self.moving_to.x += self.tile_size as i32,
                3 => self.moving_to.y += self.tile_size as i32,
                4 => self.moving_to.x -= self.tile_size as i32,
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

    pub fn get_location( &self ) -> Rect
    {
        self.location
    }
}

