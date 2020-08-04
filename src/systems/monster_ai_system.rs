use specs::prelude::*;
use serde::{Serialize, Deserialize};
use rltk::{Point, RandomNumberGenerator};
use crate::{Map, MapPather, Viewshed, Position, Monster, MonsterAI, WantsToMelee, Confusion, systems::particle_system::ParticleBuilder, RunState, Dancing, CanDoDances, HasArgroedMonsters, WantsToMove, WantsToDance, Health, Stamina, Poise, dancing, gamelog::GameLog, text::{capitalize}, Name, Resting, Noise, systems::noise::can_hear, MonsterAINoiseRecord, Turn, InFaction};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct PathInfo {
    steps: Vec<Point>,
    step_idx: u32,
    expect_pos: Point
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum MovementGoal {
    Flee,
    SeekEnemy,
    GoDance { dance: dancing::Dance, destination: Point },
    InvestigateNoise { destination: Point, surprising: bool }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum MonsterAIState {
    WAITING,
    RESTING,
    AGGRESSIVE,
    DANCING { dance: dancing::Dance },
    MOVING { goal: MovementGoal, path: Option<PathInfo> }
}

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
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
                       WriteStorage<'a, MonsterAI>,
                       WriteStorage<'a, WantsToMelee>,
                       WriteExpect<'a, ParticleBuilder>,
                       ReadStorage<'a, Dancing>,
                       WriteExpect<'a, RandomNumberGenerator>,
                       ReadStorage<'a, CanDoDances>,
                       ReadStorage<'a, HasArgroedMonsters>,
                       WriteStorage<'a, WantsToMove>,
                       WriteStorage<'a, WantsToDance>,
                       ReadStorage<'a, Health>,
                       ReadStorage<'a, Stamina>,
                       ReadStorage<'a, Poise>,
                       ReadStorage<'a, Name>,
                       WriteStorage<'a, Resting>);

    fn run(&mut self, data: Self::SystemData) {
        let (map, player_pos, player_entity, runstate, mut gamelog, entities, viewsheds, pos, mut confused, monster, mut monster_ai, mut wants_to_melee, mut particle_builder, dancers, mut rng, can_do_dances, has_agroed, mut wants_to_moves, mut want_to_dancers, health, stamina, poise, names, mut resting) = data;

        if *runstate != RunState::MonsterTurn { return; }

        for (entity, viewshed, pos, _monster, ai, health, stamina, poise, name) in (&entities, &viewsheds, &pos, &monster, &mut monster_ai, &health, &stamina, &poise, &names).join() {
            if let Some(_) = dancers.get(entity) {
                continue;
            }

            // Handle confusion
            if let Some(_) = confused.get_mut(entity) {
                continue;
            }

            let mut new_state = ai.state.clone();

            // Finished doing things
            if health.health >= health.max_health && stamina.stamina >= stamina.max_stamina && poise.poise >= poise.max_poise {
                if let MonsterAIState::MOVING { goal, .. } = &new_state {
                    if *goal == MovementGoal::Flee {
                        new_state = MonsterAIState::WAITING;
                    }
                } else if new_state == MonsterAIState::RESTING {
                    new_state = MonsterAIState::WAITING;
                }
            }

            // Prioritize fleeing if that's in progress
            let mut chose_action = false;
            if match new_state { MonsterAIState::MOVING { goal: MovementGoal::Flee, .. } => true, _ => false } {
                chose_action = true;
            }

            // Respond to surprising noise
            if new_state != MonsterAIState::AGGRESSIVE && match ai.state { MonsterAIState::MOVING { goal: MovementGoal::InvestigateNoise { surprising: true, .. }, .. } => false, _ => true } {
                if let Some(nr) = &ai.last_heard_noise {
                    if nr.surprising {
                        chose_action = true;
                        new_state = MonsterAIState::MOVING {
                            goal: MovementGoal::InvestigateNoise {
                                destination: nr.location.clone(),
                                surprising: true
                            },
                            path: None
                        };
                    }
                }
            }

            // Respond to with presence of enemy
            let enemy_in_sight = viewshed.visible_tiles.contains(&*player_pos);
            if !chose_action && enemy_in_sight {
                ai.last_saw_enemy = Some(Point::new(player_pos.x, player_pos.y));
                if match new_state { MonsterAIState::MOVING { goal: MovementGoal::GoDance { .. }, .. } => true, _ => false } {
                    // no change
                } else if health.health < health.max_health / 10 {
                    chose_action = true;
                    particle_builder.request(pos.x, pos.y, Health::colour(), rltk::to_cp437('‼'), 200.0);
                    gamelog.on(entity, &format!("{} {} for {} life.", capitalize(&name.np), name.verb("flees", "flee"), name.pronoun_pos));
                    new_state = MonsterAIState::MOVING {
                        goal: MovementGoal::Flee,
                        path: None
                    };
                } else if stamina.stamina == 0 {
                    chose_action = true;
                    particle_builder.request(pos.x, pos.y, Stamina::colour(), rltk::to_cp437('‼'), 200.0);
                    gamelog.on(entity, &format!("{} {} to rest.", capitalize(&name.np), name.verb("flees", "flee")));
                    new_state = MonsterAIState::MOVING {
                        goal: MovementGoal::Flee,
                        path: None
                    };
                } else if poise.poise == 0 {
                    chose_action = true;
                    particle_builder.request(pos.x, pos.y, Poise::colour(), rltk::to_cp437('‼'), 200.0);
                    gamelog.on(entity, &format!("{} {} in shame.", capitalize(&name.np), name.verb("flees", "flee")));
                    new_state = MonsterAIState::MOVING {
                        goal: MovementGoal::Flee,
                        path: None
                    };
                } else {
                    if match has_agroed.get(*player_entity) { None => false, _ => true } {
                    chose_action = true;
                        new_state = MonsterAIState::AGGRESSIVE;
                    } else {
                        if let Some(can) = can_do_dances.get(entity) {
                            let range =
                                if let Some(player_vs) = viewsheds.get(*player_entity) {
                                    &player_vs.visible_tiles
                                } else {
                                    &viewshed.visible_tiles
                                };
                            for _ in 0..can.dances.len() {
                                let i = rng.range(0, can.dances.len());
                                let dance = can.dances[i].clone();
                                let start_pos = look_for_dance_spot(&Point::new(pos.x, pos.y), range, &dance, &map, &dancers, &mut rng);
                                if let Some(start_pos) = start_pos {
                                    chose_action = true;
                                    new_state = MonsterAIState::MOVING {
                                        goal: MovementGoal::GoDance {
                                            dance: dance,
                                            destination: start_pos
                                        },
                                        path: None
                                    };
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            // If not busy, see if we can look for the enemy or investigate noises
            if !chose_action && new_state == MonsterAIState::WAITING && !enemy_in_sight {
                if let Some(_) = ai.last_saw_enemy {
                    new_state = MonsterAIState::MOVING {
                        goal: MovementGoal::SeekEnemy,
                        path: None
                    };
                } else if let Some(nr) = &ai.last_heard_noise {
                    new_state = MonsterAIState::MOVING {
                        goal: MovementGoal::InvestigateNoise {
                            destination: nr.location.clone(),
                            surprising: true
                        },
                        path: None
                    };
                }
            }

            // Reset movement paths if not where expected
            if let MonsterAIState::MOVING { goal, path: Some(PathInfo { expect_pos, .. }) } = &new_state {
                if pos.x != expect_pos.x || pos.y != expect_pos.y {
                    particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::WHITE), rltk::to_cp437('?'), 200.0);
                    new_state = MonsterAIState::MOVING {
                        goal: goal.clone(),
                        path: None
                    };
                }
            }

            // Try to move
            if let MonsterAIState::MOVING { goal, path } = &new_state {
                let goal = goal.clone();
                let mut done_move = false;
                match path {
                    None => {
                        let path =
                            match goal {
                                MovementGoal::Flee => {
                                    plan_flee(&map, Point::new(pos.x, pos.y), &mut rng)
                                }
                                MovementGoal::SeekEnemy => {
                                    if let Some(dest) = ai.last_saw_enemy {
                                        path_to(&map, Point::new(pos.x, pos.y), dest)
                                    } else {
                                        None
                                    }
                                }
                                MovementGoal::GoDance { destination, .. } => {
                                    path_to(&map, Point::new(pos.x, pos.y), destination)
                                }
                                MovementGoal::InvestigateNoise { destination, .. } => {
                                    path_to(&map, Point::new(pos.x, pos.y), destination)
                                }
                            };
                        if let Some(path) = path {
                            let first_pos = path[0].clone();
                            new_state = MonsterAIState::MOVING {
                                goal: goal.clone(),
                                path: Some(PathInfo {
                                    steps: path,
                                    step_idx: 0,
                                    expect_pos: first_pos
                                })
                            };
                        } else {
                            done_move = true;
                        }
                    }
                    Some(PathInfo { steps, step_idx, .. }) => {
                        let new_step_idx = step_idx + 1;
                        if new_step_idx >= steps.len() as u32 {
                            done_move = true;
                        } else {
                            new_state = MonsterAIState::MOVING {
                                goal: goal.clone(),
                                path: Some(PathInfo {
                                    steps: steps.clone(),
                                    step_idx: new_step_idx,
                                    expect_pos: steps[new_step_idx as usize]
                                })
                            };
                        }
                    }
                }
                if done_move {
                    match goal {
                        MovementGoal::Flee => {
                            new_state = MonsterAIState::RESTING;
                        }
                        MovementGoal::SeekEnemy => {
                            new_state = MonsterAIState::WAITING;
                            ai.last_saw_enemy = None;
                        }
                        MovementGoal::GoDance { dance, .. } => {
                            new_state = MonsterAIState::DANCING {
                                dance: dance
                            };
                        }
                        MovementGoal::InvestigateNoise { .. } => {
                            new_state = MonsterAIState::WAITING
                        }
                    }
                }
            }

            // Act on new state
            ai.state = new_state;
            match &ai.state {
                MonsterAIState::WAITING => {
                    resting.insert(entity, Resting {}).expect("Failed to insert resting.");
                }
                MonsterAIState::RESTING => {
                    resting.insert(entity, Resting {}).expect("Failed to insert resting.");
                }
                MonsterAIState::AGGRESSIVE => {
                    let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
                    if distance < 1.5 {
                        wants_to_melee.insert(entity, WantsToMelee{ target: *player_entity }).expect("Unable to insert attack");
                    } else {
                        let dir = move_toward_player(&map, Point::new(pos.x, pos.y), Point::new(player_pos.x, player_pos.y));
                        if let Some(dir) = dir {
                            wants_to_moves.insert(entity, WantsToMove {
                                source: Point::new(pos.x, pos.y),
                                destination: dir
                            }).expect("Failed to insert wants move.");
                        }
                    }
                }
                MonsterAIState::DANCING { dance } => {
                    want_to_dancers.insert(entity, WantsToDance {
                        dance: dance.clone(),
                        repetitions: 1
                    }).expect("Failed to insert dance request.");
                }
                MonsterAIState::MOVING { goal: _, path: Some (PathInfo { steps, step_idx, .. }) } => {
                    wants_to_moves.insert(entity, WantsToMove {
                        source: Point::new(pos.x, pos.y),
                        destination: steps[*step_idx as usize].clone()
                    }).expect("Failed to insert wants move.");
                }
                MonsterAIState::MOVING { goal: _, path: None } => {
                    // can't act on no path
                }
            }
        }
    }
}

fn move_toward_player(map: &Map, start_pos: Point, player_pos: Point) -> Option<Point> {
    let pather = MapPather::new(map, player_pos, false);
    let path = rltk::a_star_search(
        map.xy_idx(start_pos.x, start_pos.y) as i32,
        map.xy_idx(player_pos.x, player_pos.y) as i32,
        &pather
    );
    if path.success && path.steps.len() > 1 {
        Some(Point::new(
            path.steps[1] as i32 % map.width,
            path.steps[1] as i32 / map.width
        ))
    } else {
        None
    }
}

fn plan_flee(map: &Map, start_pos: Point, rng: &mut RandomNumberGenerator) -> Option<Vec<Point>> {
    for _ in 0..50 {
        let dest = Point::new(rng.range(0, map.width), rng.range(0, map.height));
        let dest_idx = map.point_idx(&dest);
        if map.blocked[dest_idx] {
            continue
        }
        if let Some(path) = path_to(map, start_pos, dest) {
            return Some(path);
        }
    }
    None
}

fn path_to(map: &Map, start_pos: Point, dest_pos: Point) -> Option<Vec<Point>> {
    let pather = MapPather::new(map, dest_pos, false);
    let path = rltk::a_star_search(
        map.xy_idx(start_pos.x, start_pos.y) as i32,
        map.xy_idx(dest_pos.x, dest_pos.y) as i32,
        &pather
    );
    if path.success && path.steps.len() > 1 {
        let out_path = path.steps.iter().map(|step| {
            Point::new(
                *step as i32 % map.width,
                *step as i32 / map.width
            )
        }).collect();
        return Some(out_path);
    }

    None
}

fn look_for_dance_spot<'a>(current_pos: &Point, range: &Vec<Point>, dance: &dancing::Dance, map: &Map, dancers: &ReadStorage<'a, Dancing>, rng: &mut RandomNumberGenerator) -> Option<Point> {
    if is_good_start_position(current_pos, dance, map, dancers) {
        return Some(*current_pos);
    }
    for _ in 0..10 {
        let i = rng.range(0, range.len());
        let pos = range[i];
        if is_good_start_position(&pos, dance, map, dancers) {
            return Some(pos);
        }
    }
    None
}

fn is_good_start_position<'a>(start: &Point, dance: &dancing::Dance, map: &Map, dancers: &ReadStorage<'a, Dancing>) -> bool {
    let start_idx = map.point_idx(start);
    let mut at = *start;
    for dancer in dancers.join() {
        if dancer.range.contains(&at) {
            return false;
        }
    }
    for step in dance.steps().iter() {
        at = at + step.direction;
        let at_idx = map.point_idx(&at);
        if at_idx != start_idx && map.blocked[at_idx] {
            return false;
        }
        for dancer in dancers.join() {
            if dancer.range.contains(&at) {
                return false;
            }
        }
    }
    return true;
}

pub struct MonsterAINoiseTrackSystem {}

impl<'a> System<'a> for MonsterAINoiseTrackSystem {
    type SystemData = (ReadExpect<'a, Turn>,
                       ReadStorage<'a, Monster>,
                       WriteStorage<'a, MonsterAI>,
                       ReadStorage<'a, Noise>,
                       ReadStorage<'a, InFaction>,
                       ReadStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (turn, monsters, mut monster_ais, noises, factions, positions) = data;

        for (_monster, ai, faction, pos) in (&monsters, &mut monster_ais, &factions, &positions).join() {
            for (noise,) in (&noises,).join() {
                if (noise.surprising || match &ai.last_heard_noise { Some(nr) => if nr.surprising { *turn >= nr.turn + 10  } else { true }, _ => true }) && match noise.faction { Some(f) => f == faction.faction, _ => true} && can_hear(&Point::new(pos.x, pos.y), &noise) {
                    ai.last_heard_noise = Some(MonsterAINoiseRecord {
                        turn: *turn,
                        volume: noise.volume,
                        surprising: noise.surprising,
                        location: noise.location.clone()
                    });
                }
            }
        }
    }
}
