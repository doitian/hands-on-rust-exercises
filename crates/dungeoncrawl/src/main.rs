use bracket_lib::prelude::*;

mod camera;
mod map;
mod map_builder;
mod player;
mod screen;

use crate::{camera::Camera, map::Map, map_builder::MapBuilder, player::Player};

pub struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let mut map_builder = MapBuilder::new();
        map_builder.build();

        let player_start = map_builder.player_start;

        State {
            map: map_builder.map,
            player: Player::new(player_start),
            camera: Camera::new(player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.camera.on_player_move(self.player.position);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}

fn main() -> BError {
    let context = screen::build_screen_context()?;
    main_loop(context, State::new())
}
