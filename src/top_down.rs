// ToDo:
// - 

use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use std::vec::Vec;

pub struct Game<'a>
{
    tile_size: u32,
    pub camera: Camera,
    pub player: Player<'a>,
    pub tile_map: TileMap,
    
}

impl Game<'_>
{
    pub fn new()
    {
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        self.player.draw(canvas)?;
        Ok(())
    }
}

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
    fn new (camera_mode: CameraMode, x: i32, y: i32) -> Camera
    {
        Camera {
            camera_mode,
            x,
            y,
        }
    }

    fn move_tile (&self, tile: &mut Tile)
    {
        match self.camera_mode
        {
            CameraMode::FollowPlayer => 
            {
                tile.location.x -= self.x;
                tile.location.y -= self.y;
            },
            CameraMode::StaticLocation => (),
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
    pub fn new<'a> (speed: u32, location: Rect, texture_location: Rect, texture: &'a Texture<'a> ) -> Player<'a>
    {
        Player { speed, location, texture_location, texture }
    }
        
    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        canvas.copy(&self.texture, Some(self.texture_location), Some(self.location))?;
        Ok(())
    }

    pub fn move_player(&mut self, event: Event)
    {
        self.location.x += 5;
        match event
        {
            Event::KeyDown{ keycode: Option::Some(Keycode::Up) , .. } => println!("mf"),
            _ => ()
        }
    }
}

pub enum TileHitBox
{
    Full,
    None,
}

pub struct Tile<'a>
{
    location: Rect,
    texture_location: Rect,
    texture: &'a Texture<'a>,
}

impl Tile<'_>
{
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        canvas.copy(&self.texture, Some(self.texture_location), Some(self.location))?;
        Ok(())
    }
}

pub struct TileMap
{
    tiles: Vec<Vec<u32>>,
    x_tiles: u32,
    y_tiles: u32,
}

impl TileMap
{
    pub fn new<'a>(tiles: Vec<Vec<Tile<'a>>>)
    {
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        //for tile_vec in &self.tiles
        //{
        //    for tile in tile_vec
        //    {
        //        tile.draw(canvas)?;
        //    }
        //}
        Ok(())
    }
}

