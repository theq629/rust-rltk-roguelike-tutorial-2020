use specs::prelude::*;
use rltk::{Point};
use super::{Map, Viewshed, Position, Monster, WantsToMelee, Confusion};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (WriteExpect<'a, Map>,
                       ReadExpect<'a, Point>,
                       ReadExpect<'a, Entity>,
                       Entities<'a>,
                       WriteStorage<'a, Viewshed>,
                       WriteStorage<'a, Position>,
                       WriteStorage<'a, Confusion>,
                       ReadStorage<'a, Monster>,
                       WriteStorage<'a, WantsToMelee>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, player_entity, entities, mut viewshed, mut pos, mut confused, monster, mut wants_to_melee) = data;

        for (entity, mut viewshed, mut pos, _monster) in (&entities, &mut viewshed, &mut pos, &monster).join() {
            let mut can_act = true;

            let is_confused = confused.get_mut(entity);
            if let Some(i_am_confused) = is_confused {
                i_am_confused.turns -= 1;
                if i_am_confused.turns < 1 {
                    confused.remove(entity);
                }
                can_act = false;
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
