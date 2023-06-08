use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;
use sdl2::rect::Rect;

struct Lives<'a>
{
    texture: &'a Texture<'a>,
    live_heart: Rect,
    dead_heart: Rect,
}
