use bracket_lib::prelude::*;

mod map;
mod map_builder;
mod player;
mod screen;

use crate::{map::Map, map_builder::MapBuilder, player::Player};

pub struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mut map_builder = MapBuilder::new();
        map_builder.build();

        State {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    let context = screen::build_screen_context()?;
    main_loop(context, State::new())
}
