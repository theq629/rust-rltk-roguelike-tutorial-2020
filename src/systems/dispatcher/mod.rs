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
    build [
        with (MonsterAISystem, "monster_ai", &[])
        barrier
        with (DancingMovementSystem, "dancing_movement", &[])
        barrier
        with (MovementSystem, "movement", &[])
        with (MapIndexingSystem, "map_index", &[])
        with (AutoMovementSystem, "auto_movement", &[])
        with (MeleeCombatSystem, "melee_combat", &[])
        with (DamageSystem, "damage", &[])
        with (AwesomenessSystem, "awesomeness", &[])
        with (EffectsSystem, "effects", &[])
        with (ItemPickupSystem, "item_collection", &[])
        with (ItemDropSystem, "item_drop", &[])
        with (ItemUnequipSystem, "item_unequip", &[])
        with (ItemUseSystem, "item_use", &[])
        with (EquipSystem, "equip", &["item_use"])
        with (DoHealingSystem, "do_healing", &["item_use"])
        with (DoAddStaminaSystem, "do_add_stamina", &["item_use"])
        with (DoAddPoiseSystem, "do_add_poise", &["item_use"])
        with (DoDamageSystem, "do_damage", &["item_use"])
        with (CauseConfusionSystem, "cause_confision", &["item_use"])
        with (MakeNoiseSystem, "make_noise", &["item_use"])
        with (SpreadLiquidSystem, "spread_liquid", &["item_use"])
        with (VisibilitySystem, "visibility", &[])
        with (StartDancingSystem, "start_dancing", &[])
        with (RecoverySystem, "recovery", &[])
        barrier
        with (NoiseSystem, "noise", &[])
        with (ConfusionSystem, "confusion", &[])
        barrier
        with (DancingStatusSystem, "dancing_status", &[])
        with (PlayerListeningSystem, "player_listening", &[])
        with (MonsterAINoiseTrackSystem, "monster_ai_noise_track", &[])
        barrier
        with (ParticleSpawnSystem, "particle_spawn", &[])
        with (LogUpdaterSystem, "log_updater", &[])
        with (NoiseCleanupSystem, "noise_cleanup", &[])
        with (CleanupItemUseSystem, "item_use_cleanup", &[])
    ]
);

pub fn new() -> Box<dyn UnifiedDispatcher + 'static> {
    new_dispatch()
}
