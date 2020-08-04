use specs::prelude::*;
use crate::{WantsToUnequipItem, Equipped, InBackpack};

pub struct ItemUnequipSystem {}

impl<'a> System<'a> for ItemUnequipSystem {
    type SystemData = (
            Entities<'a>,
            WriteStorage<'a, WantsToUnequipItem>,
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
