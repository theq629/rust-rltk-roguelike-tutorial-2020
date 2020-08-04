use std::cmp::{max, min};
use std::collections::HashSet;
use rltk::{Point, BaseMap, Algorithm2D};
use specs::prelude::*;
use super::{Rect}; 
use serde::{Serialize, Deserialize};
use crate::{liquids::Liquid};

pub const MAPWIDTH: usize = 80;
pub const MAPHEIGHT: usize = 43;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum TileType {
    Wall, Floor, DownStairs
}

impl TileType {
    pub fn name(self) -> String {
        match self {
            TileType::Wall => "wall".to_string(),
            TileType::Floor => "floor".to_string(),
            TileType::DownStairs => "stairs down".to_string()
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub depth: i32,
    pub stains: Vec<HashSet<Liquid>>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>
}

pub struct MapPather<'a> {
    map: &'a Map,
    dest: Point,
    dest_can_block: bool
}

pub struct WallOnlyMapPather<'a> {
    map: &'a Map
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn point_idx(&self, point: &Point) -> usize {
        (point.y as usize * self.width as usize) + point.x as usize
    }

    pub fn point_valid(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    pub fn new(new_depth: i32, rng: &mut rltk::RandomNumberGenerator) -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; MAPCOUNT],
            rooms: Vec::new(),
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            revealed_tiles: vec![false; MAPCOUNT],
            visible_tiles: vec![false; MAPCOUNT],
            blocked: vec![false; MAPCOUNT],
            tile_content: vec![Vec::new(); MAPCOUNT],
            depth: new_depth,
            stains: vec![HashSet::new(); MAPCOUNT]
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.centre();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].centre();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }
                map.rooms.push(new_room)
            }
        }

        let stairs_position = map.rooms[map.rooms.len() - 1].centre();
        let stairs_idx = map.xy_idx(stairs_position.0, stairs_position.1);
        map.tiles[stairs_idx] = TileType::DownStairs;

        let stairs_position = map.rooms[0].centre();
        let stairs_idx = map.xy_idx(stairs_position.0, stairs_position.1);
        map.tiles[stairs_idx] = TileType::DownStairs;

        map
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2) ..= max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2) ..= max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl<'a> MapPather<'a> {
    pub fn new(map: &'a Map, dest: Point, dest_can_block: bool) -> Self {
        MapPather {
            map: map,
            dest: dest,
            dest_can_block: dest_can_block
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.map.width - 1 || y < 1 || y > self.map.height - 1 {
            return false;
        }
        if !self.dest_can_block && x == self.dest.x && y == self.dest.y {
            return true;
        }
        let idx = self.map.xy_idx(x, y);
        !self.map.blocked[idx]
    }
}

impl<'a> BaseMap for MapPather<'a> {
    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.map.width;
        let y = idx as i32 / self.map.width;
        let w = self.map.width as usize;

        if self.is_exit_valid(x - 1, y) { exits.push((idx-1, 1.0)) };
        if self.is_exit_valid(x + 1, y) { exits.push((idx+1, 1.0)) };
        if self.is_exit_valid(x, y - 1) { exits.push((idx-w, 1.0)) };
        if self.is_exit_valid(x, y + 1) { exits.push((idx+w, 1.0)) };

        if self.is_exit_valid(x - 1, y - 1) { exits.push(((idx-w)-1, 1.45)) };
        if self.is_exit_valid(x + 1, y - 1) { exits.push(((idx-w)+1, 1.45)) };
        if self.is_exit_valid(x - 1, y + 1) { exits.push(((idx+w)-1, 1.45)) };
        if self.is_exit_valid(x + 1, y + 1) { exits.push(((idx+w)+1, 1.45)) };

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.map.width as usize;
        let p1 = Point::new(idx1 % 2, idx1 / w);
        let p2 = Point::new(idx2 % 2, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

impl<'a> Algorithm2D for MapPather<'a> {
    fn dimensions(&self) -> Point {
        Point::new(self.map.width, self.map.height)
    }
}

impl<'a> WallOnlyMapPather<'a> {
    pub fn new(map: &'a Map) -> Self {
        WallOnlyMapPather {
            map: map
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.map.width - 1 || y < 1 || y > self.map.height - 1 {
            return false;
        }
        let idx = self.map.xy_idx(x, y);
        self.map.tiles[idx] != TileType::Wall
    }
}

impl<'a> BaseMap for WallOnlyMapPather<'a> {
    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.map.width;
        let y = idx as i32 / self.map.width;
        let w = self.map.width as usize;

        if self.is_exit_valid(x - 1, y) { exits.push((idx-1, 1.0)) };
        if self.is_exit_valid(x + 1, y) { exits.push((idx+1, 1.0)) };
        if self.is_exit_valid(x, y - 1) { exits.push((idx-w, 1.0)) };
        if self.is_exit_valid(x, y + 1) { exits.push((idx+w, 1.0)) };

        if self.is_exit_valid(x - 1, y - 1) { exits.push(((idx-w)-1, 1.45)) };
        if self.is_exit_valid(x + 1, y - 1) { exits.push(((idx-w)+1, 1.45)) };
        if self.is_exit_valid(x - 1, y + 1) { exits.push(((idx+w)-1, 1.45)) };
        if self.is_exit_valid(x + 1, y + 1) { exits.push(((idx+w)+1, 1.45)) };

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.map.width as usize;
        let p1 = Point::new(idx1 % 2, idx1 / w);
        let p2 = Point::new(idx2 % 2, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

impl<'a> Algorithm2D for WallOnlyMapPather<'a> {
    fn dimensions(&self) -> Point {
        Point::new(self.map.width, self.map.height)
    }
}
