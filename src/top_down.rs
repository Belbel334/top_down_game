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

pub struct Player<'a> 
{
    speed: u32,
    location: Rect,
    texture_location: Rect,
    texture: &'a Texture<'a>,
}

enum CameraMode {
    FollowPlayer,
    OnePlace,
}

struct Camera {
    camera_mode: CameraMode,
    x: i32,
    y: i32,
}

impl Camera {

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

struct Obstacle<'a>
{
    location: Rect,
    texture_location: Rect,
    texture: &'a Texture<'a>,
}

impl Obstacle<'_>
{
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        canvas.copy(&self.texture, Some(self.texture_location), Some(self.location))?;
        Ok(())
    }
}
