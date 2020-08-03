use specs::prelude::*;
use rltk::{Point, RandomNumberGenerator};
use crate::{WantsToMove, Position, Viewshed, Map, gamelog::GameLog, text::capitalize, Name, MakeNoise, factions::Faction, Confusion};

pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Point>,
        ReadExpect<'a, Map>,
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, RandomNumberGenerator>,
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, MakeNoise>,
        ReadStorage<'a, Confusion>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            player,
            mut player_pos,
            map,
            mut gamelog,
            mut rng,
            mut wants_to_moves,
            mut positions,
            mut viewsheds,
            names,
            mut make_noises,
            confusion
        ) = data;

        for (entity, wants_move, mut pos, name) in (&entities, &wants_to_moves, &mut positions, &names).join() {
            let source_idx = map.point_idx(&wants_move.source);
            let stains = &map.stains[source_idx];
            let (dest, did_slip) =
                if stains.len() > 0 && rng.roll_dice(1, 10) < 5 {
                    let slip_on_i = rng.range(0, stains.len());
                    let mut slip_on = None;
                    for (i, stain) in stains.iter().enumerate() {
                        if i == slip_on_i {
                            slip_on = Some(stain);
                            break;
                        }
                    }
                    if let Some(slip_on) = slip_on {
                        gamelog.on(entity, &format!("{} {} on the {}.", capitalize(&name.np), name.verb("slips", "slip"), slip_on.name()));
                    } else {
                        gamelog.on(entity, &format!("{} {}.", capitalize(&name.np), name.verb("slips", "slip")));
                    }
                    let rand_dest = Point::new(
                        pos.x + rng.roll_dice(1, 3) - 2,
                        pos.y + rng.roll_dice(1, 3) - 2
                    );
                    let rand_dest_idx = map.point_idx(&rand_dest);
                    if map.point_valid(&rand_dest) && !map.blocked[rand_dest_idx] {
                        (rand_dest, true)
                    } else {
                        (Point::new(pos.x, pos.y), true)
                    }
                } else {
                    (wants_move.destination, false)
                };

            let is_confused = match confusion.get(entity) { Some(_) => true, _ => false };

            if did_slip {
                make_noises.insert(entity, MakeNoise {
                    location: Point::new(pos.x, pos.y),
                    volume: 32,
                    faction: Some(Faction::PLAYER),
                    surprising: false,
                    description: "something slipping".to_string()
                }).expect("Failed to insert make noise.");
            } else if !is_confused {
                if entity == *player { // don't need to bother with regular movement noises for monsters
                    make_noises.insert(entity, MakeNoise {
                        location: Point::new(pos.x, pos.y),
                        volume: 16,
                        faction: Some(Faction::PLAYER),
                        surprising: false,
                        description: "movement".to_string()
                    }).expect("Failed to insert make noise.");
                }
            }

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
