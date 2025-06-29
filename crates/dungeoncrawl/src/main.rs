mod camera;
mod components;
mod map;
mod map_builder;
mod screen;
mod spawner;
mod state;
mod systems;

use bracket_lib::prelude::*;

use crate::{screen::build_screen_context, state::State};

fn main() -> BError {
    let context = build_screen_context()?;
    main_loop(context, State::new())
}
