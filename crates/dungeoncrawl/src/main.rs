use bracket_lib::prelude::*;

mod map;
mod screen;

use map::Map;

pub struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        State { map: Map::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let context = screen::build_screen_context()?;
    main_loop(context, State::new())
}
