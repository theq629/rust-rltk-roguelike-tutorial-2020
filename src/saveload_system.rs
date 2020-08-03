use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator, SerializeComponents, DeserializeComponents, MarkedBuilder};
use specs::error::NoError;
use super::components::*;
use std::fs::File;
use std::path::Path;
use std::fs;

const SAVE_FILE_PATH: &str = "./savegame.json";

macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty ), *) => {
        $(
        SerializeComponents::<NoError, SimpleMarker<SerializeMe>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

macro_rules! deserialize_individually {
    ($ecs:expr, $de:expr, $data:expr, $( $type:ty ), *) => {
        $(
        DeserializeComponents::<NoError, _>::deserialize(
            &mut ( &mut $ecs.write_storage::<$type>(), ),
            &mut $data.0,
            &mut $data.1,
            &mut $data.2,
            &mut $de,
        )
        .unwrap();
        )*
    };
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_game(ecs: &mut World) {
    let map_copy = ecs.get_mut::<super::map::Map>().unwrap().clone();
    let player_log_copy = ecs.get_mut::<super::gamelog::PlayerLog>().unwrap().clone();
    let savehelper = ecs
        .create_entity()
        .with(SerializationHelper{
            map: map_copy,
            player_log: player_log_copy,
        })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    {
        let data = (ecs.entities(), ecs.read_storage::<SimpleMarker<SerializeMe>>());
        let writer = File::create(SAVE_FILE_PATH).unwrap();
        let mut serializer = serde_json::Serializer::new(writer);

        serialize_individually!(ecs, serializer, data, Position, Renderable, Player, Viewshed, Monster, MonsterAI, Name, BlocksTile, Health, CombatStats, SufferDamage, WantsToMelee, Item, Consumable, Ranged, InflictsDamage, AreaOfEffect, Confusion, ProvidesHealing, InBackpack, WantsToPickupItem, WantsToUseItem, WantsToDropItem, WantsToRemoveItem, Equippable, Equipped, MeleePowerBonus, DefenceBonus, SerializationHelper, ParticleLifetime, Dancing, Poise, EffectRequest, Awestruck, HasArgroedMonsters, WantsToMove, SpreadsLiquid, InFaction, Stamina);
    }

    ecs.delete_entity(savehelper).expect("Crash on cleanup")
}

#[cfg(target_arch = "wasm32")]
pub fn save_game(_ecs: &mut World) {
}

pub fn does_save_exist() -> bool {
    Path::new(SAVE_FILE_PATH).exists()
}

pub fn load_game(ecs: &mut World) {
    {
        let mut to_delete = Vec::new();
        for e in ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            ecs.delete_entity(*del).expect("Deletion failed");
        }
    }

    let data = fs::read_to_string(SAVE_FILE_PATH).unwrap();
    let mut de = serde_json::Deserializer::from_str(&data);

    {
        let mut d = (&mut ecs.entities(), &mut ecs.write_storage::<SimpleMarker<SerializeMe>>(), &mut ecs.write_resource::<SimpleMarkerAllocator<SerializeMe>>());
        deserialize_individually!(ecs, de, d, Position, Renderable, Player, Viewshed, Monster, MonsterAI, Name, BlocksTile, Health, CombatStats, SufferDamage, WantsToMelee, Item, Consumable, Ranged, InflictsDamage, AreaOfEffect, Confusion, ProvidesHealing, InBackpack, WantsToPickupItem, WantsToUseItem, WantsToDropItem, WantsToRemoveItem, Equippable, Equipped, MeleePowerBonus, DefenceBonus, SerializationHelper, ParticleLifetime, Dancing, Poise, EffectRequest, Awestruck, HasArgroedMonsters, WantsToMove, SpreadsLiquid, InFaction, Stamina);
    }

    let mut deleteme: Option<Entity> = None;
    {
        let entities = ecs.entities();
        let helper = ecs.read_storage::<SerializationHelper>();
        let player = ecs.read_storage::<Player>();
        let position = ecs.read_storage::<Position>();
        for (e, h) in (&entities, &helper).join() {
            let mut map = ecs.write_resource::<super::map::Map>();
            *map = h.map.clone();
            map.tile_content = vec![Vec::new(); super::map::MAPCOUNT];
            let mut player_log = ecs.write_resource::<super::gamelog::PlayerLog>();
            *player_log = h.player_log.clone();
            deleteme = Some(e);
        }
        for (e, _p, pos) in (&entities, &player, &position).join() {
            let mut ppos = ecs.write_resource::<rltk::Point>();
            *ppos = rltk::Point::new(pos.x, pos.y);
            let mut player_resource = ecs.write_resource::<Entity>();
            *player_resource = e;
        }
    }
    ecs.delete_entity(deleteme.unwrap()).expect("Unable to delete helper");
}

pub fn delete_save() {
    if Path::new(SAVE_FILE_PATH).exists() {
        std::fs::remove_file(SAVE_FILE_PATH).expect("Unable to delete file");
    }
}
