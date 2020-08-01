use specs::prelude::*;
use rltk::{Point, RandomNumberGenerator};
use crate::{WantsToMove, Position, Viewshed, Map};

pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Point>,
        ReadExpect<'a, Map>,
        WriteExpect<'a, RandomNumberGenerator>,
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Viewshed>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            player,
            mut player_pos,
            map,
            mut rng,
            mut wants_to_moves,
            mut positions,
            mut viewsheds
        ) = data;

        for (entity, wants_move, mut pos) in (&entities, &wants_to_moves, &mut positions).join() {
            let source_idx = map.point_idx(&wants_move.source);
            let dest =
                if map.stains[source_idx].len() > 0 && rng.roll_dice(1, 10) < 5 {
                    let rand_dest = Point::new(
                        pos.x + rng.roll_dice(1, 3) - 2,
                        pos.y + rng.roll_dice(1, 3) - 2
                    );
                    if map.point_valid(&rand_dest) {
                        rand_dest
                    } else {
                        Point::new(pos.x, pos.y)
                    }
                } else {
                    wants_move.destination
                };

            pos.x = dest.x;
            pos.y = dest.y;
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
