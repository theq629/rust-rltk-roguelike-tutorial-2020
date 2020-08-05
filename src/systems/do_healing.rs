use specs::prelude::*;
use crate::{gamelog::GameLog, text::capitalize, ItemUseInProgress, ProvidesHealing, Health, systems::particle_system::ParticleBuilder, Name, Position};

pub struct DoHealingSystem {}

impl<'a> System<'a> for DoHealingSystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, ParticleBuilder>,
        Entities<'a>,
        WriteStorage<'a, ItemUseInProgress>,
        ReadStorage<'a, ProvidesHealing>,
        WriteStorage<'a, Health>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Name>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gamelog, mut particle_builder, entities, use_in_progress, healing_providers, mut health, positions, names) = data;

        for (entity, useitem, name) in (&entities, &use_in_progress, &names).join() {
            if let Some(healing_provider) = healing_providers.get(useitem.item) {
                for target in useitem.targets.iter() {
                    let health = health.get_mut(*target);
                    if let Some(health) = health {
                        health.health = i32::min(health.max_health, health.health + healing_provider.heal_amount);
                        gamelog.on(entity, &format!("{} {} {}, healing {} {}.", capitalize(&name.np), name.verb("drinks", "drink"), names.get(useitem.item).unwrap().np, healing_provider.heal_amount, Health::NAME));
                        if let Some(pos) = positions.get(*target) {
                            particle_builder.request(pos.x, pos.y, Health::colour(), rltk::to_cp437('♥'), 200.0);
                        }
                    }
                }
            }
        }
    }
}

