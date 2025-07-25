use crate::screen::{SCREEN_HEIGHT, SCREEN_WIDTH};
use bracket_lib::prelude::*;

const NUM_TILES: usize = SCREEN_WIDTH as usize * (SCREEN_HEIGHT as usize);

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y as usize * SCREEN_WIDTH as usize) + x as usize
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        self.in_bounds(point).then(|| map_idx(point.x, point.y))
    }

    pub fn fill_tile_at(&mut self, point: Point, tile: TileType) -> bool {
        if let Some(idx) = self.try_idx(point) {
            self.tiles[idx] = tile;
            true
        } else {
            false
        }
    }

    pub fn fill_tiles(&mut self, tile: TileType) {
        self.tiles.iter_mut().for_each(|t| {
            *t = tile;
        });
    }
}
