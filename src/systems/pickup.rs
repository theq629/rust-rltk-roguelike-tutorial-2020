use specs::prelude::*;
use crate::{gamelog::GameLog, WantsToPickupItem, Position, Name, InBackpack, text::capitalize};

pub struct ItemPickupSystem {}

impl<'a> System<'a> for ItemPickupSystem {
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
            let picker_name = names.get(pickup.collected_by).unwrap();
            gamelog.on(pickup.collected_by, &format!("{} {} up {}.", capitalize(&picker_name.np), picker_name.verb("picks", "pick"), names.get(pickup.item).unwrap().np));
        }

        wants_pickup.clear();
    }
}
