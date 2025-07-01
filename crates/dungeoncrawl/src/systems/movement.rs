use bracket_lib::prelude::*;
use legion::{Entity, EntityStore, IntoQuery, system, systems::CommandBuffer, world::SubWorld};

use crate::{
    camera::Camera,
    components::{Enemy, Player, WantsToAttack, WantsToMove},
    map::Map,
};

fn is_player(ecs: &SubWorld, entity: &Entity) -> bool {
    ecs.entry_ref(*entity)
        .unwrap()
        .get_component::<Player>()
        .is_ok()
}

fn get_entity_at(ecs: &SubWorld, pos: &Point) -> Option<Entity> {
    <(Entity, &Point)>::query()
        .iter(ecs)
        .find_map(|(entity, entity_pos)| (entity_pos == pos).then_some(*entity))
}

#[system(for_each)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Point)]
pub fn movement(
    cmd: &mut CommandBuffer,
    ecs: &mut SubWorld,
    message_entity: &Entity,
    wants_to_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
) {
    let WantsToMove {
        entity,
        destination,
    } = wants_to_move;

    let mover_is_player = is_player(ecs, entity);
    match get_entity_at(ecs, destination) {
        Some(destination_entity) => {
            // Stop movement when the destination already has an entity. Spawn an attack if the
            // entity is on the opposite side.
            let is_opposite = mover_is_player != is_player(ecs, &destination_entity);
            if is_opposite {
                cmd.push((WantsToAttack {
                    attacker: *entity,
                    victim: destination_entity,
                },));
            }
        }
        None => {
            if map.can_enter_tile(*destination) {
                cmd.add_component(*entity, *destination);
                if mover_is_player {
                    camera.on_player_move(*destination);
                }
            }
        }
    }

    cmd.remove(*message_entity);
}
