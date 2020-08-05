use specs::prelude::*;
use serde::{Serialize, Deserialize};
use rltk::{Point};
use crate::{EffectRequest, Position, Awestruck, InFaction, Viewshed, Poise};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Effect {
    Awesomeness {
        poise: i32
    },
    SelfPoise {
        poise: i32
    }
}

pub struct EffectsSystem {}

impl<'a> System<'a> for EffectsSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, EffectRequest>,
        WriteStorage<'a, Awestruck>,
        ReadStorage<'a, InFaction>,
        ReadStorage<'a, Viewshed>,
        WriteStorage<'a, Poise>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            positions,
            mut requests,
            mut awestruckness,
            factions,
            viewsheds,
            mut poises
        ) = data;

        for (entity, pos, request) in (&entities, &positions, &requests).join() {
            match &request.effect {
                Effect::Awesomeness { poise } => {
                    let mut targets = Vec::new();
                    let pos_point = Point::new(pos.x, pos.y);
                    for (vs_entity, viewshed) in (&entities, &viewsheds).join() {
                        if viewshed.visible_tiles.contains(&pos_point) {
                            targets.push(vs_entity);
                        }
                    }
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
                Effect::SelfPoise { poise } => {
                    if let Some(mut self_poise) = poises.get_mut(entity) {
                        self_poise.poise = i32::min(self_poise.max_poise, self_poise.poise + poise);
                    }
                }
            }
        }

        requests.clear();
    }
}
