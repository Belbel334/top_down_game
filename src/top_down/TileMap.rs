use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use std::vec::Vec;
use std::collections::HashMap;

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

    pub fn get_hitbox(&self) -> &TileHitBox
    {
        &self.hitbox
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

    pub fn get_tile(&self, x: u32, y: u32) -> &Tile
    {
        &self.tile_mode[&self.tiles[y as usize][x as usize]]
    }
}

