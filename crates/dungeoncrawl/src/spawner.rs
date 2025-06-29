use bracket_lib::prelude::*;
use legion::{Entity, World};

use crate::components::{Enemy, Health, MovingRandomly, Name, Player, Render};

pub fn spawn_player(ecs: &mut World, pos: Point) -> Entity {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
    ))
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) -> Entity {
    let (hp, name, glyph) = match rng.range(0, 10) {
        0..7 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        MovingRandomly,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
    ))
}
