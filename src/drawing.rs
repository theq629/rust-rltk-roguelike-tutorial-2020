use specs::prelude::*;
use rltk::prelude::*;
use super::{Position, Renderable};
use super::map::{Map, TileType};

pub fn draw_world(ecs: &World, ctx: &mut Rltk) {
    draw_map(ecs, ctx);
    draw_entities(ecs, ctx);
}

fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        let glyph;
        let mut fg;
        let mut bg = RGB::from_f32(0., 0., 0.);
        if map.revealed_tiles[idx] {
            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.0, 0.5, 0.5);
                },
                TileType::Wall => {
                    glyph = wall_glyph(&*map, x, y);
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
            ctx.set(x, y, fg, bg, glyph);
        }
        x += 1;
        if x >= map.width {
            x = 0;
            y += 1;
        }
    }
}

fn draw_entities(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let positions = ecs.read_storage::<Position>();
    let renderables = ecs.read_storage::<Renderable>();

    let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
    data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));

    for (pos, render) in data.iter() {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible_tiles[idx] {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
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
