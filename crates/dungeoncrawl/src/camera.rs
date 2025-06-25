use bracket_lib::prelude::*;

use crate::screen::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        let left_x = player_position.x - DISPLAY_WIDTH / 2;
        let top_y = player_position.y - DISPLAY_HEIGHT / 2;
        Self {
            left_x,
            right_x: left_x + DISPLAY_WIDTH,
            top_y,
            bottom_y: top_y + DISPLAY_HEIGHT,
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        let new_camera = Self::new(player_position);
        if *self != new_camera {
            println!("new camera: {:?}", new_camera);
        }
        *self = new_camera;
    }
}
