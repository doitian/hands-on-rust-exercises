use bracket_lib::prelude::*;
use legion::{Entity, World};

use crate::components::{Player, Render};

pub fn spawn_player(ecs: &mut World, pos: Point) -> Entity {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ))
}
