use std::cmp::{min, max};
use specs::prelude::*;
use rltk::prelude::*;
use super::{Position, Renderable};
use super::map::{Map, TileType};

pub fn draw_world(ecs: &World, ctx: &mut Rltk) {
    let view_centre = ecs.fetch::<Point>();

    let map = ecs.fetch::<Map>();

    let (screen_width, screen_height) = ctx.get_char_size();
    let world_min_x = max(0, view_centre.x - (screen_width / 2) as i32);
    let world_max_x = min(map.width, world_min_x + screen_width as i32);
    let world_min_y = max(0, view_centre.y - (screen_height / 2) as i32);
    let world_max_y = min(map.height, world_min_y + screen_height as i32);

    let mut screen_x = 0;
    for world_x in world_min_x..world_max_x {
        let mut screen_y = 0;
        for world_y in world_min_y..world_max_y {
            draw_cell(world_x, world_y, screen_x, screen_y, &map, ctx);
            screen_y += 1;
        }
        screen_x += 1;
    }

    let bg = RGB::from_f32(0., 0., 0.);
    let positions = ecs.read_storage::<Position>();
    let renderables = ecs.read_storage::<Renderable>();
    let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
    data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
    for (pos, render) in data.iter() {
        if pos.x >= world_min_x && pos.x < world_max_x && pos.y >= world_min_y && pos.y < world_max_y {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                let screen_x = pos.x - world_min_x;
                let screen_y = pos.y - world_min_y;
                ctx.set(screen_x, screen_y, render.fg, bg, render.glyph);
            }
        }
    }
}

fn draw_cell(world_x: i32, world_y: i32, screen_x: i32, screen_y: i32, map: &Map, ctx: &mut Rltk) {
    let idx = map.xy_idx(world_x, world_y);
    let glyph;
    let mut fg;
    let mut bg = RGB::from_f32(0., 0., 0.);
    if map.revealed_tiles[idx] {
        let tile = map.tiles[idx];
        match tile {
            TileType::Floor => {
                glyph = rltk::to_cp437('.');
                fg = RGB::from_f32(0.0, 0.5, 0.5);
            },
            TileType::Wall => {
                glyph = wall_glyph(&*map, world_x, world_y);
                fg = RGB::from_f32(0., 1.0, 0.);
            },
            TileType::DownStairs => {
                glyph = rltk::to_cp437('>');
                fg = RGB::from_f32(0., 1.0, 1.0);
            }
        }
        if map.bloodstains.contains(&idx) {
            bg = RGB::from_f32(0.3, 0., 0.);
        }
        if !map.visible_tiles[idx] {
            fg = fg.to_greyscale();
            bg = RGB::from_f32(0., 0., 0.);
        }
        ctx.set(screen_x, screen_y, fg, bg, glyph);
    }
}

fn wall_glyph(map: &Map, x: i32, y: i32) -> rltk::FontCharType {
    let mut mask: u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) { mask += 1; }
    if is_revealed_and_wall(map, x, y + 1) { mask += 2; }
    if is_revealed_and_wall(map, x - 1, y) { mask += 4; }
    if is_revealed_and_wall(map, x + 1, y) { mask += 8; }

    match mask {
        0 => {
            let mut mask: u8 = 0;

            if is_revealed_and_not_wall(map, x, y - 1) { mask += 1; }
            if is_revealed_and_not_wall(map, x, y + 1) { mask += 2; }
            if is_revealed_and_not_wall(map, x - 1, y) { mask += 4; }
            if is_revealed_and_not_wall(map, x + 1, y) { mask += 8; }

            match mask {
                1 => { 205 } // N
                2 => { 205 } // S
                3 => { 205 } // NS
                4 => { 186 } // W
                5 => { 201 } // NW
                6 => { 200 } // SW
                7 => { 9 } // NSW
                8 => { 186 } // E
                9 => { 187 } // NE
                10 => { 188 } // SE
                11 => { 9 } // NSE
                12 => { 186 } // EW
                13 => { 9 } // EWS
                14 => { 9 } // EWN
                15 => { 9 } // NSEW
                _ => { 35 }
            }
        }
        1 => { 186 } // N
        2 => { 186 } // S
        3 => { 186 } // NS
        4 => { 205 } // W
        5 => { 188 } // NW
        6 => { 187 } // SW
        7 => { 185 } // NSW
        8 => { 205 } // E
        9 => { 200 } // NE
        10 => { 201 } // SE
        11 => { 204 } // NSE
        12 => { 205 } // EW
        13 => { 202 } // EWS
        14 => { 203 } // EWN
        15 => { 206 } // NSEW
        _ => { 35 }
    }
}

fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    if x < 0 || x >= map.width || y < 0 || y >= map.height {
        return false;
    }
    let idx = map.xy_idx(x, y);
    map.tiles[idx] == TileType::Wall && map.revealed_tiles[idx]
}

fn is_revealed_and_not_wall(map: &Map, x: i32, y: i32) -> bool {
    if x < 0 || x >= map.width || y < 0 || y >= map.height {
        return false;
    }
    let idx = map.xy_idx(x, y);
    map.tiles[idx] != TileType::Wall && map.revealed_tiles[idx]
}
