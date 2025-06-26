use bracket_lib::prelude::*;
use legion::{Query, system, world::SubWorld};

use crate::{components::MovingRandomly, map::Map};

fn player_has_moved(key: &Option<VirtualKeyCode>) -> bool {
    key.is_some_and(|key| {
        matches!(
            key,
            VirtualKeyCode::Up
                | VirtualKeyCode::Left
                | VirtualKeyCode::Down
                | VirtualKeyCode::Right
                | VirtualKeyCode::W
                | VirtualKeyCode::A
                | VirtualKeyCode::S
                | VirtualKeyCode::D
        )
    })
}

#[system]
pub fn random_move(
    world: &mut SubWorld,
    query: &mut Query<(&mut Point, &MovingRandomly)>,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] rng: &mut RandomNumberGenerator,
) {
    if player_has_moved(key) {
        query.for_each_mut(world, |(position, _)| {
            let direction = match rng.range(0, 4) {
                0 => Point::new(0, -1), // Up
                1 => Point::new(0, 1),  // Down
                2 => Point::new(-1, 0), // Left
                _ => Point::new(1, 0),  // Right
            };
            let destination = *position + direction;
            if map.can_enter_tile(destination) {
                *position = destination;
            }
        });
    }
}
