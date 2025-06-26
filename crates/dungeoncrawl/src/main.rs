use bracket_lib::prelude::*;
use legion::{Resources, Schedule, World};

mod camera;
mod components;
mod map;
mod map_builder;
mod screen;
mod spawner;
mod systems;

use crate::{
    camera::Camera, map_builder::MapBuilder, spawner::spawn_player, systems::build_scheduler,
};

pub struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut map_builder = MapBuilder::new();
        map_builder.build();
        spawn_player(&mut ecs, map_builder.player_start);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = screen::build_screen_context()?;
    main_loop(context, State::new())
}
