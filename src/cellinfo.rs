use specs::prelude::*;
use rltk::{Point};
use crate::{Map, Name, Position, Dancing, MonsterAI, systems::monster_ai_system::{MonsterAIState, MovementGoal}, Confusion};

pub fn cell_info(cell: &Point, ecs: &World) -> Vec<String> {
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();
    let dancing = ecs.read_storage::<Dancing>();
    let monster_ai = ecs.read_storage::<MonsterAI>();
    let confusion = ecs.read_storage::<Confusion>();
    let pos_idx = map.xy_idx(cell.x, cell.y);

    let mut items = Vec::<String>::new();

    for (entity, name, position) in (&entities, &names, &positions).join() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == cell.x && position.y == cell.y && map.visible_tiles[idx] {
            let mut name = name.name.to_string();
            if let Some(dancing) = dancing.get(entity) {
                name = format!("{} (doing the {} dance)", name, dancing.dance.name());
            }
            let mut ai_info = Vec::new();
            if let Some(MonsterAI { state, .. }) = monster_ai.get(entity) {
                match state {
                    MonsterAIState::RESTING => {
                        ai_info.push("resting".to_string());
                    }
                    MonsterAIState::AGGRESSIVE => {
                        ai_info.push("aggressive".to_string());
                    }
                    MonsterAIState::MOVING { goal: MovementGoal::Flee, .. } => {
                        ai_info.push("fleeing".to_string());
                    }
                    MonsterAIState::MOVING { .. } => {
                        ai_info.push("moving".to_string());
                    }
                    _ => {}
                }
            }
            if let Some(_) = confusion.get(entity) {
                ai_info.push("confused".to_string());
            }
            if ai_info.len() > 0 {
                name = format!("{} ({})", name, ai_info.join(", "));
            }
            #[cfg(debug_assertions)]
            if let Some(MonsterAI { state, .. }) = monster_ai.get(entity) {
                match state {
                    MonsterAIState::WAITING => {
                        name = format!("{} [W]", name);
                    }
                    MonsterAIState::RESTING => {
                        name = format!("{} [R]", name);
                    }
                    MonsterAIState::AGGRESSIVE => {
                        name = format!("{} [A]", name);
                    }
                    MonsterAIState::DANCING { .. } => {
                        name = format!("{} [D]", name);
                    }
                    MonsterAIState::MOVING { goal: MovementGoal::Flee, .. } => {
                        name = format!("{} [MF]", name);
                    }
                    MonsterAIState::MOVING { goal: MovementGoal::SeekEnemy, .. } => {
                        name = format!("{} [MSe]", name);
                    }
                    MonsterAIState::MOVING { goal: MovementGoal::GoDance { .. }, .. } => {
                        name = format!("{} [MGd]", name);
                    }
                    MonsterAIState::MOVING { goal: MovementGoal::InvestigateNoise { .. }, .. } => {
                        name = format!("{} [MIn]", name);
                    }
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
