use specs::prelude::*;
use rltk::{Point};
use crate::{Map, Name, Position, Dancing, MonsterAI, systems::monster_ai_system::{MonsterAIState, MovementGoal}};

pub fn cell_info(cell: &Point, ecs: &World) -> Vec<String> {
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();
    let dancing = ecs.read_storage::<Dancing>();
    let monster_ai = ecs.read_storage::<MonsterAI>();
    let pos_idx = map.xy_idx(cell.x, cell.y);

    let mut items = Vec::<String>::new();

    for (entity, name, position) in (&entities, &names, &positions).join() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == cell.x && position.y == cell.y && map.visible_tiles[idx] {
            let mut name = name.name.to_string();
            if let Some(dancing) = dancing.get(entity) {
                name = format!("{} (doing the {} dance)", name, dancing.dance.name());
            }
            if let Some(MonsterAI { state, .. }) = monster_ai.get(entity) {
                match state {
                    MonsterAIState::RESTING => {
                        name = format!("{} (resting)", name);
                    }
                    MonsterAIState::AGGRESSIVE => {
                        name = format!("{} (aggressive)", name);
                    }
                    MonsterAIState::MOVING { goal: MovementGoal::Flee, .. } => {
                        name = format!("{} (fleeing)", name);
                    }
                    MonsterAIState::MOVING { .. } => {
                        name = format!("{} (moving)", name);
                    }
                    _ => {}
                }
            }
            items.push(name);
        }
    }

    if map.visible_tiles[pos_idx] {
        for liquid in map.stains[pos_idx].iter() {
            items.push(liquid.name());
        }

        items.push(map.tiles[pos_idx].name());
    }

    items
}
