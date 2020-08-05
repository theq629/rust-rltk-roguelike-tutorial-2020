use specs::prelude::*;
use crate::{gamelog::GameLog, ItemUseInProgress, Equippable, Equipped, InBackpack, Name, text::capitalize};

pub struct EquipSystem {}

impl<'a> System<'a> for EquipSystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        ReadStorage<'a, ItemUseInProgress>,
        ReadStorage<'a, Equippable>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InBackpack>,
        ReadStorage<'a, Name>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gamelog, entities, use_in_progress, equippable, mut equipped, mut backpack, names) = data;

        for (useitem,) in (&use_in_progress,).join() {
            if let Some(can_equip) = equippable.get(useitem.item) {
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
    }
}
