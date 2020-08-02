use specs::prelude::*;
use rltk::{Point, RandomNumberGenerator};
use crate::{Map, Viewshed, Position, Monster, WantsToMelee, Confusion, systems::particle_system::ParticleBuilder, RunState, Dancing, EffectRequest, CanDoDances, HasArgroedMonsters, WantsToMove, Name, gamelog::{GameLog, capitalize}};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (WriteExpect<'a, Map>,
                       ReadExpect<'a, Point>,
                       ReadExpect<'a, Entity>,
                       ReadExpect<'a, RunState>,
                       WriteExpect<'a, GameLog>,
                       Entities<'a>,
                       WriteStorage<'a, Viewshed>,
                       ReadStorage<'a, Position>,
                       WriteStorage<'a, Confusion>,
                       ReadStorage<'a, Monster>,
                       WriteStorage<'a, WantsToMelee>,
                       WriteExpect<'a, ParticleBuilder>,
                       WriteStorage<'a, Dancing>,
                       WriteExpect<'a, RandomNumberGenerator>,
                       ReadStorage<'a, CanDoDances>,
                       ReadStorage<'a, HasArgroedMonsters>,
                       WriteStorage<'a, WantsToMove>,
                       ReadStorage<'a, Name>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, player_entity, runstate, mut gamelog, entities, mut viewshed, pos, mut confused, monster, mut wants_to_melee, mut particle_builder, mut dancers, mut rng, can_do_dances, has_agroed, mut wants_to_moves, names) = data;

        if *runstate != RunState::MonsterTurn { return; }

        for (entity, mut viewshed, pos, _monster, name) in (&entities, &mut viewshed, &pos, &monster, &names).join() {
            if let Some(_) = dancers.get(entity) {
                continue;
            }

            let mut can_act = true;

            let is_confused = confused.get_mut(entity);
            if let Some(i_am_confused) = is_confused {
                i_am_confused.turns -= 1;
                if i_am_confused.turns < 1 {
                    confused.remove(entity);
                }
                can_act = false;
                particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::MAGENTA), rltk::to_cp437('?'), 200.0);
            }

            let mut acted = false;
            if can_act && match has_agroed.get(*player_entity) { None => false, _ => true } {
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                if distance < 1.5 {
                    wants_to_melee.insert(entity, WantsToMelee{ target: *player_entity }).expect("Unable to insert attack");
                } else if viewshed.visible_tiles.contains(&*player_pos) {
                    let path = rltk::a_star_search(
                        map.xy_idx(pos.x, pos.y) as i32,
                        map.xy_idx(player_pos.x, player_pos.y) as i32,
                        &mut *map
                    );
                    if path.success && path.steps.len() > 1 {
                        wants_to_moves.insert(entity, WantsToMove {
                            source: Point::new(pos.x, pos.y),
                            destination: Point::new(
                                path.steps[1] as i32 % map.width,
                                path.steps[1] as i32 / map.width
                            )
                        }).expect("Failed to insert wants move.");
                        viewshed.dirty = true;
                        acted = true;
                    }
                }
            }

            if !acted {
                if let Some(can) = can_do_dances.get(entity) {
                    if rng.roll_dice(1, 10) < 5 {
                        let i = rng.range(0, can.dances.len());
                        let dance = &can.dances[i];
                        gamelog.on(entity, &format!("{} {} the {} dance.", capitalize(&name.np), name.verb("starts", "start"), dance.name()));
                        dancers.insert(entity, Dancing {
                            expect_pos: Point::new(pos.x, pos.y),
                            steps: dance.steps(),
                            step_idx: 0
                        }).expect("Failed to insert dancing.");
                    }
                }
            }
        }
    }
}

pub struct DancingMonsterAI {}

impl<'a> System<'a> for DancingMonsterAI {
    type SystemData = (ReadExpect<'a, Map>,
                       ReadExpect<'a, RunState>,
                       WriteExpect<'a, GameLog>,
                       Entities<'a>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, Monster>,
                       WriteExpect<'a, ParticleBuilder>,
                       WriteStorage<'a, Dancing>,
                       WriteStorage<'a, EffectRequest>,
                       WriteExpect<'a, RandomNumberGenerator>,
                       WriteStorage<'a, WantsToMove>,
                       ReadStorage<'a, Name>);

    fn run(&mut self, data: Self::SystemData) {
        let (map, runstate, mut gamelog, entities, pos, monster, mut particle_builder, mut dancers, mut effect_requests, mut rng, mut wants_to_moves, names) = data;

        if *runstate != RunState::MonsterTurn { return; }

        let mut to_stop: Vec<Entity> = Vec::new();
        for (entity, pos, _monster, mut dancer, name) in (&entities, &pos, &monster, &mut dancers, &names).join() {
            if pos.x != dancer.expect_pos.x || pos.y != dancer.expect_pos.y {
                gamelog.on(entity, &format!("{} {} {} dance.", capitalize(&name.np), name.verb("fails", "fail"), name.pronoun_pos));
                particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::MAGENTA), rltk::to_cp437('?'), 200.0);
                to_stop.push(entity);
            }
            let step = &dancer.steps[dancer.step_idx as usize];
            dancer.step_idx += 1;
            if dancer.step_idx >= dancer.steps.len() as u32 {
                if rng.roll_dice(1, 10) < 5 {
                    gamelog.on(entity, &format!("{} {} dancing.", capitalize(&name.np), name.verb("stops", "stop")));
                    to_stop.push(entity);
                } else {
                    dancer.step_idx = 0;
                }
            }
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
                    effect_requests.insert(entity, EffectRequest {
                        effect: effect.clone(),
                        effector_np_pos: names.get(entity).map(|n| n.np_pos.to_string())
                    }).expect("Failed to inert effect request.");
                }
            }
        }

        for entity in to_stop {
            dancers.remove(entity);
        }
    }
}
