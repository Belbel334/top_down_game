use crate::player::Player;

pub struct Camera {
    x: i32,
    y: i32,
}

impl Camera {
    pub fn new ( x: i32, y: i32 ) -> Camera
    {
        Camera { x, y }
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
        // getting player location
        let player_location = player.get_location();

        // moving camera to player
        self.x = player_location.x - screen_width as i32 / 2 + player_location.width() as i32 / 2;
        self.y = player_location.y - screen_heigt as i32 / 2 + player_location.height() as i32 / 2;
    }
}
