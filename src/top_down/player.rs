use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::keyboard::{Scancode, KeyboardState};

use crate::tile_map::{TileMap, TileHitBox};
use crate::animation::Animation;
use crate::top_down::in_game_ui::Lives;

use std::collections::HashMap;

use super::enemy::Enemy;

pub struct Player<'a> 
{
    speed: i32,
    tile_size: u32,
    location: Rect,
    idle_animations: HashMap<u32, Animation<'a>>,
    run_animations: HashMap<u32, Animation<'a>>,
    direction: u32,
    lives: Lives<'a>,
    // movement variables
    is_moving: bool,
    moving_to: Rect,
}

impl Player<'_>
{
    pub fn new<'a>(tile_size: u32, multiplier: u32, speed: i32, location: Rect, idle_animations: HashMap<u32, Animation<'a>>, run_animations: HashMap<u32, Animation<'a>>, lives: Lives<'a>) -> Player<'a>
    {
        Player
        {
            speed,
            tile_size: tile_size * multiplier,
            location,
            idle_animations,
            run_animations,
            direction: 3,
            lives,
            // movement variables
            is_moving: false,
            moving_to: location,
        }
    }

    pub fn draw(&mut self, screen_width: u32, screen_heigt: u32, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        match self.is_moving {
            true =>
            {
                self.run_animations.get_mut(&self.direction).map(|val| val.draw(canvas,
                            // putting player in the center of the screen
                           Rect::new(( screen_width / 2 - self.location.width() / 2 ) as i32,
                                     ( screen_heigt / 2 - self.location.height() / 2 ) as i32, 
                                     self.location.width(), self.location.height())));
           },
           false =>
           {
               self.idle_animations.get_mut(&self.direction).map(|val| val.draw(canvas,
                           // putting player in the center of the screen
                           Rect::new(( screen_width / 2 - self.location.width() / 2 ) as i32,
                                     ( screen_heigt / 2 - self.location.height() / 2 ) as i32, 
                                     self.location.width(), self.location.height())));
           },
        }
        self.lives.draw(canvas)?;
        Ok(())
    }

    pub fn get_input(&mut self, tile_map: &TileMap, event: KeyboardState, up_key: Scancode, down_key: Scancode, right_key: Scancode, left_key: Scancode)
    {
        if !self.is_moving
        {
            if event.is_scancode_pressed(up_key)
            {
                self.moving_to.y -= self.tile_size as i32;
                self.is_moving = true;
                self.direction = 1;
            }
            else if event.is_scancode_pressed(down_key)
            {
                self.moving_to.y += self.tile_size as i32;
                self.is_moving = true;
                self.direction = 3;
            }
            else if event.is_scancode_pressed(right_key)
            {
                self.moving_to.x += self.tile_size as i32;
                self.is_moving = true;
                self.direction = 2;
            }
            else if event.is_scancode_pressed(left_key)
            {
                self.moving_to.x -= self.tile_size as i32;
                self.is_moving = true;
                self.direction = 4;
            }
        }
        else 
        {
            for i in 0..self.idle_animations.len() as u32
            {
                // get rid of headbob by resetting the animations
                self.idle_animations.get_mut(&(i + 1)).map(|val| val.reset_timing());
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

    pub fn take_damage( &mut self, enemy: &Enemy ) -> bool
    {
        if (self.location.x < enemy.get_location().x || self.location.x > enemy.get_location().x + self.tile_size as i32) &&
           (self.location.x + (self.tile_size as i32) < enemy.get_location().x || self.location.x + (self.tile_size as i32) > enemy.get_location().x + self.tile_size as i32)
        {
            return false;
        }
        if (self.location.y < enemy.get_location().y || self.location.y > enemy.get_location().y + self.tile_size as i32) &&
           (self.location.y + (self.tile_size as i32) < enemy.get_location().y || self.location.y + (self.tile_size as i32) > enemy.get_location().y + self.tile_size as i32)
        {
            return false;
        }
        self.lives.take_damage(1);
        return true;
    }

    pub fn check_alive( &mut self ) -> bool
    {
        if self.lives.get_amount_of_lives() == 0
        {
            return false;
        }
        return true;
    }

    pub fn get_location(&self) -> Rect
    {
        self.location
    }

    pub fn is_moving(&self) -> bool
    {
        self.is_moving
    }
}
