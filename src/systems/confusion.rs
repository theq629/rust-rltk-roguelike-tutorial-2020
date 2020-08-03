use specs::prelude::*;
use crate::{RunState, Confusion, Player, Monster, systems::particle_system::ParticleBuilder, Position};

pub struct ConfusionSystem {}

impl<'a> System<'a> for ConfusionSystem {
    type SystemData = (
        ReadExpect<'a, RunState>,
        WriteExpect<'a, ParticleBuilder>,
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Confusion>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            runstate,
            mut particle_builder,
            entities,
            players,
            monsters,
            positions,
            mut confusion,
        ) = data;

        let mut to_remove = Vec::new();

        for (entity, pos, mut confused) in (&entities, &positions, &mut confusion).join() {
            if *runstate == RunState::PlayerTurn {
                if let None = players.get(entity) {
                    continue;
                }
            } else {
                if let None = monsters.get(entity) {
                    continue;
                }
            }

            confused.turns -= 1;
            if confused.turns < 1 {
                to_remove.push(entity);
            }
            particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::MAGENTA), rltk::to_cp437('?'), 200.0);
        }

        for entity in to_remove {
            confusion.remove(entity);
        }
    }
}
