use specs::prelude::*;
use rltk::{Point, RandomNumberGenerator};
use crate::{Position, WantsToDance, Name, Dancing, gamelog::GameLog, text::capitalize, Map, RunState, Player, Monster, systems::particle_system::ParticleBuilder, EffectRequest, WantsToMove, Poise, CanDoDances};

pub struct StartDancingSystem {}

impl<'a> System<'a> for StartDancingSystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, WantsToDance>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Dancing>,
        ReadStorage<'a, Poise>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut gamelog,
            entities,
            positions,
            mut want_to_dancers,
            names,
            mut dancers,
            poise
        ) = data;

        for (entity, pos, want_dance, name) in (&entities, &positions, &want_to_dancers, &names).join() {
            if let Some(poise) = poise.get(entity) {
                if poise.poise <= 0 {
                    gamelog.on(entity, &format!("{} {} too intimidated to dance.", capitalize(&name.np), name.verb("is", "are")));
                    continue;
                }
            }

            gamelog.on(entity, &format!("{} {} the {} dance.", capitalize(&name.np), name.verb("starts", "start"), want_dance.dance.name()));
            dancers.insert(entity, Dancing {
                expect_pos: Point::new(pos.x, pos.y),
                steps: want_dance.dance.steps(),
                step_idx: 0,
                repetitions: want_dance.repetitions
            }).expect("Failed to insert dancing.");
        }

        want_to_dancers.clear();
    }
}

pub struct DancingMovementSystem {}

impl<'a> System<'a> for DancingMovementSystem {
    type SystemData = (ReadExpect<'a, Map>,
                       ReadExpect<'a, RunState>,
                       WriteExpect<'a, RandomNumberGenerator>,
                       Entities<'a>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, Player>,
                       ReadStorage<'a, Monster>,
                       WriteExpect<'a, ParticleBuilder>,
                       WriteStorage<'a, Dancing>,
                       WriteStorage<'a, EffectRequest>,
                       WriteStorage<'a, WantsToMove>,
                       ReadStorage<'a, Name>,
                       ReadStorage<'a, CanDoDances>);

    fn run(&mut self, data: Self::SystemData) {
        let (map, runstate, mut rng, entities, pos, players, monsters, mut particle_builder, mut dancers, mut effect_requests, mut wants_to_moves, names, can_do_dances) = data;

        for (entity, pos, mut dancer) in (&entities, &pos, &mut dancers).join() {
            if *runstate != RunState::PlayerTurn {
                if let None = players.get(entity) {
                    continue;
                }
            } else {
                if let None = monsters.get(entity) {
                    continue;
                }
            }

            let step = &dancer.steps[dancer.step_idx as usize];
            dancer.step_idx += 1;

            let new_x = pos.x + step.direction.x;
            let new_y = pos.y + step.direction.y;
            let new_pos = Point::new(new_x, new_y);
            let new_idx = map.xy_idx(new_x, new_y);
            dancer.expect_pos = new_pos;
            if !map.blocked[new_idx] {
                wants_to_moves.insert(entity, WantsToMove {
                    source: Point::new(pos.x, pos.y),
                    destination: new_pos
                }).expect("Failed to insert wants move.");
                particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::MAGENTA), rltk::to_cp437('~'), 50.0);
                if let Some(effect) = &step.effect {
                    let mut reason = "dancing".to_string();
                    if let Some(can_dance) = can_do_dances.get(entity) {
                        let i = rng.range(0, can_dance.descriptors.len());
                        reason = format!("{} {}", can_dance.descriptors[i], reason);
                    }
                    effect_requests.insert(entity, EffectRequest {
                        effect: effect.clone(),
                        reason: reason,
                        effector_np_pos: names.get(entity).map(|n| n.np_pos.to_string())
                    }).expect("Failed to inert effect request.");
                }
            }
        }
    }
}

pub struct DancingStatusSystem {}

impl<'a> System<'a> for DancingStatusSystem {
    type SystemData = (WriteExpect<'a, GameLog>,
                       Entities<'a>,
                       ReadStorage<'a, Position>,
                       WriteExpect<'a, ParticleBuilder>,
                       WriteStorage<'a, Dancing>,
                       ReadStorage<'a, Name>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut gamelog, entities, pos, mut particle_builder, mut dancers, names) = data;

        let mut to_stop: Vec<Entity> = Vec::new();
        for (entity, pos, mut dancer, name) in (&entities, &pos, &mut dancers, &names).join() {
            if pos.x != dancer.expect_pos.x || pos.y != dancer.expect_pos.y {
                gamelog.on(entity, &format!("{} {} {} dance.", capitalize(&name.np), name.verb("fails", "fail"), name.pronoun_pos));
                particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::MAGENTA), rltk::to_cp437('?'), 200.0);
                to_stop.push(entity);
                continue;
            }

            if dancer.step_idx >= dancer.steps.len() as u32 {
                dancer.step_idx = 0;
                dancer.repetitions -= 1;
            }
            if dancer.repetitions <= 0 {
                gamelog.on(entity, &format!("{} {} dancing.", capitalize(&name.np), name.verb("finishes", "finish")));
                to_stop.push(entity);
                continue;
            }
        }

        for entity in to_stop {
            dancers.remove(entity);
        }
    }
}
