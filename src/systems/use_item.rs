use specs::prelude::*;
use rltk::{Point};
use crate::{WantsToUseItem, Map, AreaOfEffect, systems::particle_system::ParticleBuilder, Monster, Player, ItemUseInProgress};

pub struct ItemUseSystem {}

impl<'a> System<'a> for ItemUseSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Map>,
        Entities<'a>,
        WriteExpect<'a, ParticleBuilder>,
        ReadStorage<'a, WantsToUseItem>,
        ReadStorage<'a, AreaOfEffect>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, ItemUseInProgress>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, player_pos, map, entities, mut particle_builder, wants_use, aoe, players, monsters, mut use_in_progress) = data;

        for (entity, useitem) in (&entities, &wants_use).join() {
            let mut target_tiles: Vec<Point> = Vec::new();
            let targets_centre: Point;
            let mut targets: Vec<Entity> = Vec::new();
            match useitem.target {
                None => {
                    targets_centre = player_pos.clone();
                    targets.push(*player_entity);
                }
                Some(target) => {
                    targets_centre = target.clone();
                    let area_effect = aoe.get(useitem.item);
                    match area_effect {
                        None => {
                            target_tiles.push(target.clone());
                            let idx = map.xy_idx(target.x, target.y);
                            for mob in map.tile_content[idx].iter() {
                                match (monsters.get(*mob), players.get(*mob)) {
                                    (Some(_), _) => targets.push(*mob),
                                    (_, Some(_)) => targets.push(*mob),
                                    _ => {}
                                }
                            }
                        }
                        Some(area_effect) => {
                            target_tiles = rltk::field_of_view(target, area_effect.radius, &*map);
                            target_tiles.retain(|p| p.x > 0 && p.x < map.width-1 && p.y > 0 && p.y < map.height-1);
                            for tile_idx in target_tiles.iter() {
                                let idx = map.xy_idx(tile_idx.x, tile_idx.y);
                                for mob in map.tile_content[idx].iter() {
                                    match (monsters.get(*mob), players.get(*mob)) {
                                        (Some(_), _) => targets.push(*mob),
                                        (_, Some(_)) => targets.push(*mob),
                                        _ => {}
                                    }
                                }
                                particle_builder.request(tile_idx.x, tile_idx.y, rltk::RGB::named(rltk::ORANGE), rltk::to_cp437('░'), 200.0);
                            }
                        }
                    }
                }
            }

            use_in_progress.insert(entity, ItemUseInProgress {
                item: useitem.item,
                targets_centre: targets_centre,
                target_tiles: target_tiles,
                targets: targets
            }).expect("Failed to insert item use in progress");
        }
    }
}
