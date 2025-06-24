use bracket_lib::prelude::*;

mod map;
mod player;
mod screen;

use map::Map;
use player::Player;

pub struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        State {
            map: Map::new(),
            player: Player::new(Point::new(5, 5)),
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
