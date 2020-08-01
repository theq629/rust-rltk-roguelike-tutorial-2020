use std::cmp::{min};
use specs::prelude::*;
use crate::{gamelog::{GameLog, capitalize}, Awe, Awestruck, systems::particle_system::ParticleBuilder, Position, Name};

pub struct AwesomenessSystem {}

impl<'a> System<'a> for AwesomenessSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Awe>,
        WriteStorage<'a, Awestruck>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Name>
    );

    fn run(&mut self, data : Self::SystemData) {
        let (
            entities,
            mut gamelog,
            mut awe,
            mut awestruck,
            mut particle_builder,
            positions,
            names
        ) = data;

        for (entity, mut awe, awestruck, name) in (&entities, &mut awe, &awestruck, &names).join() {
            gamelog.on(entity, &format!("{} {} awed by {} for {}.", capitalize(&name.np), name.verb("is", "are"), awestruck.reason, awestruck.awe));
            if let Some(pos) = positions.get(entity) {
                particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::ORANGE), rltk::to_cp437('!'), 200.0);
            }
            awe.awe = min(awe.max_awe, awe.awe + awestruck.awe);
        }

        awestruck.clear();
    }
}
