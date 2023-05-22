extern crate sdl2;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::vec::Vec;
use std::path::Path;
use std::collections::HashMap;


mod top_down;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    
    let tile_size: u32 = 64;

    let screen_width: u32 = 13 * tile_size;
    let screen_height: u32 = 9 * tile_size;

    let window = video_subsystem
        .window("Top Down Game", screen_width, screen_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let texture = texture_creator.load_texture(Path::new("textures.png"))?;

    let tiles = Vec::from([
                          Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                          Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                          Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                          Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                          Vec::from([0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0]),
                          Vec::from([0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0]),
                          Vec::from([0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0]),
                          Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                          Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                          Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    ]);

    let tile_mode = HashMap::from([(0, top_down::Tile::new(top_down::TileHitBox::None, Rect::new(32, 0, 32, 32), &texture)),
                                   (1, top_down::Tile::new(top_down::TileHitBox::Full, Rect::new(0, 32, 32, 32), &texture))]);

    let mut camera = top_down::Camera::new(top_down::CameraMode::FollowPlayer, 64, 64);

    let mut player = top_down::Player::new(64, Rect::new(128, 128, 64, 64), Rect::new(0, 0, 32, 32), &texture);

    let tile_map = top_down::TileMap::new(tiles, tile_mode, 13, 9, tile_size);

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {

            match event {
                Event::Quit { .. } => break 'mainloop,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    player.move_player(&tile_map, keycode, Keycode::Up, Keycode::Down, Keycode::Right, Keycode::Left);
                }
                _ => {}
            }
        }
        camera.move_camera(&player, screen_width, screen_height);

        canvas.set_draw_color(Color::RGB(45, 45, 45));
        canvas.clear();

        tile_map.draw(&camera, &mut canvas)?;
        player.draw(&camera, screen_width, screen_height, &mut canvas)?;

        canvas.present();
    }

    Ok(())
}

