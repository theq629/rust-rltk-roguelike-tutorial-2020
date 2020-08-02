use std::collections::HashSet;
use specs::prelude::*;
use specs_derive::*;
use serde::{Serialize, Deserialize};
use specs::saveload::{Marker, ConvertSaveload, SimpleMarker, SimpleMarkerAllocator};
use rltk::{RGB, Point};
use specs::error::NoError;
use crate::{dancing::{Dance, Step}, systems::effects::Effect, liquids::Liquid};

#[derive(Component, ConvertSaveload, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
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
    pub name: String,
    pub np: String,
    pub np_pos: String,
    pub pronoun_pos: String,
    pub verb_plural: bool,
}

impl Name {
    pub fn new_regular<S: ToString>(name: S) -> Self {
        Name {
            name: name.to_string(),
            np: format!("the {}", name.to_string()),
            np_pos: format!("the {}'s", name.to_string()),
            pronoun_pos: "their".to_string(),
            verb_plural: false
        }
    }

    pub fn verb<S>(&self, singular: S, plural: S) -> S {
        if self.verb_plural {
            plural
        } else {
            singular
        }
    }
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

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct CanDoDances {
    pub dances: Vec<Dance>
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Dancing {
    pub expect_pos: Point,
    pub steps: Vec<Step>,
    pub step_idx: u32,
    pub repetitions: u32
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Poise {
    pub max_poise: i32,
    pub poise: i32
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct EffectRequest {
    pub effect: Effect,
    pub effector_np_pos: Option<String>
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Awestruck {
    pub poise: i32,
    pub reason: String
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct WantsToMove {
    pub source: Point,
    pub destination: Point
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct HasArgroedMonsters {}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SpreadsLiquid {
    pub liquid: Liquid
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct WantsToDance {
    pub dance: Dance,
    pub repetitions: u32
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
    ecs.register::<CanDoDances>();
    ecs.register::<Dancing>();
    ecs.register::<Poise>();
    ecs.register::<EffectRequest>();
    ecs.register::<Awestruck>();
    ecs.register::<HasArgroedMonsters>();
    ecs.register::<WantsToMove>();
    ecs.register::<SpreadsLiquid>();
    ecs.register::<WantsToDance>();
    ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());
}
