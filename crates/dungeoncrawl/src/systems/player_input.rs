use bracket_lib::prelude::*;
use legion::{Entity, component, system, systems::CommandBuffer};

use crate::{
    components::{Player, WantsToMove},
    state::TurnState,
};

#[system(for_each)]
#[filter(component::<Player>())]
pub fn player_input(
    entity: &Entity,
    position: &mut Point,
    cmd: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
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
        cmd.push((WantsToMove {
            entity: *entity,
            destination,
        },));
        *turn_state = TurnState::PlayerTurn;
    }
}
