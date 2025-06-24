use bracket_lib::prelude::*;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

pub fn build_screen_context() -> BResult<BTerm> {
    BTermBuilder::simple80x50()
        .with_title("Dungeon Crawl")
        .build()
}
