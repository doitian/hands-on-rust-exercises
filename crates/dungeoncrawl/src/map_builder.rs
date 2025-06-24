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
        let mut rng = RandomNumberGenerator::new();
        self.build_random_room(&mut rng);
        self.build_corridors(&mut rng);
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

    pub fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            let point = Point::new(x, y);
            if self.map.in_bounds(point) {
                self.map.fill_tile_at(point, TileType::Floor);
            }
        }
    }

    pub fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            let point = Point::new(x, y);
            if self.map.in_bounds(point) {
                self.map.fill_tile_at(point, TileType::Floor);
            }
        }
    }

    pub fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by_key(|r| r.center().x);

        for w in rooms.windows(2) {
            let start = w[0].center();
            let end = w[1].center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(start.x, end.x, start.y);
                self.apply_vertical_tunnel(start.y, end.y, end.x);
            } else {
                self.apply_vertical_tunnel(start.y, end.y, start.x);
                self.apply_horizontal_tunnel(start.x, end.x, end.y);
            }
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
