use specs::prelude::*;
use crate::{CombatStats, WantsToMelee, Name, SufferDamage, gamelog::{GameLog, capitalize}, MeleePowerBonus, DefenceBonus, Equipped, Position, systems::particle_system::ParticleBuilder, HasArgroedMonsters};

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (Entities<'a>,
                       WriteExpect<'a, GameLog>,
                       WriteStorage<'a, WantsToMelee>,
                       ReadStorage<'a, Name>,
                       ReadStorage<'a, CombatStats>,
                       WriteStorage<'a, SufferDamage>,
                       ReadStorage<'a, MeleePowerBonus>,
                       ReadStorage<'a, DefenceBonus>,
                       ReadStorage<'a, Equipped>,
                       ReadStorage<'a, Position>,
                       WriteExpect<'a, ParticleBuilder>,
                       WriteStorage<'a, HasArgroedMonsters>);

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut gamelog, mut wants_melee, names, combat_stats, mut inflict_damage, melee_power_bonuses, defence_bonuses, equipped, positions, mut particle_builder, mut has_agroed) = data;

        for (entity, wants_melee, name, stats) in (&entities, &wants_melee, &names, &combat_stats).join() {
            has_agroed.insert(entity, HasArgroedMonsters {}).expect("Failed to insert agro.");

            if stats.hp > 0 {
                let mut offensive_bonus = 0;
                for (_item_entity, power_bonus, equipped_by) in (&entities, &melee_power_bonuses, &equipped).join() {
                    if equipped_by.owner == entity {
                        offensive_bonus += power_bonus.power;
                    }
                }

                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                if target_stats.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();

                    let mut defensive_bonus = 0;
                    for (_item_entity, defence_bonus, equipped_by) in (&entities, &defence_bonuses, &equipped).join() {
                        if equipped_by.owner == wants_melee.target {
                            defensive_bonus += defence_bonus.defence;
                        }
                    }

                    let pos = positions.get(wants_melee.target);
                    if let Some(pos) = pos {
                        particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::ORANGE), rltk::to_cp437('‼'), 200.0);
                    }

                    let damage = i32::max(0, (stats.power + offensive_bonus) - (target_stats.defence + defensive_bonus));
                    if damage == 0 {
                        gamelog.on(wants_melee.target, &format!("{} {} unable to hurt {}", capitalize(&name.np), name.verb("is", "are"), target_name.np));
                    } else {
                        gamelog.on(wants_melee.target, &format!("{} {} {} for {} hp", capitalize(&name.np), name.verb("hits", "hit"), target_name.np, damage));
                        SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
                    }
                }
            }
        }

        wants_melee.clear();
    }
}
