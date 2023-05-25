use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use crate::camera::{Camera, CameraMode};
use crate::tile_map::{TileMap, TileHitBox};

use std::collections::HashMap;

use super::animation::Animation;

pub struct Player<'a> 
{
    speed: i32,
    tile_size: u32,
    location: Rect,
    animations: HashMap<u32, Animation<'a>>,
    direction: u32,
    // movement variables
    is_moving: bool,
    moving_to: Rect,
}

impl Player<'_>
{
    pub fn new<'a>(tile_size: u32, multiplier: u32, speed: i32, location: Rect, animations: HashMap<u32, Animation<'a>>) -> Player<'a>
    {
        Player
        {
            speed,
            tile_size: tile_size * multiplier,
            location,
            animations,
            direction: 3,
            // movement variables
            is_moving: false,
            moving_to: location,
        }
    }

    pub fn draw(&mut self, camera: &Camera, screen_width: u32, screen_heigt: u32, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        match camera.get_camera_mode()
        {
            CameraMode::FollowPlayer =>
            {
                self.animations.get_mut(&self.direction).map(|val| val.draw(canvas,
                            // putting player in the center of the screen
                            Rect::new(( screen_width / 2 - self.location.width() / 2 ) as i32,
                                      ( screen_heigt / 2 - self.location.height() / 2 ) as i32, 
                                      self.location.width(), self.location.height())));
            },
            CameraMode::StaticLocation => 
            {
                self.animations.get_mut(&self.direction).map(|val| val.draw(canvas,
                            // drawing the player according to the camera
                            Rect::new(self.location.x - camera.get_x(),
                                      self.location.y - camera.get_y(),
                                      self.location.width(), self.location.height())));
            }
        }
        Ok(())
    }

    pub fn get_input(&mut self, tile_map: &TileMap, keycode: Keycode, up_key: Keycode, down_key: Keycode, right_key: Keycode, left_key: Keycode)
    {
        if !self.is_moving
        {
            match keycode
            {
                key if key == up_key =>
                {
                    self.moving_to.y -= self.tile_size as i32;
                    self.is_moving = true;
                    self.direction = 1;
                },
                key if key == down_key =>
                {
                    self.moving_to.y += self.tile_size as i32;
                    self.is_moving = true;
                    self.direction = 3;
                },
                key if key == right_key =>
                {
                    self.moving_to.x += self.tile_size as i32;
                    self.is_moving = true;
                    self.direction = 2;
                },
                key if key == left_key =>
                {
                    self.moving_to.x -= self.tile_size as i32;
                    self.is_moving = true;
                    self.direction = 4;
                },
                _ => ()
            }
        }
        // returning moveto location to player if wanting to move in a tile
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
        // checking if at the right location
        if self.location.x == self.moving_to.x && self.location.y == self.moving_to.y
        {
            self.is_moving = false;
            return;
        }

        // moving x to the right location
        if self.location.x > self.moving_to.x
        {
            self.location.x -= self.speed;
        }
        else if self.location.x < self.moving_to.x {
            self.location.x += self.speed;
        }

        // moving y to the right location
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


