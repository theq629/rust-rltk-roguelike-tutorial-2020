use specs::prelude::World;
use super::*;

#[cfg(target_arch="wasm32")]
#[macro_use]
mod single_thread;

#[cfg(not(target_arch="wasm32"))]
#[macro_use]
mod multi_thread;

#[cfg(target_arch="wasm32")]
pub use single_thread::*;

#[cfg(not(target_arch="wasm32"))]
pub use multi_thread::*;

pub trait UnifiedDispatcher {
    fn run_now(&mut self, ecs: *mut World);
}

construct_dispatcher!(
    (VisibilitySystem, "visibility", &[]),
    (MonsterAI, "monster_ai", &[]),
    (MapIndexingSystem, "map_index", &[]),
    (AutoMovementSystem, "auto_movement", &[]),
    (MeleeCombatSystem, "melee_combat", &[]),
    (DamageSystem, "damage", &[]),
    (ItemCollectionSystem, "item_collection", &[]),
    (ItemUseSystem, "item_use", &[]),
    (ItemDropSystem, "item_drop", &[]),
    (ItemRemoveSystem, "item_remove", &[]),
    (ParticleSpawnSystem, "particle_spawn", &[])
);

pub fn new() -> Box<dyn UnifiedDispatcher + 'static> {
    new_dispatch()
}
