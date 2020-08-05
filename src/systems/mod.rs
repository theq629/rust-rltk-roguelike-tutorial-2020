mod dispatcher;
pub use dispatcher::UnifiedDispatcher;

mod visibility_system;
pub use visibility_system::VisibilitySystem;
pub mod monster_ai_system;
pub use monster_ai_system::{MonsterAISystem, MonsterAINoiseTrackSystem};
pub mod auto_movement_system;
pub use auto_movement_system::AutoMovementSystem;
mod map_indexing_system;
pub use map_indexing_system::MapIndexingSystem;
mod melee_combat_system;
pub use melee_combat_system::MeleeCombatSystem;
pub mod damage_system;
pub use damage_system::DamageSystem;
mod pickup;
use pickup::ItemPickupSystem;
mod drop;
use drop::ItemDropSystem;
mod unequip;
use unequip::ItemUnequipSystem;
mod use_item;
pub use use_item::ItemUseSystem;
mod equip;
pub use equip::EquipSystem;
mod do_healing;
pub use do_healing::DoHealingSystem;
mod do_damage;
pub use do_damage::DoDamageSystem;
mod cause_confusion;
pub use cause_confusion::CauseConfusionSystem;
mod make_noise;
pub use make_noise::MakeNoiseSystem;
mod spread_liquid;
pub use spread_liquid::SpreadLiquidSystem;
mod cleanup_item_use;
pub use cleanup_item_use::CleanupItemUseSystem;
pub mod particle_system;
pub use particle_system::{ParticleSpawnSystem};
pub mod effects;
pub use effects::{EffectsSystem};
mod awesomeness;
pub use awesomeness::AwesomenessSystem;
mod movement;
pub use movement::MovementSystem;
mod log_updater;
pub use log_updater::LogUpdaterSystem;
pub mod dancing;
pub use dancing::{StartDancingSystem, DancingMovementSystem, DancingStatusSystem};
mod recovery;
pub use recovery::{RecoverySystem};
pub mod noise;
pub use noise::{NoiseSystem, NoiseCleanupSystem, PlayerListeningSystem};
mod confusion;
pub use confusion::{ConfusionSystem};

pub fn build() -> Box<dyn UnifiedDispatcher + 'static> {
    dispatcher::new()
}
