use specs::prelude::*;
use crate::{gamelog::GameLog, text::capitalize, ItemUseInProgress, ProvidesStamina, Stamina, systems::particle_system::ParticleBuilder, Name, Position};

pub struct DoAddStaminaSystem {}

impl<'a> System<'a> for DoAddStaminaSystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, ParticleBuilder>,
        Entities<'a>,
        WriteStorage<'a, ItemUseInProgress>,
        ReadStorage<'a, ProvidesStamina>,
        WriteStorage<'a, Stamina>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Name>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gamelog, mut particle_builder, entities, use_in_progress, stamina_providers, mut stamina, positions, names) = data;

        for (entity, useitem, name) in (&entities, &use_in_progress, &names).join() {
            if let Some(stamina_provider) = stamina_providers.get(useitem.item) {
                for target in useitem.targets.iter() {
                    let stamina = stamina.get_mut(*target);
                    if let Some(stamina) = stamina {
                        stamina.stamina = i32::min(stamina.max_stamina, stamina.stamina + stamina_provider.stamina);
                        gamelog.on(entity, &format!("{} {} {}, gaining {} {}.", capitalize(&name.np), name.verb("drinks", "drink"), names.get(useitem.item).unwrap().np, stamina_provider.stamina, Stamina::NAME));
                        if let Some(pos) = positions.get(*target) {
                            particle_builder.request(pos.x, pos.y, Stamina::colour(), rltk::to_cp437('♥'), 200.0);
                        }
                    }
                }
            }
        }
    }
}

