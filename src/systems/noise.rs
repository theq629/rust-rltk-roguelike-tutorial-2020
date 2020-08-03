use specs::prelude::*;
use crate::{MakeNoise, Noise, Turn, gamelog::PlayerLog};

pub struct NoiseSystem {}

impl<'a> System<'a> for NoiseSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Turn>,
        WriteStorage<'a, MakeNoise>,
        WriteStorage<'a, Noise>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            turn,
            mut make_noises,
            mut noises
        ) = data;

        let mut to_delete = Vec::new();
        for (entity, noise) in (&entities, &noises).join() {
            if noise.turn + noise.lasts <= *turn {
                to_delete.push(entity);
            }
        }

        for make_noise in make_noises.join() {
            let noise_entity = entities.create();
            noises.insert(noise_entity, Noise {
                turn: *turn,
                location: make_noise.location.clone(),
                volume: make_noise.volume,
                lasts: u32::min(make_noise.volume, 6),
                faction: make_noise.faction,
                surprising: make_noise.surprising,
                description: make_noise.description.to_string(),
                player_processed: false
            }).expect("Failed to insert noise.");
        }
        make_noises.clear();

        for entity in to_delete {
            entities.delete(entity).expect("Failed to delete noise.");
        }
    }
}

pub struct PlayerListeningSystem {}

impl<'a> System<'a> for PlayerListeningSystem {
    type SystemData = (
        WriteExpect<'a, PlayerLog>,
        WriteStorage<'a, Noise>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut player_log,
            mut noises
        ) = data;

        for (mut noise,) in (&mut noises,).join() {
            if !noise.player_processed {
                if noise.surprising {
                    player_log.insert(&format!("You hear {}.", noise.description));
                }
                noise.player_processed = true;
            }
        }
    }
}
