// ToDo:
// - alling player and tilemap
// - custom player input

use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use std::vec::Vec;
use std::collections::HashMap;

pub enum CameraMode {
    FollowPlayer,
    StaticLocation,
}

pub struct Camera {
    camera_mode: CameraMode,
    x: i32,
    y: i32,
}

impl Camera {
    pub fn new (camera_mode: CameraMode, x: i32, y: i32) -> Camera
    {
        Camera { camera_mode, x, y }
    }

    pub fn get_mode(&self) -> &CameraMode
    {
        &self.camera_mode
    }

    pub fn get_x(&self) -> i32 
    {
        self.x
    }
    
    pub fn get_y(&self) -> i32 
    {
        self.y
    }

    pub fn move_camera(&mut self, player: &Player, screen_width: u32, screen_heigt: u32)
    {
        match self.camera_mode
        {
            CameraMode::FollowPlayer =>
            {
                let player_location = player.get_location();
                
                self.x = player_location.x - screen_width as i32 / 2;
                self.y = player_location.y - screen_heigt as i32 / 2;
            }
            CameraMode::StaticLocation => ()
        }
    }
}

pub struct Player<'a> 
{
    speed: u32,
    location: Rect,
    texture_location: Rect,
    texture: &'a Texture<'a>,
}

impl Player<'_>
{
    pub fn new<'a>(speed: u32, location: Rect, texture_location: Rect, texture: &'a Texture<'a>) -> Player
    {
        Player
        {
            speed,
            location,
            texture_location,
            texture
        }
    }

    pub fn draw(&self, camera: &Camera, screen_width: u32, screen_heigt: u32, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        match camera.camera_mode
        {
            CameraMode::FollowPlayer =>
            {
                canvas.copy(&self.texture, Some(self.texture_location), Some(Rect::new((screen_width/2-self.location.width()/2) as i32, (screen_heigt/2-self.location.height()/2) as i32, self.location.width(), self.location.height())))?;
            },
            CameraMode::StaticLocation => ()
        }
        Ok(())
    }

    pub fn move_player(&mut self, event: &Event, up_key: Keycode, down_key: Keycode, right_key: Keycode, left_key: Keycode)
    {
        match &event
        {
            Event::KeyDown{ keycode: Option::Some(Keycode::Up) , .. } => self.location.y -= self.speed as i32,
            Event::KeyDown{ keycode: Option::Some(Keycode::Down) , .. } => self.location.y += self.speed as i32,
            Event::KeyDown{ keycode: Option::Some(Keycode::Right) , .. } => self.location.x += self.speed as i32,
            Event::KeyDown{ keycode: Option::Some(Keycode::Left) , .. } => self.location.x -= self.speed as i32,
            _ => ()
        }
    }

    pub fn get_location(&self) -> Rect
    {
        self.location
    }
}

pub enum TileHitBox
{
    Full,
    None,
}

pub struct Tile<'a>
{
    hitbox: TileHitBox,
    texture_location: Rect,
    texture: &'a Texture<'a>,
}

impl Tile<'_>
{
    pub fn new<'a>(hitbox: TileHitBox, texture_location: Rect, texture: &'a Texture<'a>) -> Tile
    {
        Tile 
        {
            hitbox,
            texture_location,
            texture
        }
    }
    pub fn get_texture_location(&self) -> Rect
    {
        self.texture_location
    }
}

pub struct TileMap<'a>
{
    tiles: Vec<Vec<u32>>,
    tile_mode: HashMap<u32, Tile<'a>>,
    x_tiles: u32,
    y_tiles: u32,
    tile_size: u32,
}

impl TileMap<'_>
{
    pub fn new<'a>( tiles: Vec<Vec<u32>>, tile_mode: HashMap<u32, Tile<'a>>, x_tiles: u32, y_tiles: u32, tile_size: u32) -> TileMap<'a>
    {
        TileMap
        {
            tiles,
            tile_mode,
            x_tiles,
            y_tiles,
            tile_size
        }
    }

    pub fn draw(&self, camera: &Camera, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        for x in 0..self.x_tiles
        {
            for y in 0..self.y_tiles
            {
                let tile = self.tiles[y as usize][x as usize];

                canvas.copy(&self.tile_mode[&tile].texture,
                            Some(self.tile_mode[&tile].get_texture_location()),
                            Some(Rect::new((x * self.tile_size) as i32 - camera.get_x(), (y * self.tile_size) as i32 - camera.get_y(),
                            self.tile_size, self.tile_size)))?;
            }
        }
        Ok(())
    }
}

