use bracket_lib::prelude::*;
use legion::{component, system};

use crate::{camera::Camera, components::Player, map::Map, state::TurnState};

#[system(for_each)]
#[filter(component::<Player>())]
pub fn player_input(
    position: &mut Point,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let transform = match key {
            VirtualKeyCode::Up | VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::S => Point::new(0, 1),
            VirtualKeyCode::Left | VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::D => Point::new(1, 0),
            _ => return,
        };
        let destination = *position + transform;
        if map.can_enter_tile(destination) {
            *position = destination;
            camera.on_player_move(destination);
            *turn_state = TurnState::PlayerTurn;
        }
    }
}
