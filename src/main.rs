// ToDo:
// - Ui
// - draw only tiles in screen
// - slime only follows player if in screen
// - combat

extern crate sdl2;

use sdl2::event::Event;
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Scancode;

use std::path::Path;
use std::collections::HashMap;
use std::time::{Instant, Duration};

use crate::top_down::{player, tile_map, camera, animation, enemy, in_game_ui, menu};
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
    let tile_size: u32 = 32;
    let multiplier: u32 = 2;

    // window size
    let screen_width: u32 = 13 * tile_size * multiplier;
    let screen_height: u32 = 9 * tile_size * multiplier;

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

    let mut events = sdl_context.event_pump()?;

    // variables for the game
    let texture_creator = canvas.texture_creator();

    let ground_texture = texture_creator.load_texture(Path::new("res/ground.png"))?;

    let tiles = vec![
                          vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                          vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let tile_mode = HashMap::from([(0, tile_map::Tile::new(tile_map::TileHitBox::None, Rect::new(32, 0, tile_size, tile_size), &ground_texture)),
                                   (1, tile_map::Tile::new(tile_map::TileHitBox::Full, Rect::new(0, 0, tile_size, tile_size), &ground_texture))]);

    let mut camera = camera::Camera::new( 0, 0 );

    let player_idle_texture = texture_creator.load_texture(Path::new("res/player_idle.png"))?;

    let player_run_texture = texture_creator.load_texture(Path::new("res/player_run.png"))?;

    let player_idle_animations = HashMap::from([
        (1, animation::Animation::new(&player_idle_texture, 0, 0, tile_size, 4, 15)),
        (2, animation::Animation::new(&player_idle_texture, 0, 32, tile_size, 4, 15)),
        (3, animation::Animation::new(&player_idle_texture, 0, 64, tile_size, 4, 15)),
        (4, animation::Animation::new(&player_idle_texture, 0, 96, tile_size, 4, 15)),
    ]);

    let player_run_animations = HashMap::from([
        (1, animation::Animation::new(&player_run_texture, 0, 0, tile_size, 4, 5)),
        (2, animation::Animation::new(&player_run_texture, 0, 32, tile_size, 4, 5)),
        (3, animation::Animation::new(&player_run_texture, 0, 64, tile_size, 4, 5)),
        (4, animation::Animation::new(&player_run_texture, 0, 96, tile_size, 4, 5)),
    ]);

    let igui = texture_creator.load_texture(Path::new("res/IGUI.png"))?;
    let lives = in_game_ui::Lives::new(&igui, 3, Rect::new(0, 0, 32, 32), Rect::new(32, 0, 32, 32), 55, 1);

    let mut player = player::Player::new(tile_size, multiplier, 4, Rect::new(512, 512, 64, 64), player_idle_animations, player_run_animations, lives);

    let tile_map = tile_map::TileMap::new(tiles, tile_mode, 32, 32, tile_size, multiplier);

    let enemy_texture = texture_creator.load_texture(Path::new("res/enemy.png"))?;
    let mut enemies = vec![
        enemy::Enemy::new(Rect::new(1024, 1024, tile_size, tile_size), animation::Animation::new(&enemy_texture, 0, 0, tile_size, 4, 15), tile_size, multiplier, 250. ),
    ];

    let menu_texture = texture_creator.load_texture(Path::new("res/menu.png"))?;
    let mut main_menu = menu::Menu::new(tile_size, multiplier, &menu_texture, 
                                        Rect::new(0, 32, 64, 32), Rect::new(screen_width as i32/2-128, screen_height as i32/2, 256, 128),
                                        Rect::new(0, 0, 256, 32), Rect::new(screen_width as i32/2-512, 70, 1024, 128),
                                        Rect::new(64, 32, 32, 32));

    let hit_delay = 40;
    let mut frame = 0;

    let mut keyboard_state;

    let mut playing = false;

    'mainloop: loop {
        frame += 1;

        loop_instant = Instant::now();

        for event in events.poll_iter() {
            match event {
                // quiting window
                Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }
        
        if playing
        {
            if !player.check_alive()
            {
                break 'mainloop;
            }

            keyboard_state = events.keyboard_state();
            player.get_input(&tile_map, keyboard_state, Scancode::Up, Scancode::Down, Scancode::Right, Scancode::Left);

            // moving the player
            player.move_player();

            // moving the camera 
            camera.move_camera(&player, screen_width, screen_height);

            // clearing window
            canvas.set_draw_color(Color::RGB(45, 45, 45));
            canvas.clear();

            // drawing the tilemap
            tile_map.draw(&camera, &mut canvas)?;

            // drawing and moving the enemy to the player
            for enemy in &mut enemies
            {
                enemy.draw(&camera, &mut canvas)?;

                enemy.move_enemy(2);

                if !player.is_moving()
                {
                    enemy.go_to(Point::new(player.get_location().x, player.get_location().y), &tile_map, &[1]);
                }

                if frame >= hit_delay
                {
                    if player.take_damage(enemy)
                    {
                        frame = 0;
                    }
                }
            }

            // drawing the player
            player.draw(screen_width, screen_height, &mut canvas)?;
        }
        else {
            main_menu.draw(screen_width, screen_height, &mut canvas)?;
            playing = main_menu.get_input( events.mouse_state() );

            keyboard_state = events.keyboard_state();
            if keyboard_state.is_scancode_pressed(Scancode::Space) || keyboard_state.is_scancode_pressed(Scancode::Return)
            {
                playing = true;
            }
        }

        //canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        //canvas.set_draw_color(Color::RGBA(255, 0, 0, 128));
        //canvas.fill_rect(Rect::new(screen_width as i32 / 2 - 32, screen_height as i32 / 2 - 32, 64, 64))?;

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

