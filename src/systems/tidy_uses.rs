use specs::prelude::*;
use rltk::{Point};
use crate::{WantsToPickupItem, Name, InBackpack, Position, gamelog::GameLog, text::capitalize, WantsToUseItem, ProvidesHealing, Health, WantsToDropItem, Consumable, InflictsDamage, SufferDamage, Map, AreaOfEffect, CausesConfusion, Confusion, Equippable, Equipped, WantsToRemoveItem, systems::particle_system::ParticleBuilder, SpreadsLiquid, MakeNoise, MakesNoise, Monster, Player, HasAggroedMosters};

pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    type SystemData = (ReadExpect<'a, Entity>,
                       WriteExpect<'a, Map>,
                       WriteExpect<'a, GameLog>,
                       ReadExpect<'a, Point>,
                       Entities<'a>,
                       WriteStorage<'a, WantsToUseItem>,
                       ReadStorage<'a, Name>,
                       ReadStorage<'a, Consumable>,
                       ReadStorage<'a, ProvidesHealing>,
                       ReadStorage<'a, InflictsDamage>,
                       ReadStorage<'a, AreaOfEffect>,
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
                       ReadStorage<'a, Monster>,
                       ReadStorage<'a, Player>,
                       WriteStorage<'a, HasAggroedMosters>);

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut map, mut gamelog, player_pos, entities, mut wants_use, names, consumables, healing_providers, inflict_damage, aoe, mut health, mut suffer_damage, causes_confusion, mut confused, equippable, mut equipped, mut backpack, positions, mut particle_builder, liquid_spreaders, mut make_noises, makes_noises, monsters, players, mut has_agroed) = data;

        for (entity, useitem, name) in (&entities, &wants_use, &names).join() {
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

        for (entity, to_drop, name) in (&entities, &wants_drop, &names).join() {
            let mut dropper_pos : Position = Position{x: 0, y: 0};
            {
                let dropped_pos = positions.get(entity).unwrap();
                dropper_pos.x = dropped_pos.x;
                dropper_pos.y = dropped_pos.y;
            }
            positions.insert(to_drop.item, Position{x: dropper_pos.x, y: dropper_pos.y}).expect("Unable to insert position");
            backpack.remove(to_drop.item);
            gamelog.on(entity, &format!("{} {} {}.", capitalize(&name.np), name.verb("drops", "drop"), names.get(to_drop.item).unwrap().np));
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
