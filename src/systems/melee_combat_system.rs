use specs::prelude::*;
use rltk::{Point};
use crate::{CombatStats, Health, WantsToMelee, Name, SufferDamage, gamelog::GameLog, text::capitalize, MeleePowerBonus, DefenceBonus, Equipped, Position, systems::particle_system::ParticleBuilder, HasArgroedMonsters, Stamina, MakeNoise};

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (Entities<'a>,
                       WriteExpect<'a, GameLog>,
                       WriteStorage<'a, WantsToMelee>,
                       ReadStorage<'a, Name>,
                       ReadStorage<'a, CombatStats>,
                       ReadStorage<'a, Health>,
                       WriteStorage<'a, Stamina>,
                       WriteStorage<'a, SufferDamage>,
                       ReadStorage<'a, MeleePowerBonus>,
                       ReadStorage<'a, DefenceBonus>,
                       ReadStorage<'a, Equipped>,
                       ReadStorage<'a, Position>,
                       WriteExpect<'a, ParticleBuilder>,
                       WriteStorage<'a, HasArgroedMonsters>,
                       WriteStorage<'a, MakeNoise>);

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut gamelog, mut wants_melee, names, combat_stats, health, mut stamina, mut inflict_damage, melee_power_bonuses, defence_bonuses, equipped, positions, mut particle_builder, mut has_agroed, mut make_noises) = data;

        for (entity, wants_melee, health, mut stamina, name, stats) in (&entities, &wants_melee, &health, &mut stamina, &names, &combat_stats).join() {
            has_agroed.insert(entity, HasArgroedMonsters {}).expect("Failed to insert agro.");

            if health.health > 0 && stamina.stamina > 0 {
                let mut offensive_bonus = 0;
                for (_item_entity, power_bonus, equipped_by) in (&entities, &melee_power_bonuses, &equipped).join() {
                    if equipped_by.owner == entity {
                        offensive_bonus += power_bonus.power;
                    }
                }

                stamina.stamina -= 1;

                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                if health.health > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();

                    let mut defensive_bonus = 0;
                    for (_item_entity, defence_bonus, equipped_by) in (&entities, &defence_bonuses, &equipped).join() {
                        if equipped_by.owner == wants_melee.target {
                            defensive_bonus += defence_bonus.defence;
                        }
                    }

                    let pos = positions.get(wants_melee.target);
                    if let Some(pos) = pos {
                        particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::ORANGE), rltk::to_cp437('/'), 200.0);
                        make_noises.insert(entity, MakeNoise {
                            location: Point::new(pos.x, pos.y),
                            volume: 32,
                            faction: None,
                            surprising: false,
                            description: "fighting".to_string()
                        }).expect("Failed to insert make noise.");
                    }

                    let damage = i32::max(0, (stats.power + offensive_bonus) - (target_stats.defence + defensive_bonus));
                    if damage == 0 {
                        gamelog.on(wants_melee.target, &format!("{} {} unable to hurt {}", capitalize(&name.np), name.verb("is", "are"), target_name.np));
                    } else {
                        gamelog.on(wants_melee.target, &format!("{} {} {} ({} {})", capitalize(&name.np), name.verb("hits", "hit"), target_name.np, damage, Health::NAME));
                        SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
                    }
                }
            }
        }

        wants_melee.clear();
    }
}
