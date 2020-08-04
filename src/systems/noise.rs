use specs::prelude::*;
use rltk::{Point};
use crate::{MakeNoise, Noise, gamelog::PlayerLog};

pub struct NoiseSystem {}

impl<'a> System<'a> for NoiseSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, MakeNoise>,
        WriteStorage<'a, Noise>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut make_noises,
            mut noises
        ) = data;

        for make_noise in make_noises.join() {
            let noise_entity = entities.create();
            noises.insert(noise_entity, Noise {
                location: make_noise.location.clone(),
                volume: make_noise.volume,
                faction: make_noise.faction,
                surprising: make_noise.surprising,
                description: make_noise.description.to_string()
            }).expect("Failed to insert noise.");
        }

        make_noises.clear();
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
            noises
        ) = data;

        for (noise,) in (&noises,).join() {
            if noise.surprising && can_hear(&noise.location, noise) {
                player_log.insert(&format!("You hear {}.", noise.description));
            }
        }
    }
}

pub struct NoiseCleanupSystem {}

impl<'a> System<'a> for NoiseCleanupSystem {
    type SystemData = (
        WriteStorage<'a, Noise>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut noises,
        ) = data;

        noises.clear();
    }
}

pub fn can_hear(pos: &Point, noise: &Noise) -> bool {
    let (dx, dy) = (pos.x - noise.location.x, pos.y - noise.location.y);
    let d = dx * dx + dy * dy;
    d < (noise.volume * noise.volume) as i32
}
