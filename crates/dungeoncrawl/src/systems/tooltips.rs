use bracket_lib::prelude::*;
use legion::{
    Entity, EntityStore, Query, system,
    world::{ComponentError, SubWorld},
};

use crate::{
    camera::Camera,
    components::{Health, Name},
    screen::HUD_SCALE,
};

#[system]
#[read_component(Health)]
pub fn tooltips(
    ecs: &SubWorld,
    query: &mut Query<(Entity, &Point, &Name)>,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera,
) {
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    for (entity, _, name) in query.iter(ecs).filter(|(_, pos, _)| **pos == map_pos) {
        let screen_pos = *mouse_pos * HUD_SCALE;
        let display = match ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
            Ok(Health { current, .. }) => format!("{} : {} hp", name.0, current),
            Err(ComponentError::NotFound { .. }) => name.0.clone(),
            Err(err) => {
                log(format!(
                    "Failed to get Health component in tooltips_system: {}",
                    err
                ));
                name.0.clone()
            }
        };
        draw_batch.print(screen_pos, &display);
    }

    draw_batch.submit(10100).expect("Batch error");
}
