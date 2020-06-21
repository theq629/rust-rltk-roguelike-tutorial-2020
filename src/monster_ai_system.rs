use specs::prelude::*;
use super::{Map, Viewshed, Position, Monster, Name};
use rltk::{console, Point};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (WriteExpect<'a, Map>,
                       ReadExpect<'a, Point>,
                       WriteStorage<'a, Viewshed>,
                       WriteStorage<'a, Position>,
                       ReadStorage<'a, Monster>,
                       ReadStorage<'a, Name>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, mut viewshed, mut pos, monster, name) = data;

        for (mut viewshed, mut pos, _monster, name) in (&mut viewshed, &mut pos, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} shouts insults", name.name));
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
