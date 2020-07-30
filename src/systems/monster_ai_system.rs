use specs::prelude::*;
use rltk::{Point};
use crate::{Map, Viewshed, Position, Monster, WantsToMelee, Confusion, systems::particle_system::ParticleBuilder, RunState, Dancing};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (WriteExpect<'a, Map>,
                       ReadExpect<'a, Point>,
                       ReadExpect<'a, Entity>,
                       ReadExpect<'a, RunState>,
                       Entities<'a>,
                       WriteStorage<'a, Viewshed>,
                       WriteStorage<'a, Position>,
                       WriteStorage<'a, Confusion>,
                       ReadStorage<'a, Monster>,
                       WriteStorage<'a, WantsToMelee>,
                       WriteExpect<'a, ParticleBuilder>,
                       ReadStorage<'a, Dancing>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, player_entity, runstate, entities, mut viewshed, mut pos, mut confused, monster, mut wants_to_melee, mut particle_builder, dancers) = data;

        if *runstate != RunState::MonsterTurn { return; }

        for (entity, mut viewshed, mut pos, _monster) in (&entities, &mut viewshed, &mut pos, &monster).join() {
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

            if can_act {
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
                        pos.x = path.steps[1] as i32 % map.width;
                        pos.y = path.steps[1] as i32 / map.width;
                        viewshed.dirty = true;
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
                       Entities<'a>,
                       WriteStorage<'a, Position>,
                       WriteStorage<'a, Confusion>,
                       ReadStorage<'a, Monster>,
                       WriteExpect<'a, ParticleBuilder>,
                       WriteStorage<'a, Dancing>);

    fn run(&mut self, data: Self::SystemData) {
        let (map, runstate, entities, mut pos, mut confused, monster, mut particle_builder, mut dancers) = data;

        if *runstate != RunState::MonsterTurn { return; }

        let mut to_stop: Vec<Entity> = Vec::new();
        for (entity, mut pos, _monster, mut dancer) in (&entities, &mut pos, &monster, &mut dancers).join() {
            let dpos = dancer.dance.steps[dancer.step_idx as usize].direction;
            dancer.step_idx = (dancer.step_idx + 1) % dancer.dance.steps.len() as u32;
            if dpos.x != 0 || dpos.y != 0 {
                let new_x = pos.x + dpos.x;
                let new_y = pos.y + dpos.y;
                let new_idx = map.xy_idx(new_x, new_y);
                if map.blocked[new_idx] {
                    confused.insert(entity, Confusion{ turns: 3 }).expect("Failed to insert confusion.");
                    to_stop.push(entity);
                } else {
                    pos.x = new_x;
                    pos.y = new_y;
                    particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::MAGENTA), rltk::to_cp437('~'), 50.0);
                }
            }
        }

        for entity in to_stop {
            dancers.remove(entity);
        }
    }
}
