use std::cmp::{max};
use specs::prelude::*;
use crate::{gamelog::{GameLog, capitalize}, Poise, Awestruck, systems::particle_system::ParticleBuilder, Position, Name};

pub struct AwesomenessSystem {}

impl<'a> System<'a> for AwesomenessSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Poise>,
        WriteStorage<'a, Awestruck>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Name>
    );

    fn run(&mut self, data : Self::SystemData) {
        let (
            entities,
            mut gamelog,
            mut poise,
            mut awestruck,
            mut particle_builder,
            positions,
            names
        ) = data;

        for (entity, mut poise, awestruck, name) in (&entities, &mut poise, &awestruck, &names).join() {
            gamelog.on(entity, &format!("{} {} awed by {} for {} poise.", capitalize(&name.np), name.verb("is", "are"), awestruck.reason, awestruck.poise));
            if let Some(pos) = positions.get(entity) {
                particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::ORANGE), rltk::to_cp437('!'), 200.0);
            }
            poise.poise = max(0, poise.poise - awestruck.poise);
        }

        awestruck.clear();
    }
}
