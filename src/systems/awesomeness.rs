use std::cmp::{min};
use specs::prelude::*;
use crate::{gamelog::GameLog, Awe, Awestruck, systems::particle_system::ParticleBuilder, Name, Position};

pub struct AwesomenessSystem {}

impl<'a> System<'a> for AwesomenessSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Awe>,
        WriteStorage<'a, Awestruck>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Position>
    );

    fn run(&mut self, data : Self::SystemData) {
        let (
            entities,
            mut gamelog,
            mut awe,
            mut awestruck,
            mut particle_builder,
            names,
            positions
        ) = data;

        for (entity, mut awe, awestruck) in (&entities, &mut awe, &awestruck).join() {
            if let Some(name) = names.get(entity) {
                gamelog.entries.push(format!("{} awes you with their {} for {}.", name.name, awestruck.reason, awestruck.awe));
            } else {
                gamelog.entries.push(format!("You are awed by the {} for {}.", awestruck.reason, awestruck.awe));
            }
            if let Some(pos) = positions.get(entity) {
                particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::ORANGE), rltk::to_cp437('!'), 200.0);
            }
            awe.awe = min(awe.max_awe, awe.awe + awestruck.awe);
        }

        awestruck.clear();
    }
}
