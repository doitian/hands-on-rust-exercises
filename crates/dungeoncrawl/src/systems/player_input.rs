use bracket_lib::prelude::*;
use legion::{Entity, system, systems::CommandBuffer};

use crate::{components::WantsToMove, state::TurnState};

#[system(for_each)]
pub fn player_input(
    cmd: &mut CommandBuffer,
    player_entity: &Entity,
    player_position: &Point,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let key = match key {
        Some(key) => key,
        None => return,
    };
    let transform = match key {
        VirtualKeyCode::Up | VirtualKeyCode::W => Point::new(0, -1),
        VirtualKeyCode::Down | VirtualKeyCode::S => Point::new(0, 1),
        VirtualKeyCode::Left | VirtualKeyCode::A => Point::new(-1, 0),
        VirtualKeyCode::Right | VirtualKeyCode::D => Point::new(1, 0),
        _ => return,
    };

    let destination = *player_position + transform;
    cmd.push((WantsToMove {
        entity: *player_entity,
        destination,
    },));
    *turn_state = TurnState::PlayerTurn;
}
