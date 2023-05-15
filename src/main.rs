extern crate sdl2;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::path::Path;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    
    let tile_size: i32 = 64;

    let window = video_subsystem
        .window("Image Test", 13 * tile_size as u32, 7 * tile_size as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let texture = texture_creator.load_texture(Path::new("textures.png"));

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                Event::KeyDown {keycode: Option::Some(Keycode::Up), ..} => (),
                _ => {}
            }

            canvas.set_draw_color(Color::RGB(45, 45, 45));
            canvas.clear();

            canvas.present();
        }
    }

    Ok(())
}

mod top_down
{
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
}
