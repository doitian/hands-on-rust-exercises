use bracket_lib::prelude::*;
use legion::system;

use crate::{
    camera::Camera,
    map::{Map, TileType},
};

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &mut Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let position = Point::new(x, y);

            if let Some(idx) = map.try_idx(position) {
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                draw_batch.set(
                    camera.world_to_view(position),
                    ColorPair::new(WHITE, BLACK),
                    glyph,
                );
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}
