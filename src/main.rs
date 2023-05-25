// ToDo:
// - player animation
// - mulitplier 

extern crate sdl2;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::path::Path;
use std::collections::HashMap;
use std::time::{Instant, Duration};

use crate::top_down::{player, tile_map, camera, animation};
pub mod top_down;

fn main() -> Result<(), String> {
    // fps variables 
    let fps = 60; // the fps you want
    let frame_delay = 1000 / fps; // the time each frame should take in miliseconds 

    let mut loop_instant;
    let mut time_elapsed;

    // setting up sdl2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    
    // defining tile size 
    let tile_size: u32 = 64;

    // window size
    let screen_width: u32 = 13 * tile_size;
    let screen_height: u32 = 9 * tile_size;

    // setting up window
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

    // variables for the game
    let texture_creator = canvas.texture_creator();

    let ground_texture = texture_creator.load_texture(Path::new("res/ground.png"))?;
    let player_texture = texture_creator.load_texture(Path::new("res/player.png"))?;
    let hsau = texture_creator.load_texture(Path::new("res/Sprite-0001.png"))?;

    let tiles = vec![
                          vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                          vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                          vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
                          vec![1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1],
                          vec![1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1],
                          vec![1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
                          vec![1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
                          vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                          vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let tile_mode = HashMap::from([(0, tile_map::Tile::new(tile_map::TileHitBox::None, Rect::new(32, 0, 32, 32), &ground_texture)),
                                   (1, tile_map::Tile::new(tile_map::TileHitBox::Full, Rect::new(0, 0, 32, 32), &ground_texture))]);

    let mut camera = camera::Camera::new(camera::CameraMode::FollowPlayer, 0, 0);

    let player_texture_locations = HashMap::from([
        (0, Rect::new(0, 0, 32, 32)),
        (1, Rect::new(32, 0, 32, 32)),
        (2, Rect::new(0, 32, 32, 32)),
        (3, Rect::new(0, 0, 32, 32)),
        (4, Rect::new(32, 32, 32, 32)),
    ]);

    let mut player = player::Player::new(tile_size, 4, Rect::new(256, 256, 64, 64), player_texture_locations, &player_texture);

    let tile_map = tile_map::TileMap::new(tiles, tile_mode, 13, 9, tile_size);

    let frame_locations = vec![
        Rect::new(0, 0, 32, 32),
        Rect::new(32, 0, 32, 32),
        Rect::new(64, 0, 32, 32),
    ];

    let mut animation = animation::Animation::new(hsau, frame_locations, 3, 25);

    'mainloop: loop {
        loop_instant = Instant::now();

        for event in sdl_context.event_pump()?.poll_iter() {

            match event {
                // quiting window
                Event::Quit { .. } => break 'mainloop,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    // player movement
                    player.get_input(&tile_map, keycode, Keycode::Up, Keycode::Down, Keycode::Right, Keycode::Left);
                }
                _ => {}
            }
        }

        // moving the player
        player.move_player();

        // moving the camera 
        camera.move_camera(&player, screen_width, screen_height);

        // clearing window
        canvas.set_draw_color(Color::RGB(45, 45, 45));
        canvas.clear();

        // drawing the tilemap
        tile_map.draw(&camera, &mut canvas)?;

        // drawing the player
        player.draw(&camera, screen_width, screen_height, &mut canvas)?;

        // animation test
        animation.draw(&mut canvas, Rect::new(0, 0, tile_size, tile_size))?;
       
        // drawing to the screen
        canvas.present();

        // calculate frame delay
        time_elapsed = Instant::now() - loop_instant;
        if time_elapsed.as_millis() < frame_delay {
            std::thread::sleep( Duration::from_millis( ( frame_delay - time_elapsed.as_millis() ) as u64 ) );
        }
    }

    Ok(())
}

