use bracket_lib::prelude::*;
use legion::{Query, system, world::SubWorld};

use crate::{camera::Camera, components::Render};

#[system]
pub fn entity_render(
    world: &SubWorld,
    query: &mut Query<(&Point, &Render)>,
    #[resource] camera: &mut Camera,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    for (position, render) in query.iter(world) {
        if camera.is_in_view(*position) {
            draw_batch.set(camera.world_to_view(*position), render.color, render.glyph);
        }
    }

    draw_batch.submit(1).expect("Batch error");
}
