use bracket_lib::prelude::*;

use crate::map::Map;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let mut position = self.position;
            match key {
                VirtualKeyCode::Up | VirtualKeyCode::W => {
                    position.y -= 1;
                }
                VirtualKeyCode::Down | VirtualKeyCode::S => {
                    position.y += 1;
                }
                VirtualKeyCode::Left | VirtualKeyCode::A => {
                    position.x -= 1;
                }
                VirtualKeyCode::Right | VirtualKeyCode::D => {
                    position.x += 1;
                }
                _ => {}
            }
            if map.can_enter_tile(position) {
                self.position = position;
            }
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        )
    }
}
