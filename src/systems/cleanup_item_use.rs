use specs::prelude::*;
use crate::{WantsToUseItem, Consumable};

pub struct CleanupItemUseSystem {}

impl<'a> System<'a> for CleanupItemUseSystem {
    type SystemData = (Entities<'a>,
                       WriteStorage<'a, WantsToUseItem>,
                       ReadStorage<'a, Consumable>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_use, consumables) = data;

        for (use_item,) in (&wants_use,).join() {
            if let Some(_) = consumables.get(use_item.item) {
                entities.delete(use_item.item).expect("Delete failed");
            }
        }

        wants_use.clear();
    }
}
