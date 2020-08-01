use specs::prelude::*;
use rltk::{Point};
use crate::{WantsToPickupItem, Name, InBackpack, Position, gamelog::GameLog, WantsToUseItem, ProvidesHealing, CombatStats, WantsToDropItem, Consumable, InflictsDamage, SufferDamage, Map, AreaOfEffect, Confusion, Equippable, Equipped, WantsToRemoveItem, systems::particle_system::ParticleBuilder, SpreadsLiquid};

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    type SystemData = (WriteExpect<'a, GameLog>,
                       WriteStorage<'a, WantsToPickupItem>,
                       WriteStorage<'a, Position>,
                       ReadStorage<'a, Name>,
                       WriteStorage<'a, InBackpack>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut gamelog, mut wants_pickup, mut positions, names, mut backpack) = data;

        for pickup in wants_pickup.join() {
            positions.remove(pickup.item);
            backpack.insert(pickup.item, InBackpack{ owner: pickup.collected_by }).expect("Unable to insert backpack entry");
            gamelog.on(pickup.collected_by, &format!("You pick up the {}.", names.get(pickup.item).unwrap().name));
        }

        wants_pickup.clear();
    }
}

pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    type SystemData = (ReadExpect<'a, Entity>,
                       WriteExpect<'a, Map>,
                       WriteExpect<'a, GameLog>,
                       Entities<'a>,
                       WriteStorage<'a, WantsToUseItem>,
                       ReadStorage<'a, Name>,
                       ReadStorage<'a, Consumable>,
                       ReadStorage<'a, ProvidesHealing>,
                       ReadStorage<'a, InflictsDamage>,
                       ReadStorage<'a, AreaOfEffect>,
                       WriteStorage<'a, CombatStats>,
                       WriteStorage<'a, SufferDamage>,
                       WriteStorage<'a, Confusion>,
                       ReadStorage<'a, Equippable>,
                       WriteStorage<'a, Equipped>,
                       WriteStorage<'a, InBackpack>,
                       ReadStorage<'a, Position>,
                       WriteExpect<'a, ParticleBuilder>,
                       ReadStorage<'a, SpreadsLiquid>);

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut map, mut gamelog, entities, mut wants_use, names, consumables, healing_providers, inflict_damage, aoe, mut combat_stats, mut suffer_damage, mut confused, equippable, mut equipped, mut backpack, positions, mut particle_builder, liquid_spreaders) = data;

        for (entity, useitem) in (&entities, &wants_use).join() {
            let mut target_tiles: Vec<Point> = Vec::new();
            let mut targets: Vec<Entity> = Vec::new();
            match useitem.target {
                None => { targets.push(*player_entity); }
                Some(target) => {
                    let area_effect = aoe.get(useitem.item);
                    match area_effect {
                        None => {
                            target_tiles.push(target.clone());
                            let idx = map.xy_idx(target.x, target.y);
                            for mob in map.tile_content[idx].iter() {
                                targets.push(*mob);
                            }
                        }
                        Some(area_effect) => {
                            target_tiles = rltk::field_of_view(target, area_effect.radius, &*map);
                            target_tiles.retain(|p| p.x > 0 && p.x < map.width-1 && p.y > 0 && p.y < map.height-1);
                            for tile_idx in target_tiles.iter() {
                                let idx = map.xy_idx(tile_idx.x, tile_idx.y);
                                for mob in map.tile_content[idx].iter() {
                                    targets.push(*mob);
                                }
                                particle_builder.request(tile_idx.x, tile_idx.y, rltk::RGB::named(rltk::ORANGE), rltk::to_cp437('░'), 200.0);
                            }
                        }
                    }
                }
            }

            let item_equippable = equippable.get(useitem.item);
            match item_equippable {
                None => {}
                Some(can_equip) => {
                    let target_slot = can_equip.slot;
                    let target = targets[0];

                    let mut to_unequip: Vec<Entity> = Vec::new();
                    for (item_entity, already_equipped, name) in (&entities, &equipped, &names).join() {
                        if already_equipped.owner == target && already_equipped.slot == target_slot {
                            to_unequip.push(item_entity);
                            gamelog.on(target, &format!("You unequip {}.", name.name));
                        }
                    }
                    for item in to_unequip.iter() {
                        equipped.remove(*item);
                        backpack.insert(*item, InBackpack{ owner: target }).expect("Unable to insert backpack entry");
                    }

                    equipped.insert(useitem.item, Equipped{ owner: target, slot: target_slot }).expect("Unable to insert equipped component");
                    backpack.remove(useitem.item);
                    gamelog.on(target, &format!("You equip {}.", names.get(useitem.item).unwrap().name));
                }
            }

            let healing_provider = healing_providers.get(useitem.item);
            match healing_provider {
                None => {}
                Some(healing_provider) => {
                    for target in targets.iter() {
                        let stats = combat_stats.get_mut(*target);
                        if let Some(stats) = stats {
                            stats.hp = i32::min(stats.max_hp, stats.hp + healing_provider.heal_amount);
                            gamelog.on(entity, &format!("You drink the {}, healing {} hp.", names.get(useitem.item).unwrap().name, healing_provider.heal_amount));
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
                    for mob in targets.iter() {
                        SufferDamage::new_damage(&mut suffer_damage, *mob, damage.damage);
                        let mob_name = names.get(*mob).unwrap();
                        let item_name = names.get(useitem.item).unwrap();
                        gamelog.on(entity, &format!("You use {} on {}, inflicting {} hp.", item_name.name, mob_name.name, damage.damage));
                        if let Some(pos) = positions.get(*mob) {
                            particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::RED), rltk::to_cp437('‼'), 200.0);
                        }
                    }
                }
            }

            let mut add_confusion = Vec::new();
            {
                let causes_confusion = confused.get(useitem.item);
                match causes_confusion {
                    None => {}
                    Some(confusion) => {
                        for mob in targets.iter() {
                            add_confusion.push((*mob, confusion.turns));
                            let mob_name = names.get(*mob).unwrap();
                            let item_name = names.get(useitem.item).unwrap();
                            gamelog.on(entity, &format!("You use {} on {}, confusing them.", item_name.name, mob_name.name));
                            if let Some(pos) = positions.get(*mob) {
                                particle_builder.request(pos.x, pos.y, rltk::RGB::named(rltk::MAGENTA), rltk::to_cp437('?'), 200.0);
                            }
                        }
                    }
                }
            }
            for mob in add_confusion.iter() {
                confused.insert(mob.0, Confusion{ turns: mob.1 }).expect("Unable to insert status");
            }

            if let Some(spreads_liquid)  = liquid_spreaders.get(useitem.item) {
                let target_tile_idxs: Vec<usize> = target_tiles.iter().map(|t| map.xy_idx(t.x, t.y)).collect();
                for tile_idx in target_tile_idxs.iter() {
                    map.stains[*tile_idx].insert(spreads_liquid.liquid);
                }
            }

            let consumable = consumables.get(useitem.item);
            match consumable {
                None => {}
                Some(_) => {
                    entities.delete(useitem.item).expect("Delete failed");
                }
            }
        }

        wants_use.clear();
    }
}

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
    type SystemData = (WriteExpect<'a, GameLog>,
                       Entities<'a>,
                       WriteStorage<'a, WantsToDropItem>,
                       ReadStorage<'a, Name>,
                       WriteStorage<'a, Position>,
                       WriteStorage<'a, InBackpack>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut gamelog, entities, mut wants_drop, names, mut positions, mut backpack) = data;

        for (entity, to_drop) in (&entities, &wants_drop).join() {
            let mut dropper_pos : Position = Position{x: 0, y: 0};
            {
                let dropped_pos = positions.get(entity).unwrap();
                dropper_pos.x = dropped_pos.x;
                dropper_pos.y = dropped_pos.y;
            }
            positions.insert(to_drop.item, Position{x: dropper_pos.x, y: dropper_pos.y}).expect("Unable to insert position");
            backpack.remove(to_drop.item);
            gamelog.on(entity, &format!("You drop the {}.", names.get(to_drop.item).unwrap().name));
        }

        wants_drop.clear();
    }
}

pub struct ItemRemoveSystem {}

impl<'a> System<'a> for ItemRemoveSystem {
    type SystemData = (
            Entities<'a>,
            WriteStorage<'a, WantsToRemoveItem>,
            WriteStorage<'a, Equipped>,
            WriteStorage<'a, InBackpack>
        );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_remove, mut equipped, mut backpack) = data;
        for (entity, to_remove) in (&entities, &wants_remove).join() {
            equipped.remove(to_remove.item);
            backpack.insert(to_remove.item, InBackpack{ owner: entity }).expect("Unable to insert in backpack");
        }
        wants_remove.clear();
    }
}
