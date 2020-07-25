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
                    glyph = rltk::to_cp437('#');
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
