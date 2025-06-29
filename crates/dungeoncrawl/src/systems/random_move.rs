use bracket_lib::prelude::*;
use legion::{Entity, Query, system, systems::CommandBuffer, world::SubWorld};

use crate::components::{MovingRandomly, WantsToMove};

#[system]
pub fn random_move(
    world: &mut SubWorld,
    query: &mut Query<(Entity, &mut Point, &MovingRandomly)>,
    cmd: &mut CommandBuffer,
    #[state] rng: &mut RandomNumberGenerator,
) {
    query.for_each_mut(world, |(entity, position, _)| {
        let direction = match rng.range(0, 4) {
            0 => Point::new(0, -1), // Up
            1 => Point::new(0, 1),  // Down
            2 => Point::new(-1, 0), // Left
            _ => Point::new(1, 0),  // Right
        };
        let destination = *position + direction;
        cmd.push((WantsToMove {
            entity: *entity,
            destination,
        },));
    });
}
