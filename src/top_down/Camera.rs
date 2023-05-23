use crate::top_down::Player::Player;

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
