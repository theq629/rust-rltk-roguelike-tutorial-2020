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
    match map_depth {
        1 =>
            vec![
                (Stuff::Rabbit, 15, 30),
                (Stuff::BigRabbit, 5, 10),
                (Stuff::Thrall, 1, 5),
                (Stuff::ToughThrall, 0, 1),
                (Stuff::StunDart, 1, 1),
                (Stuff::Dart, 1, 1),
                (Stuff::WaterDart, 2, 3),
                (Stuff::WaterBalloon, 1, 2),
                (Stuff::OilDart, 0, 1),
                (Stuff::OilBalloon, 0, 1),
                (Stuff::Coffee, 1, 2),
                (Stuff::StrongCoffee, 0, 1),
                (Stuff::HealthKit, 1, 2),
                (Stuff::Firecracker, 2, 3),
                (Stuff::Flashbang, 1, 2),
                (Stuff::Knife, 1, 2),
                (Stuff::LightArmour, 1, 2),
            ],
        2 =>
            vec![
                (Stuff::Rabbit, 5, 10),
                (Stuff::BigRabbit, 5, 10),
                (Stuff::Thrall, 10, 20),
                (Stuff::ToughThrall, 5, 10),
                (Stuff::StunDart, 1, 2),
                (Stuff::LongStunDart, 1, 1),
                (Stuff::Dart, 1, 1),
                (Stuff::WaterDart, 1, 3),
                (Stuff::WaterBalloon, 1, 2),
                (Stuff::OilDart, 2, 3),
                (Stuff::OilBalloon, 1, 2),
                (Stuff::BloodDart, 0, 1),
                (Stuff::BloodBalloon, 0, 1),
                (Stuff::Grenade, 0, 1),
                (Stuff::Coffee, 1, 2),
                (Stuff::StrongCoffee, 0, 1),
                (Stuff::HealthKit, 1, 2),
                (Stuff::SuperHealthKit, 0, 1),
                (Stuff::Firecracker, 2, 3),
                (Stuff::Flashbang, 1, 2),
                (Stuff::Sword, 1, 2),
                (Stuff::MediumArmour, 1, 2),
                (Stuff::Shield, 0, 1),
            ],
        3 =>
            vec![
                (Stuff::Rabbit, 1, 3),
                (Stuff::BigRabbit, 1, 3),
                (Stuff::Thrall, 5, 10),
                (Stuff::ToughThrall, 2, 5),
                (Stuff::Vampire, 10, 20),
                (Stuff::OldVampire, 5, 10),
                (Stuff::StunDart, 1, 2),
                (Stuff::LongStunDart, 1, 2),
                (Stuff::Dart, 1, 1),
                (Stuff::WaterDart, 1, 3),
                (Stuff::WaterBalloon, 1, 2),
                (Stuff::OilDart, 1, 3),
                (Stuff::OilBalloon, 1, 2),
                (Stuff::BloodDart, 2, 3),
                (Stuff::BloodBalloon, 1, 2),
                (Stuff::Grenade, 1, 1),
                (Stuff::Coffee, 1, 2),
                (Stuff::StrongCoffee, 1, 2),
                (Stuff::HealthKit, 1, 2),
                (Stuff::SuperHealthKit, 1, 2),
                (Stuff::Firecracker, 2, 3),
                (Stuff::Flashbang, 1, 2),
                (Stuff::ElectroSword, 1, 2),
                (Stuff::HeavyArmour, 1, 2),
                (Stuff::Shield, 1, 2),
                (Stuff::SuperSword, 0, 1),
            ],
        _ =>
            vec![
            ],
    }
}

fn start_room_table(map_depth: i32) -> Vec<(Stuff, i32, i32)> {
    match map_depth {
        1 =>
            vec![
                (Stuff::WaterDart, 1, 1),
                (Stuff::WaterBalloon, 1, 1),
                (Stuff::Knife, 1, 1),
                (Stuff::Firecracker, 1, 1),
                (Stuff::Coffee, 1, 1),
            ],
        2 =>
            vec![
                (Stuff::Coffee, 1, 1),
            ],
        3 =>
            vec![
                (Stuff::Coffee, 1, 1),
            ],
        _ =>
            vec![
            ],
    }
}
