mod dispatcher;
pub use dispatcher::UnifiedDispatcher;

mod visibility_system;
pub use visibility_system::VisibilitySystem;
mod monster_ai_system;
pub use monster_ai_system::MonsterAI;
pub mod auto_movement_system;
pub use auto_movement_system::AutoMovementSystem;
mod map_indexing_system;
pub use map_indexing_system::MapIndexingSystem;
mod melee_combat_system;
pub use melee_combat_system::MeleeCombatSystem;
pub mod damage_system;
pub use damage_system::DamageSystem;
mod inventory_system;
pub use inventory_system::{ItemCollectionSystem, ItemUseSystem, ItemDropSystem, ItemRemoveSystem};
pub mod particle_system;
pub use particle_system::{ParticleSpawnSystem};

pub fn build() -> Box<dyn UnifiedDispatcher + 'static> {
    dispatcher::new()
}
