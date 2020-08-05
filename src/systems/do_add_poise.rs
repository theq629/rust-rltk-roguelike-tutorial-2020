use specs::prelude::*;
use crate::{gamelog::GameLog, text::capitalize, ItemUseInProgress, ProvidesPoise, Poise, systems::particle_system::ParticleBuilder, Name, Position};

pub struct DoAddPoiseSystem {}

impl<'a> System<'a> for DoAddPoiseSystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, ParticleBuilder>,
        Entities<'a>,
        WriteStorage<'a, ItemUseInProgress>,
        ReadStorage<'a, ProvidesPoise>,
        WriteStorage<'a, Poise>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Name>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gamelog, mut particle_builder, entities, use_in_progress, poise_providers, mut poise, positions, names) = data;

        for (entity, useitem, name) in (&entities, &use_in_progress, &names).join() {
            if let Some(poise_provider) = poise_providers.get(useitem.item) {
                for target in useitem.targets.iter() {
                    let poise = poise.get_mut(*target);
                    if let Some(poise) = poise {
                        poise.poise = i32::min(poise.max_poise, poise.poise + poise_provider.poise);
                        gamelog.on(entity, &format!("{} {} {}, gaining {} {}.", capitalize(&name.np), name.verb("drinks", "drink"), names.get(useitem.item).unwrap().np, poise_provider.poise, Poise::NAME));
                        if let Some(pos) = positions.get(*target) {
                            particle_builder.request(pos.x, pos.y, Poise::colour(), rltk::to_cp437('♥'), 200.0);
                        }
                    }
                }
            }
        }
    }
}
