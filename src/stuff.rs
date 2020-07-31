use specs::prelude::*;
use specs::saveload::{SimpleMarker, MarkedBuilder};
use rltk::{RGB};
use super::{SerializeMe, CombatStats, Player, Renderable, Name, Position, Viewshed, Monster, BlocksTile, Item, ProvidesHealing, Consumable, Ranged, InflictsDamage, AreaOfEffect, Confusion, EquipmentSlot, Equippable, MeleePowerBonus, DefenceBonus, CanDoDances, dancing::Dance, Awe};

pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            render_order: 0
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(Name{ name: "Player".to_string() })
        .with(CombatStats{ max_hp: 30, hp: 30, defence: 2, power: 5 })
        .with(Awe{ max_awe: 10, awe: 0 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

pub fn vampire(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Name{ name: "Vampire".to_string() })
        .with(Renderable{
            glyph: rltk::to_cp437('V'),
            fg: RGB::named(rltk::WHITE),
            render_order: 1
        })
        .with(Monster{})
        .with(BlocksTile{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(CombatStats{ max_hp: 1, hp: 1, defence: 1, power: 1 })
        .with(CanDoDances{ dances: vec![Dance::CIRCLE] })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn thrall(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Name{ name: "Thrall".to_string() })
        .with(Renderable{
            glyph: rltk::to_cp437('v'),
            fg: RGB::named(rltk::WHITE),
            render_order: 1
        })
        .with(Monster{})
        .with(BlocksTile{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(CombatStats{ max_hp: 1, hp: 1, defence: 1, power: 1 })
        .with(CanDoDances{ dances: vec![Dance::JITTER] })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn rabbit(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Name{ name: "Rabbit".to_string() })
        .with(Renderable{
            glyph: rltk::to_cp437('r'),
            fg: RGB::named(rltk::WHITE),
            render_order: 1
        })
        .with(Monster{})
        .with(BlocksTile{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(CombatStats{ max_hp: 1, hp: 1, defence: 1, power: 1 })
        .with(CanDoDances{ dances: vec![Dance::HOP] })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('¡'),
            fg: RGB::named(rltk::MAGENTA),
            render_order: 2
        })
        .with(Name{ name: "Health Potion".to_string() })
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
            render_order: 2
        })
        .with(Name{ name: "Magic Missile Scroll".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn fireball_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::ORANGE),
            render_order: 2
        })
        .with(Name{ name: "Fireball Scroll".to_string() })
        .with(Item{})
        .with(Consumable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 8 })
        .with(AreaOfEffect{ radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437(')'),
            fg: RGB::named(rltk::PINK),
            render_order: 2
        })
        .with(Name{ name: "Confusion Scroll".to_string() })
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
            render_order: 2
        })
        .with(Name{ name: "Dagger".to_string() })
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
            render_order: 2
        })
        .with(Name{ name: "Longsword".to_string() })
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
            render_order: 2
        })
        .with(Name{ name: "Shield".to_string() })
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
            render_order: 2
        })
        .with(Name{ name: "Tower Shield".to_string() })
        .with(Item{})
        .with(Equippable{ slot: EquipmentSlot::Shield })
        .with(DefenceBonus{ defence: 1 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
