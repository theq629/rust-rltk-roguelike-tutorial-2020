use specs::prelude::*;
use specs::saveload::{SimpleMarker, MarkedBuilder};
use rltk::{RGB};
use super::{SerializeMe, CombatStats, Health, Player, Renderable, Name, Position, Viewshed, Monster, MonsterAI, BlocksTile, Item, ProvidesHealing, Consumable, Ranged, InflictsDamage, AreaOfEffect, CausesConfusion, EquipmentSlot, Equippable, MeleePowerBonus, DefenceBonus, CanDoDances, dancing::Dance, Poise, liquids::Liquid, SpreadsLiquid, InFaction, factions::Faction, Stamina, MakesNoise};

#[derive(Clone, PartialEq)]
pub enum Stuff {
    Rabbit,
    BigRabbit,
    Thrall,
    ToughThrall,
    Vampire,
    OldVampire,
    WaterDart,
    WaterBalloon,
    BloodDart,
    BloodBalloon,
    OilDart,
    OilBalloon,
    HealthKit,
    SuperHealthKit,
    Grenade,
    StunDart,
    LongStunDart,
    Dart,
    Firecracker,
    Flashbang,
    Knife,
    Sword,
    ElectroSword,
    SuperSword,
    LightArmour,
    MediumArmour,
    HeavyArmour,
    Shield,
}

impl Stuff {
    pub fn spawn(&self, ecs: &mut World, x: i32, y: i32) {
        match self {
            Stuff::Rabbit => rabbit(ecs, x, y),
            Stuff::BigRabbit => big_rabbit(ecs, x, y),
            Stuff::Vampire => vampire(ecs, x, y),
            Stuff::Thrall => thrall(ecs, x, y),
            Stuff::ToughThrall => tough_thrall(ecs, x, y),
            Stuff::OldVampire => old_vampire(ecs, x, y),
            Stuff::WaterDart => water_dart(ecs, x, y),
            Stuff::WaterBalloon => water_balloon(ecs, x, y),
            Stuff::BloodDart => blood_dart(ecs, x, y),
            Stuff::BloodBalloon => blood_balloon(ecs, x, y),
            Stuff::OilDart => oil_dart(ecs, x, y),
            Stuff::OilBalloon => oil_balloon(ecs, x, y),
            Stuff::HealthKit => health_kit(ecs, x, y),
            Stuff::SuperHealthKit => super_health_kit(ecs, x, y),
            Stuff::Grenade => grenade(ecs, x, y),
            Stuff::StunDart => stun_dart(ecs, x, y),
            Stuff::LongStunDart => long_stun_dart(ecs, x, y),
            Stuff::Dart => dart(ecs, x, y),
            Stuff::Firecracker => firecracker(ecs, x, y),
            Stuff::Flashbang => flashbang(ecs, x, y),
            Stuff::Knife => knife(ecs, x, y),
            Stuff::Sword => sword(ecs, x, y),
            Stuff::ElectroSword => electro_sword(ecs, x, y),
            Stuff::SuperSword => super_sword(ecs, x, y),
            Stuff::LightArmour => light_armour(ecs, x, y),
            Stuff::MediumArmour => medium_armour(ecs, x, y),
            Stuff::HeavyArmour => heavy_armour(ecs, x, y),
            Stuff::Shield => shield(ecs, x, y),
        }
    }
}

pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            render_order: 1
        })
        .with(Player{})
        .with(BlocksTile{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(Name{
            name: "player".to_string(),
            np: "you".to_string(),
            np_pos: "your".to_string(),
            pronoun: "yourself".to_string(),
            pronoun_pos: "your".to_string(),
            verb_plural: true
        })
        .with(InFaction{ faction: Faction::PLAYER })
        .with(CombatStats{ defence: 2, power: 5 })
        .with(Health{ max_health: 30, health: 30 })
        .with(Stamina{ max_stamina: 10, stamina: 10 })
        .with(Poise{ max_poise: 10, poise: 10 })
        .with(CanDoDances{
            dances: vec![Dance::HOP, Dance::JITTER, Dance::CIRCLE],
            descriptors: vec!["cool", "awesome", "impressive", "elegant"].iter().map(|s| s.to_string()).collect()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

pub fn rabbit(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Name::new_regular("rabbit"))
        .with(Renderable{
            glyph: rltk::to_cp437('r'),
            fg: RGB::named(rltk::WHITE),
            render_order: 2
        })
        .with(Monster{})
        .with(MonsterAI::new())
        .with(BlocksTile{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(InFaction{ faction: Faction::ENEMIES })
        .with(CombatStats{ defence: 1, power: 1 })
        .with(Health{ max_health: 2, health: 2 })
        .with(Stamina{ max_stamina: 30, stamina: 30 })
        .with(Poise{ max_poise: 5, poise: 5 })
        .with(CanDoDances{
            dances: vec![Dance::HOP],
            descriptors: vec!["cute", "nimble", "furry", "hoppy"].iter().map(|s| s.to_string()).collect()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn big_rabbit(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Name::new_regular("big rabbit"))
        .with(Renderable{
            glyph: rltk::to_cp437('R'),
            fg: RGB::named(rltk::WHITE),
            render_order: 2
        })
        .with(Monster{})
        .with(MonsterAI::new())
        .with(BlocksTile{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(InFaction{ faction: Faction::ENEMIES })
        .with(CombatStats{ defence: 4, power: 4 })
        .with(Health{ max_health: 3, health: 3 })
        .with(Stamina{ max_stamina: 40, stamina: 40 })
        .with(Poise{ max_poise: 7, poise: 7 })
        .with(CanDoDances{
            dances: vec![Dance::HOP],
            descriptors: vec!["shaggy", "hoppy", "solid", "jumpy", "kicky"].iter().map(|s| s.to_string()).collect()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn thrall(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Name::new_regular("thrall"))
        .with(Renderable{
            glyph: rltk::to_cp437('h'),
            fg: RGB::named(rltk::WHITE),
            render_order: 2
        })
        .with(Monster{})
        .with(MonsterAI::new())
        .with(BlocksTile{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(InFaction{ faction: Faction::ENEMIES })
        .with(CombatStats{ defence: 10, power: 5 })
        .with(Health{ max_health: 50, health: 50 })
        .with(Stamina{ max_stamina: 50, stamina: 50 })
        .with(Poise{ max_poise: 10, poise: 10 })
        .with(CanDoDances{
            dances: vec![Dance::JITTER],
            descriptors: vec!["jittery", "clunky", "heavy", "solid"].iter().map(|s| s.to_string()).collect()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn tough_thrall(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Name::new_regular("tough thrall"))
        .with(Renderable{
            glyph: rltk::to_cp437('H'),
            fg: RGB::named(rltk::WHITE),
            render_order: 2
        })
        .with(Monster{})
        .with(MonsterAI::new())
        .with(BlocksTile{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(InFaction{ faction: Faction::ENEMIES })
        .with(CombatStats{ defence: 15, power: 10 })
        .with(Health{ max_health: 50, health: 50 })
        .with(Stamina{ max_stamina: 70, stamina: 70 })
        .with(Poise{ max_poise: 13, poise: 13 })
        .with(CanDoDances{
            dances: vec![Dance::JITTER],
            descriptors: vec!["jittery", "clunky", "heavy", "solid"].iter().map(|s| s.to_string()).collect()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn vampire(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Name::new_regular("vampire"))
        .with(Renderable{
            glyph: rltk::to_cp437('v'),
            fg: RGB::named(rltk::WHITE),
            render_order: 2
        })
        .with(Monster{})
        .with(MonsterAI::new())
        .with(BlocksTile{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(InFaction{ faction: Faction::ENEMIES })
        .with(CombatStats{ defence: 15, power: 30 })
        .with(Health{ max_health: 20, health: 20 })
        .with(Stamina{ max_stamina: 100, stamina: 100 })
        .with(Poise{ max_poise: 20, poise: 20 })
        .with(CanDoDances{
            dances: vec![Dance::CIRCLE],
            descriptors: vec!["cool", "awesome", "creepy", "scary", "bloodthirsty", "elegant"].iter().map(|s| s.to_string()).collect()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn old_vampire(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Name::new_regular("old vampire"))
        .with(Renderable{
            glyph: rltk::to_cp437('V'),
            fg: RGB::named(rltk::WHITE),
            render_order: 2
        })
        .with(Monster{})
        .with(MonsterAI::new())
        .with(BlocksTile{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(InFaction{ faction: Faction::ENEMIES })
        .with(CombatStats{ defence: 20, power: 35 })
        .with(Health{ max_health: 20, health: 20 })
        .with(Stamina{ max_stamina: 150, stamina: 150 })
        .with(Poise{ max_poise: 30, poise: 30 })
        .with(CanDoDances{
            dances: vec![Dance::CIRCLE],
            descriptors: vec!["cool", "awesome", "creepy", "scary", "bloodthirsty", "elegant"].iter().map(|s| s.to_string()).collect()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn water_dart(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('↑'),
            fg: RGB::named(rltk::BLUE),
            render_order: 3
        })
        .with(Name::new_regular("water dart"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(SpreadsLiquid{ liquid: Liquid::WATER })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn water_balloon(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('↑'),
            fg: RGB::named(rltk::BLUE),
            render_order: 3
        })
        .with(Name::new_regular("water balloon"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(SpreadsLiquid{ liquid: Liquid::WATER })
        .with(AreaOfEffect{ radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn oil_dart(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('↑'),
            fg: RGB::named(rltk::GREY),
            render_order: 3
        })
        .with(Name::new_regular("oil dart"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(SpreadsLiquid{ liquid: Liquid::OIL })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn oil_balloon(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('↑'),
            fg: RGB::named(rltk::GREY),
            render_order: 3
        })
        .with(Name::new_regular("oil balloon"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(SpreadsLiquid{ liquid: Liquid::OIL })
        .with(AreaOfEffect{ radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn blood_dart(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('↑'),
            fg: RGB::named(rltk::RED),
            render_order: 3
        })
        .with(Name::new_regular("blood dart"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(SpreadsLiquid{ liquid: Liquid::BLOOD })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn blood_balloon(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('↑'),
            fg: RGB::named(rltk::RED),
            render_order: 3
        })
        .with(Name::new_regular("blood balloon"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(SpreadsLiquid{ liquid: Liquid::BLOOD })
        .with(AreaOfEffect{ radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn health_kit(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('!'),
            fg: RGB::named(rltk::MAGENTA),
            render_order: 3
        })
        .with(Name::new_regular("health kit"))
        .with(Item{})
        .with(Consumable{})
        .with(ProvidesHealing{ heal_amount: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn super_health_kit(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('!'),
            fg: RGB::named(rltk::RED),
            render_order: 3
        })
        .with(Name::new_regular("super health kit"))
        .with(Item{})
        .with(Consumable{})
        .with(ProvidesHealing{ heal_amount: 16 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn dart(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('↑'),
            fg: RGB::named(rltk::CYAN),
            render_order: 3
        })
        .with(Name::new_regular("dart"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 8 })
        .with(MakesNoise{
            volume: 15,
            surprising: true,
            description: "explosion".to_string()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn grenade(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('↑'),
            fg: RGB::named(rltk::ORANGE),
            render_order: 3
        })
        .with(Name::new_regular("grenade"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 8 })
        .with(MakesNoise{
            volume: 20,
            surprising: true,
            description: "fire".to_string()
        })
        .with(AreaOfEffect{ radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn firecracker(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('?'),
            fg: RGB::named(rltk::RED),
            render_order: 3
        })
        .with(Name::new_regular("firecracker"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 15 })
        .with(CausesConfusion{ turns: 4 })
        .with(MakesNoise{
            volume: 20,
            surprising: true,
            description: "a bang".to_string()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn flashbang(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('?'),
            fg: RGB::named(rltk::YELLOW),
            render_order: 3
        })
        .with(Name::new_regular("flashbang"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 30 })
        .with(AreaOfEffect{ radius: 6 })
        .with(CausesConfusion{ turns: 8 })
        .with(MakesNoise{
            volume: 30,
            surprising: true,
            description: "a loud bang".to_string()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn stun_dart(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('↑'),
            fg: RGB::named(rltk::PINK),
            render_order: 3
        })
        .with(Name::new_regular("stun dart"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(CausesConfusion{ turns: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn long_stun_dart(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('↑'),
            fg: RGB::named(rltk::PINK),
            render_order: 3
        })
        .with(Name::new_regular("long stun dart"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(CausesConfusion{ turns: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn knife(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('/'),
            fg: RGB::named(rltk::CYAN),
            render_order: 3
        })
        .with(Name::new_regular("knife"))
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power: 1 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn sword(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('/'),
            fg: RGB::named(rltk::WHITE),
            render_order: 3
        })
        .with(Name::new_regular("sword"))
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power: 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn electro_sword(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('/'),
            fg: RGB::named(rltk::YELLOW),
            render_order: 3
        })
        .with(Name::new_regular("electro-sword"))
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn super_sword(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('/'),
            fg: RGB::named(rltk::BLUE),
            render_order: 3
        })
        .with(Name::new_regular("super-sword"))
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn light_armour(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('['),
            fg: RGB::named(rltk::CYAN),
            render_order: 3
        })
        .with(Name::new_regular("light armour"))
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Shield })
        .with(DefenceBonus{ defence: 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn medium_armour(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('['),
            fg: RGB::named(rltk::WHITE),
            render_order: 3
        })
        .with(Name::new_regular("medium armour"))
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Shield })
        .with(DefenceBonus{ defence: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn heavy_armour(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('['),
            fg: RGB::named(rltk::RED),
            render_order: 3
        })
        .with(Name::new_regular("heavy armour"))
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Shield })
        .with(DefenceBonus{ defence: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn shield(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('('),
            fg: RGB::named(rltk::WHITE),
            render_order: 3
        })
        .with(Name::new_regular("shield"))
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Shield })
        .with(DefenceBonus{ defence: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
