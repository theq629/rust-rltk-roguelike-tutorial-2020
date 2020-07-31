use specs::prelude::*;
use rltk::{Point};
use crate::{WantsToMove, Position, Viewshed};

pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Point>,
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Viewshed>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            player,
            mut player_pos,
            mut wants_to_moves,
            mut positions,
            mut viewsheds
        ) = data;

        for (entity, wants_move, mut pos) in (&entities, &wants_to_moves, &mut positions).join() {
            pos.x = wants_move.destination.x;
            pos.y = wants_move.destination.y;
            if let Some(viewshed) = viewsheds.get_mut(entity) {
                viewshed.dirty = true;
            }
            if entity == *player {
                player_pos.x = pos.x;
                player_pos.y = pos.y;
            }
        }

        wants_to_moves.clear();
    }
}
