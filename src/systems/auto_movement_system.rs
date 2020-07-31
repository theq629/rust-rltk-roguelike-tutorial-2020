use std::collections::HashSet;
use specs::prelude::*;
use rltk::{Point};
use crate::{Map, Position, MovingAutomatically, Viewshed, RunState, WantsToMove};

pub struct AutoMovementSystem {}

impl<'a> System<'a> for AutoMovementSystem {
    type SystemData = (Entities<'a>,
                       ReadExpect<'a, Map>,
                       ReadExpect<'a, RunState>,
                       ReadStorage<'a, Position>,
                       WriteStorage<'a, MovingAutomatically>,
                       ReadStorage<'a, Viewshed>,
                       WriteStorage<'a, WantsToMove>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, map, runstate, positions, mut auto_moving, viewsheds, mut wants_to_moves) = data;

        if *runstate != RunState::MonsterTurn { return; }

        let mut to_remove: Vec<Entity> = Vec::new();
        for (entity, pos, mut auto_move, viewshed) in (&entities, &positions, &mut auto_moving, &viewsheds).join() {
            let (x, y) = (pos.x + auto_move.direction.x, pos.y + auto_move.direction.y);
            let saw_new = update_seen(&mut auto_move, &viewshed, &map);
            let clearance_grew = update_left_right(&mut auto_move, &Point::new(pos.x, pos.y), &map);
            if saw_new || clearance_grew || x < 0 || x >= map.width || y < 0 || y >= map.height || map.blocked[map.xy_idx(x, y)] {
                to_remove.push(entity);
            } else {
                wants_to_moves.insert(entity, WantsToMove {
                    destination: Point::new(x, y)
                }).expect("Failed to insert wants move.");
            }
        }

        for entity in to_remove {
            auto_moving.remove(entity);
        }
    }
}

pub fn start(ecs: &mut World, mover: Entity, direction: Point) {
    let mut auto_moving = ecs.write_storage::<MovingAutomatically>();
    let viewsheds = ecs.read_storage::<Viewshed>();
    let positions = ecs.read_storage::<Position>();
    let map = ecs.fetch::<Map>();
    let mut auto_move = MovingAutomatically {
        direction: direction,
        seen_entities: HashSet::new(),
        left_clearance: 0,
        right_clearance: 0
    };
    if let Some(pos) = positions.get(mover) {
        update_left_right(&mut auto_move, &Point::new(pos.x, pos.y), &map);
    }
    if let Some(viewshed) = viewsheds.get(mover) {
        update_seen(&mut auto_move, &viewshed, &map);
    }
    auto_moving.insert(mover, auto_move).expect("Unable to insert auto-movement");
}

pub fn stop(ecs: &mut World, mover: Entity) {
    let mut auto_moving = ecs.write_storage::<MovingAutomatically>();
    auto_moving.remove(mover);
}

pub fn is_auto_moving(ecs: &World, mover: Entity) -> bool {
    let auto_moving = ecs.read_storage::<MovingAutomatically>();
    if let Some(_) = auto_moving.get(mover) {
        return true;
    }
    false
}

fn update_seen(auto_move: &mut MovingAutomatically, viewshed: &Viewshed, map: &Map) -> bool {
    let mut got_new = false;
    for tile in &viewshed.visible_tiles {
        for entity in &map.tile_content[map.xy_idx(tile.x, tile.y)] {
            if !auto_move.seen_entities.contains(entity) {
                got_new = true;
                auto_move.seen_entities.insert(*entity);
            }
        }
    }
    got_new
}

fn update_left_right(auto_move: &mut MovingAutomatically, position: &Point, map: &Map) -> bool {
    let right_vec = Point::new(-auto_move.direction.y, auto_move.direction.x);
    let left_vec = Point::new(auto_move.direction.y, -auto_move.direction.x);
    let right = count_open_cells_to(*position, right_vec, map);
    let left = count_open_cells_to(*position, left_vec, map);
    let grew = right > auto_move.right_clearance || left > auto_move.left_clearance;
    auto_move.right_clearance = right;
    auto_move.left_clearance = left;
    grew
}

fn count_open_cells_to(from: Point, dir: Point, map: &Map) -> i32 {
    let mut at = from.clone();
    let mut count = 0;
    loop {
        if at.x < 0 || at.x >= map.width || at.y < 0 || at.y >= map.height || map.blocked[map.xy_idx(at.x, at.y)] {
            break;
        }
        count += 1;
        at.x += dir.x;
        at.y += dir.y;
    }
    count
}
