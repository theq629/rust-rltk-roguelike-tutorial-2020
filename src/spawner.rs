use rltk::{RandomNumberGenerator, Point};
use specs::prelude::*;
use std::collections::{HashSet};
use super::map::{MAPWIDTH};
use super::rect::{Rect};
use super::stuff::Stuff;

pub fn spawn(ecs: &mut World, rooms: &Vec<Rect>, map_depth: i32) {
    let start_rooms = rooms[0..1].to_vec();
    let other_rooms = rooms[1..].to_vec();
    spawn_rooms(ecs, start_rooms, start_room_table(map_depth));
    spawn_rooms(ecs, other_rooms, floor_table(map_depth));
}

fn spawn_rooms(ecs: &mut World, rooms: Vec<Rect>, spawn_table: Vec<(Stuff, i32, i32)>) {
    let mut to_spawn: Vec<(Stuff, Point)> = Vec::new();
    let mut spawn_points: HashSet<usize> = HashSet::new();

    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();

        for (stuff, min_num, max_num) in spawn_table {
            let num = rng.range(min_num, max_num + 1);
            for _ in 0..num {
                let mut tries = 0;
                loop {
                    let room_idx = rng.range(0, rooms.len());
                    let room = &rooms[room_idx];
                    let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                    let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                    let idx = (y * MAPWIDTH) + x;
                    if !spawn_points.contains(&idx) || tries >= 10 {
                        to_spawn.push((stuff.clone(), Point::new(x, y)));
                        break;
                    }
                    spawn_points.insert(idx);
                    tries += 1;
                }
            }
        }
    }

    for (stuff, point) in to_spawn {
        stuff.spawn(ecs, point.x, point.y);
    }
}

fn floor_table(map_depth: i32) -> Vec<(Stuff, i32, i32)> {
    vec![
        (Stuff::Rabbit, 20, 50),
        (Stuff::Thrall, 10, 20),
        (Stuff::Vampire, 5, 10),
        (Stuff::BloodDart, 1, 5),
        (Stuff::BloodBalloon, 1, 5),
        (Stuff::OilDart, 1, 5),
        (Stuff::OilBalloon, 1, 5),
        (Stuff::HealthPotion, 1, 5),
        (Stuff::Firecracker, 1, 5),
        (Stuff::Flashbang, 1, 5),
        (Stuff::Dagger, 1, 5),
        (Stuff::Shield, 1, 5),
        (Stuff::Longsword, 0 + map_depth, 1 + map_depth),
        (Stuff::TowerShield, 0 + map_depth, 1 + map_depth),
    ]
}

fn start_room_table(_map_depth: i32) -> Vec<(Stuff, i32, i32)> {
    vec![
        (Stuff::BloodDart, 3, 5),
        (Stuff::BloodBalloon, 3, 5),
        (Stuff::OilDart, 3, 5),
        (Stuff::OilBalloon, 3, 5),
        (Stuff::HealthPotion, 3, 5),
        (Stuff::FireballScroll, 3, 5),
        (Stuff::ConfusionScroll, 3, 5),
        (Stuff::MagicMissileScroll, 3, 5),
        (Stuff::Firecracker, 3, 5),
        (Stuff::Flashbang, 3, 5),
        (Stuff::Dagger, 1, 1),
        (Stuff::Shield, 1, 1),
        (Stuff::Longsword, 1, 1),
        (Stuff::TowerShield, 1, 1),
    ]
}
