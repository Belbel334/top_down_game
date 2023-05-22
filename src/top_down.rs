// ToDo:
// - smooth player movement

use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

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
                
                self.x = player_location.x - screen_width as i32 / 2 + player_location.width() as i32 / 2;
                self.y = player_location.y - screen_heigt as i32 / 2 + player_location.height() as i32 / 2;
            }
            CameraMode::StaticLocation => ()
        }
    }
}

pub struct Player<'a> 
{
    speed: u32,
    tile_size: u32,
    location: Rect,
    texture_location: Rect,
    texture: &'a Texture<'a>,
    
    is_moving: bool,
    has_moved: u32,
    direction: u8,
}

impl Player<'_>
{
    pub fn new<'a>(tile_size: u32, speed: u32, location: Rect, texture_location: Rect, texture: &'a Texture<'a>) -> Player
    {
        Player
        {
            speed,
            tile_size,
            location,
            texture_location,
            texture,

            is_moving: false,
            has_moved: 0, // amount of pixels moved
            direction: 0, // 0: no movement, 1: up, 2: right, 3: down, 4: left
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

    pub fn get_input(&mut self, keycode: Keycode, up_key: Keycode, down_key: Keycode, right_key: Keycode, left_key: Keycode)
    {
        match keycode
        {
            key if key == up_key =>
            {
                if !self.is_moving
                {
                    self.direction = 1;
                    self.is_moving = true;
                }
            },
            key if key == down_key =>
            {
                if !self.is_moving
                {
                    self.direction = 3;
                    self.is_moving = true;
                }
            },
            key if key == right_key =>
            {
                if !self.is_moving
                {
                    self.direction = 2;
                    self.is_moving = true;
                }
            },
            key if key == left_key =>
            {
                if !self.is_moving
                {
                    self.direction = 4;
                    self.is_moving = true;
                }
            },
            _ => ()
        }
    }

    pub fn move_player(&mut self, tile_map: &TileMap)
    {
        println!("{}, {}", self.location.x as u32 / self.tile_size, self.location.y as u32 / self.tile_size);
        match tile_map.get_tile( self.location.x as u32 / self.tile_size, self.location.y as u32 / self.tile_size ).get_hitbox()
        {
                    &TileHitBox::Full => 
                    {
                        println!("FULL");
                    },
                    _ => (),
        }

        match self.direction
        {
            1 =>
            {
                self.location.y -= self.speed as i32; 
                self.has_moved += self.speed; 

                match tile_map.get_tile( self.location.x as u32 / self.tile_size, self.location.y as u32 / self.tile_size ).get_hitbox()
                {
                    &TileHitBox::Full => 
                    {
                        self.direction = 0;
                        self.has_moved = 0;
                        self.is_moving = false;
                        self.location.y += self.speed as i32; 
                    },
                    _ => (),
                }
                
                if self.has_moved >= self.tile_size
                {
                    self.direction = 0;
                    self.has_moved = 0;
                        self.is_moving = false;
                }
            },
            2 =>
            {
                self.location.x += self.speed as i32; 
                self.has_moved += self.speed; 

                match tile_map.get_tile(self.location.x as u32 / self.tile_size + 1, self.location.y as u32 / self.tile_size).get_hitbox()
                {
                    &TileHitBox::Full => 
                    {
                        self.direction = 0;
                        self.is_moving = false;
                        self.location.x -= self.speed as i32; 
                        self.has_moved = 0;
                    },
                    _ => (),
                }
                
                if self.has_moved >= self.tile_size
                {
                    self.direction = 0;
                    self.has_moved = 0;
                        self.is_moving = false;
                }
            },
            3 =>
            {
                self.location.y += self.speed as i32; 
                self.has_moved += self.speed; 

                match tile_map.get_tile(self.location.x as u32 / self.tile_size, self.location.y as u32 / self.tile_size + 1).get_hitbox()
                {
                    &TileHitBox::Full => 
                    {
                        self.direction = 0;
                        self.is_moving = false;
                        self.location.y -= self.speed as i32; 
                        self.has_moved = 0;
                    },
                    _ => (),
                }
                
                if self.has_moved >= self.tile_size
                {
                    self.direction = 0;
                    self.has_moved = 0;
                        self.is_moving = false;
                }
            },
            4 =>
            {
                self.location.x -= self.speed as i32; 
                self.has_moved += self.speed; 

                match tile_map.get_tile(self.location.x as u32 / self.tile_size, self.location.y as u32 / self.tile_size).get_hitbox()
                {
                    &TileHitBox::Full => 
                    {
                        self.direction = 0;
                        self.is_moving = false;
                        self.location.x += self.speed as i32; 
                        self.has_moved = 0;
                    },
                    _ => (),
                }
                
                if self.has_moved >= self.tile_size
                {
                    self.direction = 0;
                        self.is_moving = false;
                    self.has_moved = 0;
                }
                },
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

