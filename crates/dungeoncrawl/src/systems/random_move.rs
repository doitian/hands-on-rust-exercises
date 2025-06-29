use bracket_lib::prelude::*;
use legion::{Query, system, world::SubWorld};

use crate::{components::MovingRandomly, map::Map};

#[system]
pub fn random_move(
    world: &mut SubWorld,
    query: &mut Query<(&mut Point, &MovingRandomly)>,
    #[state] rng: &mut RandomNumberGenerator,
    #[resource] map: &Map,
) {
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
