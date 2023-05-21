// ToDo:
// - minimal playable game 

use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use std::vec::Vec;
use std::collections::HashMap;

pub struct Game<'a>
{
    camera: Camera,
    player: Player<'a>,
    tile_map: TileMap<'a>,
    
}

impl Game<'_>
{
    pub fn new<'a>(texture: &'a Texture, tile_size: u32,
               camera_mode: CameraMode, camera_x: i32, camera_y: i32,
               player_location: Rect, player_texture_location: Rect,
               tiles: Vec<Vec<u32>>, tile_mode: HashMap<u32, Tile<'a>>, x_tiles: u32, y_tiles: u32,
               ) -> Game<'a>
    {
        let camera = Camera { camera_mode, x: camera_x, y: camera_y };
        let player = Player { speed: tile_size, location: player_location, texture_location: player_texture_location, texture };
        let tile_map = TileMap { tiles, tile_mode, x_tiles, y_tiles, tile_size };

        Game
        {
            camera,
            player,
            tile_map
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        self.tile_map.draw(canvas)?;
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

    fn move_tile (&self)
    {
        match self.camera_mode
        {
            CameraMode::FollowPlayer => 
            {
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
    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        //for tile_vec in &self.tiles
        //{
        //    for tile in tile_vec
        //    {
        //        // spaggeti 
        //        canvas.copy(&self.tile_mode[tile].texture , Some(self.tile_mode[tile].get_texture_location()), Some(Rect::new(x, y, self.tile_size, self.tile_size)))?;
        //    }
        //}
        for x in 0..self.x_tiles
        {
            for y in 0..self.y_tiles
            {
                let tile = self.tiles[y as usize][x as usize];
                canvas.copy(&self.tile_mode[&tile].texture, Some(self.tile_mode[&tile].get_texture_location()), Some(Rect::new((x * self.tile_size) as i32, (y * self.tile_size) as i32, self.tile_size, self.tile_size)))?;
            }
        }
        Ok(())
    }
}

