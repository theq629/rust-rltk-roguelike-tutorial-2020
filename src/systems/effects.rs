use specs::prelude::*;
use serde::{Serialize, Deserialize};
use rltk::{Point};
use crate::{EffectRequest, Position, systems::particle_system::ParticleBuilder, map::Map, Awestruck};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Effect {
    AWESOMENESS {
        awe: i32,
        reason: String,
        range: i32
    }
}

pub struct EffectsSystem {}

impl<'a> System<'a> for EffectsSystem {
    type SystemData = (
        WriteExpect<'a, ParticleBuilder>,
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, EffectRequest>,
        WriteStorage<'a, Awestruck>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut particle_builder,
            map,
            positions,
            mut requests,
            mut awestruckness
        ) = data;

        for (pos, request) in (&positions, &requests).join() {
            match &request.effect {
                Effect::AWESOMENESS { awe, reason, range } => {
                    let (tiles, targets) = get_targets(Point::new(pos.x, pos.y), *range, &map);
                    for tile in tiles {
                        particle_builder.request(tile.x, tile.y, rltk::RGB::named(rltk::ORANGE), rltk::to_cp437('░'), 100.0);
                    }
                    let full_reason =
                        if let Some(effector_name) = &request.effector_name {
                            format!("{}'s {}", effector_name, reason)
                        } else {
                            reason.to_string()
                        };
                    for target in targets {
                        awestruckness.insert(target, Awestruck {
                            awe: *awe,
                            reason: full_reason.to_string()
                        }).expect("Unable to insert awestruckness.");
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
