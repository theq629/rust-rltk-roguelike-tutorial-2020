use specs::prelude::*;
use crate::{Name, InBackpack, Position, gamelog::GameLog, text::capitalize, ItemUseInProgress, ProvidesHealing, Health, InflictsDamage, SufferDamage, Map, CausesConfusion, Confusion, Equippable, Equipped, systems::particle_system::ParticleBuilder, SpreadsLiquid, MakeNoise, MakesNoise, HasAggroedMosters};

pub struct HandleItemUseSystem {}

impl<'a> System<'a> for HandleItemUseSystem {
    type SystemData = (WriteExpect<'a, Map>,
                       WriteExpect<'a, GameLog>,
                       Entities<'a>,
                       WriteStorage<'a, ItemUseInProgress>,
                       ReadStorage<'a, Name>,
                       ReadStorage<'a, ProvidesHealing>,
                       ReadStorage<'a, InflictsDamage>,
                       WriteStorage<'a, Health>,
                       WriteStorage<'a, SufferDamage>,
                       ReadStorage<'a, CausesConfusion>,
                       WriteStorage<'a, Confusion>,
                       ReadStorage<'a, Equippable>,
                       WriteStorage<'a, Equipped>,
                       WriteStorage<'a, InBackpack>,
                       ReadStorage<'a, Position>,
                       WriteExpect<'a, ParticleBuilder>,
                       ReadStorage<'a, SpreadsLiquid>,
                       WriteStorage<'a, MakeNoise>,
                       WriteStorage<'a, MakesNoise>,
                       WriteStorage<'a, HasAggroedMosters>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, mut gamelog, entities, use_in_progress, names, healing_providers, inflict_damage, mut health, mut suffer_damage, causes_confusion, mut confused, equippable, mut equipped, mut backpack, positions, mut particle_builder, liquid_spreaders, mut make_noises, makes_noises, mut has_agroed) = data;

        for (entity, useitem, name) in (&entities, &use_in_progress, &names).join() {
            let item_equippable = equippable.get(useitem.item);
            match item_equippable {
                None => {}
                Some(can_equip) => {
                    let target_slot = can_equip.slot;
                    let target = useitem.targets[0];
                    let target_name = names.get(target).unwrap();

                    let mut to_unequip: Vec<Entity> = Vec::new();
                    for (item_entity, already_equipped, name) in (&entities, &equipped, &names).join() {
                        if already_equipped.owner == target && already_equipped.slot == target_slot {
                            to_unequip.push(item_entity);
                            gamelog.on(target, &format!("{} {} {}.", capitalize(&target_name.np), target_name.verb("unequips", "unequip"), name.np));
                        }
                    }
                    for item in to_unequip.iter() {
                        equipped.remove(*item);
                        backpack.insert(*item, InBackpack{ owner: target }).expect("Unable to insert backpack entry");
                    }

                    equipped.insert(useitem.item, Equipped{ owner: target, slot: target_slot }).expect("Unable to insert equipped component");
                    backpack.remove(useitem.item);
                    gamelog.on(target, &format!("{} {} {}.", capitalize(&target_name.np), target_name.verb("equips", "equip"), names.get(useitem.item).unwrap().np));
                }
            }

            let healing_provider = healing_providers.get(useitem.item);
            match healing_provider {
                None => {}
                Some(healing_provider) => {
                    for target in useitem.targets.iter() {
                        let health = health.get_mut(*target);
                        if let Some(health) = health {
                            health.health = i32::min(health.max_health, health.health + healing_provider.heal_amount);
                            gamelog.on(entity, &format!("{} {} {}, healing {} hp.", capitalize(&name.np), name.verb("drinks", "drink"), names.get(useitem.item).unwrap().np, healing_provider.heal_amount));
                            if let Some(pos) = positions.get(*target) {
                                particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::GREEN), rltk::to_cp437('♥'), 200.0);
                            }
                        }
                    }
                }
            }

            let item_damages = inflict_damage.get(useitem.item);
            match item_damages {
                None => {}
                Some(damage) => {
                    for mob in useitem.targets.iter() {
                        SufferDamage::new_damage(&mut suffer_damage, *mob, damage.damage);
                        let mob_name = names.get(*mob).unwrap();
                        let item_name = names.get(useitem.item).unwrap();
                        gamelog.on(entity, &format!("{} {} {} on {}, inflicting {} hp.", capitalize(&name.np), name.verb("uses", "use"), item_name.np, mob_name.np, damage.damage));
                        has_agroed.insert(entity, HasAggroedMosters {}).expect("Failed to insert agro.");
                        if let Some(pos) = positions.get(*mob) {
                            particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::RED), rltk::to_cp437('‼'), 200.0);
                        }
                    }
                }
            }

            let mut add_confusion = Vec::new();
            {
                let causes_confusion = causes_confusion.get(useitem.item);
                match causes_confusion {
                    None => {}
                    Some(confusion) => {
                        for mob in useitem.targets.iter() {
                            add_confusion.push((*mob, confusion.turns));
                            let mob_name = names.get(*mob).unwrap();
                            let item_name = names.get(useitem.item).unwrap();
                            gamelog.on(entity, &format!("{} {} {} on {}, confusing {}.", capitalize(&name.np), name.verb("uses", "use"), item_name.np, mob_name.np, mob_name.pronoun));
                        }
                    }
                }
            }
            for mob in add_confusion.iter() {
                confused.insert(mob.0, Confusion{ turns: mob.1 }).expect("Unable to insert status");
            }

            if let Some(makes_noise) = makes_noises.get(useitem.item) {
                make_noises.insert(entity, MakeNoise {
                    location: useitem.targets_centre.clone(),
                    volume: makes_noise.volume,
                    faction: None,
                    surprising: makes_noise.surprising,
                    description: makes_noise.description.to_string()
                }).expect("Failed to insert make noise.");
            }

            if let Some(spreads_liquid)  = liquid_spreaders.get(useitem.item) {
                let target_tile_idxs: Vec<usize> = useitem.target_tiles.iter().map(|t| map.xy_idx(t.x, t.y)).collect();
                for tile_idx in target_tile_idxs.iter() {
                    map.stains[*tile_idx].insert(spreads_liquid.liquid);
                }
            }
        }
    }
}
