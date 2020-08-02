use specs::prelude::*;
use rltk::{Point, RandomNumberGenerator};
use crate::{Map, Viewshed, Position, Monster, WantsToMelee, Confusion, systems::particle_system::ParticleBuilder, RunState, Dancing, CanDoDances, HasArgroedMonsters, WantsToMove, WantsToDance};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (WriteExpect<'a, Map>,
                       ReadExpect<'a, Point>,
                       ReadExpect<'a, Entity>,
                       ReadExpect<'a, RunState>,
                       Entities<'a>,
                       WriteStorage<'a, Viewshed>,
                       ReadStorage<'a, Position>,
                       WriteStorage<'a, Confusion>,
                       ReadStorage<'a, Monster>,
                       WriteStorage<'a, WantsToMelee>,
                       WriteExpect<'a, ParticleBuilder>,
                       ReadStorage<'a, Dancing>,
                       WriteExpect<'a, RandomNumberGenerator>,
                       ReadStorage<'a, CanDoDances>,
                       ReadStorage<'a, HasArgroedMonsters>,
                       WriteStorage<'a, WantsToMove>,
                       WriteStorage<'a, WantsToDance>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, player_entity, runstate, entities, mut viewshed, pos, mut confused, monster, mut wants_to_melee, mut particle_builder, dancers, mut rng, can_do_dances, has_agroed, mut wants_to_moves, mut want_to_dancers) = data;

        if *runstate != RunState::MonsterTurn { return; }

        for (entity, mut viewshed, pos, _monster) in (&entities, &mut viewshed, &pos, &monster).join() {
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
                        want_to_dancers.insert(entity, WantsToDance {
                            dance: dance.clone(),
                            repetitions: rng.roll_dice(1, 10) as u32
                        }).expect("Failed to insert dance request.");
                    }
                }
            }
        }
    }
}
