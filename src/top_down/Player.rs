use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use crate::Camera::Camera;
use crate::TileMap::TileMap;

pub struct Player<'a> 
{
    speed: i32,
    tile_size: u32,
    location: Rect,
    texture_location: Rect,
    texture: &'a Texture<'a>,
    
    is_moving: bool,
    moving_to: Rect,
}

impl Player<'_>
{
    pub fn new<'a>(tile_size: u32, speed: i32, location: Rect, texture_location: Rect, texture: &'a Texture<'a>) -> Player
    {
        Player
        {
            speed,
            tile_size,
            location,
            texture_location,
            texture,

            is_moving: false,
            moving_to: location,
        }
    }

    pub fn draw(&self, camera: &Camera, screen_width: u32, screen_heigt: u32, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        match camera.camera_mode
        {
            CameraMode::FollowPlayer =>
            {
                canvas.copy(&self.texture, Some(self.texture_location),
                Some(Rect::new((screen_width/2-self.location.width()/2) as i32,
                (screen_heigt/2-self.location.height()/2) as i32, 
                self.location.width(), self.location.height())))?;
            },
            CameraMode::StaticLocation => ()
        }
        Ok(())
    }

    pub fn get_input(&mut self, tile_map: &TileMap, keycode: Keycode, up_key: Keycode, down_key: Keycode, right_key: Keycode, left_key: Keycode)
    {
        match keycode
        {
            key if key == up_key =>
            {
                if !self.is_moving
                {
                    self.moving_to.y -= self.tile_size as i32;
                    self.is_moving = true;
                }
            },
            key if key == down_key =>
            {
                if !self.is_moving
                {
                    self.moving_to.y += self.tile_size as i32;
                    self.is_moving = true;
                }
            },
            key if key == right_key =>
            {
                if !self.is_moving
                {
                    self.moving_to.x += self.tile_size as i32;
                    self.is_moving = true;
                }
            },
            key if key == left_key =>
            {
                if !self.is_moving
                {
                    self.moving_to.x -= self.tile_size as i32;
                    self.is_moving = true;
                }
            },
            _ => ()
        }
        match tile_map.get_tile(self.moving_to.x as u32 / self.tile_size, self.moving_to.y as u32 / self.tile_size).get_hitbox()
        {
            TileHitBox::Full =>
            {
                self.moving_to = self.location;
            },
            _ => (),
        }
    }

    pub fn move_player(&mut self)
    {
        if self.location.x == self.moving_to.x && self.location.y == self.moving_to.y
        {
            self.is_moving = false;
            return;
        }
        if self.location.x > self.moving_to.x
        {
            self.location.x -= self.speed;
        }
        else if self.location.x < self.moving_to.x {
            self.location.x += self.speed;
        }
        if self.location.y > self.moving_to.y
        {
            self.location.y -= self.speed;
        }
        else if self.location.y < self.moving_to.y {
            self.location.y += self.speed;
        }
    }

    pub fn get_location(&self) -> Rect
    {
        self.location
    }
}
