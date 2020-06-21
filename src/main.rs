use rltk::{Rltk, GameState, RGB};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::*;
mod visibility_system;
pub use visibility_system::VisibilitySystem;

pub struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();
        draw_map(&self.ecs, ctx);
        draw_entities(&self.ecs, ctx);
    }
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

    for (pos, render) in (&positions, &renderables).join() {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible_tiles[idx] {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn setup_ecs(ecs: &mut World) {
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<Player>();
    ecs.register::<Viewshed>();
}

fn setup_world(ecs: &mut World, map : &Map) {
    let mut rng = rltk::RandomNumberGenerator::new();

    let (player_x, player_y) = map.rooms[0].centre();
    ecs.create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .build();

    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.centre();
        let glyph : rltk::FontCharType;
        match rng.roll_dice(1, 2) {
            1 => { glyph = rltk::to_cp437('g') }
            _ => { glyph = rltk::to_cp437('o') }
        }
        ecs.create_entity()
            .with(Position{ x, y })
            .with(Renderable{
                glyph: glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
            .build();
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State {
        ecs: World::new()
    };
    setup_ecs(&mut gs.ecs);
    let map = Map::new_map_room_and_corridors();
    setup_world(&mut gs.ecs, &map);
    gs.ecs.insert(map);
    rltk::main_loop(context, gs)
}
