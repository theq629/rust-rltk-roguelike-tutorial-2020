use std::collections::HashSet;
use specs::prelude::*;
use specs_derive::*;
use serde::{Serialize, Deserialize};
use specs::saveload::{Marker, ConvertSaveload, SimpleMarker, SimpleMarkerAllocator};
use rltk::{RGB, Point};
use specs::error::NoError;
use crate::{dancing::{Dance, Step}, systems::effects::Effect, liquids::Liquid, factions::Faction, systems::monster_ai_system::MonsterAIState, Turn};

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

#[derive(Serialize, Deserialize, Clone)]
pub struct MonsterAINoiseRecord {
    pub turn: Turn,
    pub volume: u32,
    pub surprising: bool,
    pub location: Point
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct MonsterAI {
    pub state: MonsterAIState,
    pub saw_enemy_last_turn: bool,
    pub last_saw_enemy: Option<Point>,
    pub last_heard_noise: Option<MonsterAINoiseRecord>
}

impl MonsterAI {
    pub fn new() -> Self {
        MonsterAI {
            state: MonsterAIState::WAITING,
            saw_enemy_last_turn: false,
            last_saw_enemy: None,
            last_heard_noise: None
        }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Name {
    pub name: String,
    pub np: String,
    pub np_pos: String,
    pub pronoun: String,
    pub pronoun_pos: String,
    pub verb_plural: bool,
}

impl Name {
    pub fn new_regular<S: ToString>(name: S) -> Self {
        Name {
            name: name.to_string(),
            np: format!("the {}", name.to_string()),
            np_pos: format!("the {}'s", name.to_string()),
            pronoun: "them".to_string(),
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
pub struct Health {
    pub max_health: i32,
    pub health: i32
}

impl Health {
    pub const NAME: &'static str = "health";

    pub fn colour() -> RGB {
        RGB::named(rltk::RED)
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct CombatStats {
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

#[derive(Component, ConvertSaveload, Clone)]
pub struct CausesConfusion {
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
    pub map: super::map::Map,
    pub player_log: super::gamelog::PlayerLog
}

#[derive(Component, ConvertSaveload, Clone)]
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

#[derive(Component, ConvertSaveload, Clone)]
pub struct CanDoDances {
    pub dances: Vec<Dance>,
    pub descriptors: Vec<String>
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Dancing {
    pub dance: Dance,
    pub range: HashSet::<Point>,
    pub expect_pos: Point,
    pub steps: Vec<Step>,
    pub step_idx: u32,
    pub repetitions: u32
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Poise {
    pub max_poise: i32,
    pub poise: i32
}

impl Poise {
    pub const NAME: &'static str = "poise";

    pub fn colour() -> RGB {
        RGB::named(rltk::YELLOW)
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct EffectRequest {
    pub effect: Effect,
    pub reason: String,
    pub effector_np_pos: Option<String>
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Awestruck {
    pub poise: i32,
    pub reason: String
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct WantsToMove {
    pub source: Point,
    pub destination: Point
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct HasArgroedMonsters {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct SpreadsLiquid {
    pub liquid: Liquid
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct WantsToDance {
    pub dance: Dance,
    pub repetitions: u32
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct InFaction {
    pub faction: Faction
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Stamina {
    pub stamina: i32,
    pub max_stamina: i32
}

impl Stamina {
    pub const NAME: &'static str = "stamina";

    pub fn colour() -> RGB {
        RGB::from_u8(0, 0, 128)
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Resting {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct MakeNoise {
    pub location: Point,
    pub volume: u32,
    pub faction: Option<Faction>,
    pub surprising: bool,
    pub description: String
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Noise {
    pub location: Point,
    pub volume: u32,
    pub faction: Option<Faction>,
    pub surprising: bool,
    pub description: String
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct MakesNoise {
    pub volume: u32,
    pub surprising: bool,
    pub description: String
}

pub fn setup_ecs(ecs: &mut World) {
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<Player>();
    ecs.register::<Viewshed>();
    ecs.register::<Monster>();
    ecs.register::<MonsterAI>();
    ecs.register::<Name>();
    ecs.register::<BlocksTile>();
    ecs.register::<Health>();
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
    ecs.register::<CausesConfusion>();
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
    ecs.register::<InFaction>();
    ecs.register::<Stamina>();
    ecs.register::<Resting>();
    ecs.register::<MakeNoise>();
    ecs.register::<Noise>();
    ecs.register::<MakesNoise>();
    ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());
}
