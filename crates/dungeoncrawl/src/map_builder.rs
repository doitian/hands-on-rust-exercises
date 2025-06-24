use bracket_lib::prelude::*;

use crate::{
    map::{Map, TileType},
    screen::{SCREEN_HEIGHT, SCREEN_WIDTH},
};

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new() -> Self {
        let map = Map::new();
        let player_start = Point::new(0, 0);
        MapBuilder {
            map,
            rooms: Vec::new(),
            player_start,
        }
    }

    pub fn build(&mut self) {
        self.map.fill_tiles(TileType::Wall);
        self.rooms.clear();
        self.player_start = Point::new(0, 0);
        self.build_random_room(&mut RandomNumberGenerator::new());
    }

    pub fn build_random_room(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = random_room(rng);
            if !self.is_room_overlap(&room) {
                self.add_room(room);
            }
        }

        if let Some(first_room) = self.rooms.first() {
            self.player_start = first_room.center();
        }
    }

    fn is_room_overlap(&self, room: &Rect) -> bool {
        self.rooms.iter().any(|r| r.intersect(room))
    }

    fn add_room(&mut self, room: Rect) {
        room.for_each(|p| {
            self.map.fill_tile_at(p, TileType::Floor);
        });
        self.rooms.push(room);
    }
}

fn random_room(rng: &mut RandomNumberGenerator) -> Rect {
    Rect::with_size(
        rng.range(1, SCREEN_WIDTH - 10),
        rng.range(1, SCREEN_HEIGHT - 10),
        rng.range(2, 10),
        rng.range(2, 10),
    )
}
