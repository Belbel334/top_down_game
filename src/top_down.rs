use sdl2::render::Texture;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

pub struct Game<'a>
{
    tile_size: u32,
    player: Player<'a>,
}

enum CameraMode {
    FollowPlayer,
    StaticLocation,
}

struct Camera {
    camera_mode: CameraMode,
    x: i32,
    y: i32,
}

impl Camera {
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

enum TileHitBox
{
    Full,
    None,
}

struct Tile<'a>
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
