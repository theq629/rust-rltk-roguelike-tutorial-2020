use specs::prelude::World;
use super::*;

#[macro_use]
mod single_thread;
pub use single_thread::*;

pub trait UnifiedDispatcher {
    fn run_now(&mut self, ecs: *mut World);
}

construct_dispatcher!(
    (VisibilitySystem, "visibility", &[]),
    (MonsterAI, "monster_ai", &[]),
    (MapIndexingSystem, "map_index", &[]),
    (MeleeCombatSystem, "melee_combat", &[]),
    (DamageSystem, "damage", &[]),
    (ItemCollectionSystem, "item_collection", &[]),
    (ItemUseSystem, "item_use", &[]),
    (ItemDropSystem, "item_drop", &[]),
    (ItemRemoveSystem, "item_remove", &[])
);

pub fn new() -> Box<dyn UnifiedDispatcher + 'static> {
    new_dispatch()
}
