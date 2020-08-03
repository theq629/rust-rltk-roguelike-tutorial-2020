use specs::prelude::*;
use specs::saveload::{SimpleMarker, MarkedBuilder};
use rltk::{RGB};
use super::{SerializeMe, CombatStats, Health, Player, Renderable, Name, Position, Viewshed, Monster, MonsterAI, BlocksTile, Item, ProvidesHealing, Consumable, Ranged, InflictsDamage, AreaOfEffect, Confusion, EquipmentSlot, Equippable, MeleePowerBonus, DefenceBonus, CanDoDances, dancing::Dance, Poise, liquids::Liquid, SpreadsLiquid, InFaction, factions::Faction, Stamina, MakesNoise};

#[derive(Clone, PartialEq)]
pub enum Stuff {
    Thrall,
    Vampire,
    Rabbit,
    BloodDart,
    BloodBalloon,
    OilDart,
    OilBalloon,
    HealthPotion,
    FireballScroll,
    ConfusionScroll,
    MagicMissileScroll,
    Firecracker,
    Flashbang,
    Dagger,
    Longsword,
    Shield,
    TowerShield,
}

impl Stuff {
    pub fn spawn(&self, ecs: &mut World, x: i32, y: i32) {
        match self {
            Stuff::Thrall => thrall(ecs, x, y),
            Stuff::Vampire => vampire(ecs, x, y),
            Stuff::Rabbit => rabbit(ecs, x, y),
            Stuff::BloodDart => blood_dart(ecs, x, y),
            Stuff::BloodBalloon => blood_balloon(ecs, x, y),
            Stuff::OilDart => oil_dart(ecs, x, y),
            Stuff::OilBalloon => oil_balloon(ecs, x, y),
            Stuff::HealthPotion => health_potion(ecs, x, y),
            Stuff::FireballScroll => fireball_scroll(ecs, x, y),
            Stuff::ConfusionScroll => confusion_scroll(ecs, x, y),
            Stuff::MagicMissileScroll => magic_missile_scroll(ecs, x, y),
            Stuff::Firecracker => firecracker(ecs, x, y),
            Stuff::Flashbang => flashbang(ecs, x, y),
            Stuff::Dagger => dagger(ecs, x, y),
            Stuff::Longsword => longsword(ecs, x, y),
            Stuff::Shield => shield(ecs, x, y),
            Stuff::TowerShield => tower_shield(ecs, x, y),
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

pub fn vampire(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Name::new_regular("vampire"))
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
        .with(CombatStats{ defence: 1, power: 1 })
        .with(Health{ max_health: 20, health: 20 })
        .with(Stamina{ max_stamina: 10, stamina: 10 })
        .with(Poise{ max_poise: 10, poise: 10 })
        .with(CanDoDances{
            dances: vec![Dance::CIRCLE],
            descriptors: vec!["cool", "awesome", "creepy", "scary", "bloodthirsty", "elegant"].iter().map(|s| s.to_string()).collect()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn thrall(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Name::new_regular("thrall"))
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
        .with(CombatStats{ defence: 1, power: 1 })
        .with(Health{ max_health: 10, health: 10 })
        .with(Stamina{ max_stamina: 5, stamina: 5 })
        .with(Poise{ max_poise: 5, poise: 5 })
        .with(CanDoDances{
            dances: vec![Dance::JITTER],
            descriptors: vec!["jittery", "clunky", "heavy", "solid"].iter().map(|s| s.to_string()).collect()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
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
        .with(Health{ max_health: 3, health: 3 })
        .with(Stamina{ max_stamina: 5, stamina: 5 })
        .with(Poise{ max_poise: 1, poise: 1 })
        .with(CanDoDances{
            dances: vec![Dance::HOP],
            descriptors: vec!["cute", "nimble", "furry", "hoppy"].iter().map(|s| s.to_string()).collect()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn oil_dart(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('/'),
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
            glyph: rltk::to_cp437('!'),
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
            glyph: rltk::to_cp437('/'),
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
            glyph: rltk::to_cp437('!'),
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

pub fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('¡'),
            fg: RGB::named(rltk::MAGENTA),
            render_order: 3
        })
        .with(Name::new_regular("health potion"))
        .with(Item{})
        .with(Consumable{})
        .with(ProvidesHealing{ heal_amount: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn magic_missile_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::CYAN),
            render_order: 3
        })
        .with(Name::new_regular("magic missile scroll"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 8 })
        .with(MakesNoise{
            volume: 64,
            surprising: true,
            description: "magic".to_string()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::ORANGE),
            render_order: 3
        })
        .with(Name::new_regular("fireball scroll"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 8 })
        .with(MakesNoise{
            volume: 128,
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
        .with(Ranged{ range: 10 })
        .with(Confusion{ turns: 4 })
        .with(MakesNoise{
            volume: 128,
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
        .with(Ranged{ range: 10 })
        .with(AreaOfEffect{ radius: 6 })
        .with(Confusion{ turns: 8 })
        .with(MakesNoise{
            volume: 256,
            surprising: true,
            description: "a loud bang".to_string()
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::PINK),
            render_order: 3
        })
        .with(Name::new_regular("confusion"))
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(Confusion{ turns: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn dagger(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('/'),
            fg: RGB::named(rltk::CYAN),
            render_order: 3
        })
        .with(Name::new_regular("dagger"))
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power: 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn longsword(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('/'),
            fg: RGB::named(rltk::YELLOW),
            render_order: 3
        })
        .with(Name::new_regular("longsword"))
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Melee })
        .with(MeleePowerBonus{ power: 2 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn shield(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('('),
            fg: RGB::named(rltk::CYAN),
            render_order: 3
        })
        .with(Name::new_regular("shield"))
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Shield })
        .with(DefenceBonus{ defence: 1 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn tower_shield(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('('),
            fg: RGB::named(rltk::YELLOW),
            render_order: 3
        })
        .with(Name::new_regular("tower shield"))
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Shield })
        .with(DefenceBonus{ defence: 1 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
