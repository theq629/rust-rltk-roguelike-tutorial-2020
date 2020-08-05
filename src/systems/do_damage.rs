use specs::prelude::*;
use crate::{gamelog::GameLog, systems::particle_system::ParticleBuilder, ItemUseInProgress, InflictsDamage, SufferDamage, Name, Position, HasAggroedMosters, text::capitalize, Health};

pub struct DoDamageSystem {}

impl<'a> System<'a> for DoDamageSystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, ParticleBuilder>,
        Entities<'a>,
        WriteStorage<'a, ItemUseInProgress>,
        ReadStorage<'a, InflictsDamage>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, HasAggroedMosters>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gamelog, mut particle_builder, entities, use_in_progress, inflict_damage, mut suffer_damage, names, positions, mut has_agroed) = data;

        for (entity, useitem, name) in (&entities, &use_in_progress, &names).join() {
            if let Some(damage) = inflict_damage.get(useitem.item) {
                for mob in useitem.targets.iter() {
                    SufferDamage::new_damage(&mut suffer_damage, *mob, damage.damage);
                    let mob_name = names.get(*mob).unwrap();
                    let item_name = names.get(useitem.item).unwrap();
                    gamelog.on(entity, &format!("{} {} {} on {}, inflicting {} hp.", capitalize(&name.np), name.verb("uses", "use"), item_name.np, mob_name.np, damage.damage));
                    has_agroed.insert(entity, HasAggroedMosters {}).expect("Failed to insert agro.");
                    if let Some(pos) = positions.get(*mob) {
                        particle_builder.request(pos.x, pos.y, Health::colour(), rltk::to_cp437('↑'), 200.0);
                    }
                }
            }
        }
    }
}
