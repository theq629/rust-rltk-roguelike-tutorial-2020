use specs::prelude::*;
use serde::{Serialize, Deserialize};
use rltk::{Point};
use crate::{EffectRequest, Position, map::Map, Awestruck, InFaction};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Effect {
    AWESOMENESS {
        poise: i32,
        range: i32
    }
}

pub struct EffectsSystem {}

impl<'a> System<'a> for EffectsSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, EffectRequest>,
        WriteStorage<'a, Awestruck>,
        ReadStorage<'a, InFaction>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            map,
            positions,
            mut requests,
            mut awestruckness,
            factions
        ) = data;

        for (entity, pos, request) in (&entities, &positions, &requests).join() {
            match &request.effect {
                Effect::AWESOMENESS { poise, range } => {
                    let (_, targets) = get_targets(Point::new(pos.x, pos.y), *range, &map);
                    let full_reason =
                        if let Some(effector_np_pos) = &request.effector_np_pos {
                            format!("{} {}", effector_np_pos, request.reason)
                        } else {
                            request.reason.to_string()
                        };
                    let entity_faction = factions.get(entity);
                    for target in targets {
                        let are_enemies =
                            match (entity_faction, factions.get(target)) {
                                (Some(f1), Some(f2)) => f1.faction != f2.faction,
                                (_, _) => true
                            };
                        if are_enemies {
                            awestruckness.insert(target, Awestruck {
                                poise: *poise,
                                reason: full_reason.to_string()
                            }).expect("Unable to insert awestruckness.");
                        }
                    }
                }
            }
        }

        requests.clear();
    }
}

fn get_targets(centre: Point, radius: i32, map: &Map) -> (Vec<Point>, Vec<Entity>) {
    let mut blast_tiles = rltk::field_of_view(centre, radius, map);
    blast_tiles.retain(|p| p.x > 0 && p.x < map.width-1 && p.y > 0 && p.y < map.height-1);

    let mut targets: Vec<Entity> = Vec::new();
    for tile_idx in blast_tiles.iter() {
        let idx = map.xy_idx(tile_idx.x, tile_idx.y);
        for mob in map.tile_content[idx].iter() {
            targets.push(*mob);
        }
    }

    (blast_tiles, targets)
}
