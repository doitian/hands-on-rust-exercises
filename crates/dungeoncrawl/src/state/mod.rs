mod turn_state;

use bracket_lib::prelude::*;
use legion::{Resources, Schedule, World};

use crate::{
    camera::Camera,
    map_builder::MapBuilder,
    spawner::{spawn_monster, spawn_player},
    systems::{build_input_scheduler, build_monster_scheduler, build_player_scheduler},
};

pub use crate::state::turn_state::TurnState;

pub struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new();
        map_builder.build(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        map_builder.rooms.iter().skip(1).for_each(|r| {
            spawn_monster(&mut ecs, &mut rng, r.center());
        });
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(rng),
        }
    }

    pub fn execute_systems(&mut self) {
        let systems = match *(self.resources.get::<TurnState>().unwrap()) {
            TurnState::AwaitingInput => &mut self.input_systems,
            TurnState::PlayerTurn => &mut self.player_systems,
            TurnState::MonasterTurn => &mut self.monster_systems,
        };
        systems.execute(&mut self.ecs, &mut self.resources);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();

        ctx.set_active_console(0);
        self.resources.insert(ctx.key);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        self.execute_systems();
        render_draw_buffer(ctx).expect("Render error");
    }
}
