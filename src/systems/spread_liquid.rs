use specs::prelude::*;
use crate::{Map, ItemUseInProgress, SpreadsLiquid};

pub struct SpreadLiquidSystem {}

impl<'a> System<'a> for SpreadLiquidSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, ItemUseInProgress>,
        ReadStorage<'a, SpreadsLiquid>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, use_in_progress, liquid_spreaders) = data;

        for (useitem,) in (&use_in_progress,).join() {

            if let Some(spreads_liquid)  = liquid_spreaders.get(useitem.item) {
                let target_tile_idxs: Vec<usize> = useitem.target_tiles.iter().map(|t| map.xy_idx(t.x, t.y)).collect();
                for tile_idx in target_tile_idxs.iter() {
                    map.stains[*tile_idx].insert(spreads_liquid.liquid);
                }
            }
        }
    }
}
