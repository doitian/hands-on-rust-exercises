use bracket_lib::prelude::*;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

const TILE_WIDTH: i32 = 32;
const TILE_HEIGHT: i32 = 32;

pub fn build_screen_context() -> BResult<BTerm> {
    BTermBuilder::new()
        .with_title("Dungeon Crawl")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", TILE_WIDTH, TILE_HEIGHT)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()
}
