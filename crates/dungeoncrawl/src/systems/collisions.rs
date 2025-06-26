use bracket_lib::prelude::*;
use legion::{
    Entity, IntoQuery, component, maybe_changed, system, systems::CommandBuffer, world::SubWorld,
};

use crate::components::{Enemy, Player};

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(world: &SubWorld, cmd: &mut CommandBuffer) {
    let mut moved_players =
        <&Point>::query().filter(component::<Player>() & maybe_changed::<Point>());
    moved_players.for_each(world, |player_position| {
        <(Entity, &Point)>::query()
            .filter(component::<Enemy>())
            .for_each(world, |(entity, enemy_position)| {
                if player_position == enemy_position {
                    cmd.remove(*entity);
                }
            })
    });

    let mut moved_enemies =
        <(Entity, &Point)>::query().filter(component::<Enemy>() & maybe_changed::<Point>());
    moved_enemies.for_each(world, |(entity, enemy_position)| {
        <&Point>::query()
            .filter(component::<Player>())
            .for_each(world, |player_position| {
                if player_position == enemy_position {
                    cmd.remove(*entity);
                }
            })
    });
}
