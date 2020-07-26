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
            if !map.visible_tiles[idx] {
                fg = fg.to_greyscale()
            }
            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
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
    if x < 1 || x > map.width - 2 || y < 1 || x > map.height - 2 {
        return 35;
    }

    let mut mask: u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) { mask += 1; }
    if is_revealed_and_wall(map, x, y + 1) { mask += 2; }
    if is_revealed_and_wall(map, x - 1, y) { mask += 4; }
    if is_revealed_and_wall(map, x + 1, y) { mask += 8; }

    match mask {
        0 => { 9 }
        1 => { 186 }
        2 => { 186 }
        3 => { 186 }
        4 => { 205 }
        5 => { 188 }
        6 => { 187 }
        7 => { 185 }
        8 => { 205 }
        9 => { 200 }
        10 => { 201 }
        11 => { 204 }
        12 => { 205 }
        13 => { 202 }
        14 => { 203 }
        15 => { 206 }
        _ => { 35 }
    }
}

fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map.xy_idx(x, y);
    map.tiles[idx] == TileType::Wall && map.revealed_tiles[idx]
}
