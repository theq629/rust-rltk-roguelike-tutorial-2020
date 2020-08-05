use specs::prelude::*;
use crate::{ItemUseInProgress, MakesNoise, MakeNoise};

pub struct MakeNoiseSystem {}

impl<'a> System<'a> for MakeNoiseSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ItemUseInProgress>,
        ReadStorage<'a, MakesNoise>,
        WriteStorage<'a, MakeNoise>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, use_in_progress, makes_noises, mut make_noises) = data;

        for (entity, useitem) in (&entities, &use_in_progress).join() {
            if let Some(makes_noise) = makes_noises.get(useitem.item) {
                make_noises.insert(entity, MakeNoise {
                    location: useitem.targets_centre.clone(),
                    volume: makes_noise.volume,
                    faction: None,
                    surprising: makes_noise.surprising,
                    description: makes_noise.description.to_string()
                }).expect("Failed to insert make noise.");
            }
        }
    }
}
