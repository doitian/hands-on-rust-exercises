use legion::{Entity, EntityStore, system, systems::CommandBuffer, world::SubWorld};

use crate::{
    camera::Camera,
    components::{Player, WantsToMove},
    map::Map,
};

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    ecs: &mut SubWorld,
    cmd: &mut CommandBuffer,
    movement_entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
) {
    let WantsToMove {
        entity,
        destination,
    } = want_move;

    if map.can_enter_tile(*destination) {
        cmd.add_component(*entity, *destination);

        if ecs
            .entry_ref(*entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(*destination);
        }
    }

    cmd.remove(*movement_entity);
}
