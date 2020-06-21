use specs::prelude::*;
use super::{Viewshed, Position, Monster, Name};
use rltk::{console, Point};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (ReadExpect<'a, Point>,
                       ReadStorage<'a, Viewshed>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, Monster>,
                       ReadStorage<'a, Name>);

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, viewshed, pos, monster, name) = data;

        for (viewshed, _pos, _monster, name) in (&viewshed, &pos, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} shouts insults", name.name));
            }
        }
    }
}
