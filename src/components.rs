use std::collections::HashSet;
use specs::prelude::*;
use specs_derive::*;
use serde::{Serialize, Deserialize};
use specs::saveload::{Marker, ConvertSaveload, SimpleMarker, SimpleMarkerAllocator};
use rltk::{RGB, Point};
use specs::error::NoError;

#[derive(Component, ConvertSaveload, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Monster {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Name {
    pub name: String
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct BlocksTile {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defence: i32,
    pub power: i32
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct WantsToMelee {
    pub target : Entity
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct SufferDamage {
    pub amount : Vec<i32>
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage { amount: vec![amount] };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Item {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct ProvidesHealing {
    pub heal_amount: i32
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct InBackpack {
    pub owner: Entity
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<rltk::Point>
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct WantsToDropItem {
    pub item: Entity
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct WantsToRemoveItem {
    pub item: Entity
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Consumable {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Ranged {
    pub range: i32
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct InflictsDamage {
    pub damage: i32
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct AreaOfEffect {
    pub radius: i32
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Confusion {
    pub turns: i32
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum EquipmentSlot { Melee, Shield }

#[derive(Component, ConvertSaveload, Clone)]
pub struct Equippable {
    pub slot : EquipmentSlot
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Equipped {
    pub owner: Entity,
    pub slot: EquipmentSlot
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct MeleePowerBonus {
    pub power: i32
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct DefenceBonus {
    pub defence: i32
}

pub struct SerializeMe;

#[derive(Component, ConvertSaveload, Clone)]
pub struct SerializationHelper {
    pub map: super::map::Map
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct ParticleLifetime {
    pub lifetime_ms: f32
}

#[derive(Component, Clone)]
pub struct MovingAutomatically {
    pub direction: Point,
    pub seen_entities: HashSet<Entity>,
    pub right_clearance: i32,
    pub left_clearance: i32
}

pub fn setup_ecs(ecs: &mut World) {
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<Player>();
    ecs.register::<Viewshed>();
    ecs.register::<Monster>();
    ecs.register::<Name>();
    ecs.register::<BlocksTile>();
    ecs.register::<CombatStats>();
    ecs.register::<WantsToMelee>();
    ecs.register::<SufferDamage>();
    ecs.register::<Item>();
    ecs.register::<ProvidesHealing>();
    ecs.register::<InBackpack>();
    ecs.register::<WantsToPickupItem>();
    ecs.register::<WantsToUseItem>();
    ecs.register::<WantsToDropItem>();
    ecs.register::<WantsToRemoveItem>();
    ecs.register::<Consumable>();
    ecs.register::<Ranged>();
    ecs.register::<InflictsDamage>();
    ecs.register::<AreaOfEffect>();
    ecs.register::<Confusion>();
    ecs.register::<Equippable>();
    ecs.register::<Equipped>();
    ecs.register::<MeleePowerBonus>();
    ecs.register::<DefenceBonus>();
    ecs.register::<SerializationHelper>();
    ecs.register::<SimpleMarker<SerializeMe>>();
    ecs.register::<ParticleLifetime>();
    ecs.register::<MovingAutomatically>();
    ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());
}
