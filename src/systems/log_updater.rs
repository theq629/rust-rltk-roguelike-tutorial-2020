use specs::prelude::*;
use rltk::{Point};
use crate::{gamelog::{PlayerLog, GameLog, Scope}, Player, Viewshed, Position, InBackpack};

const MAX_LOG_SIZE: u32 = 1000;

pub struct LogUpdaterSystem {}

impl<'a> System<'a> for LogUpdaterSystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, PlayerLog>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, InBackpack>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut game_log,
            mut player_log,
            players,
            viewsheds,
            positions,
            in_backpacks
        ) = data;

        for (_player, viewshed) in (&players, &viewsheds).join() {
            for item in game_log.entries.iter() {
                let want =
                    match item.scope {
                        Scope::GLOBAL => true,
                        Scope::AT { at } => {
                            viewshed.visible_tiles.contains(&at)
                        }
                        Scope::ON { on } => {
                            if let Some(pos) = positions.get(on) {
                                viewshed.visible_tiles.contains(&Point::new(pos.x, pos.y))
                            } else if let Some(backpack) = in_backpacks.get(on) {
                                if let Some(pos) = positions.get(backpack.owner) {
                                    viewshed.visible_tiles.contains(&Point::new(pos.x, pos.y))
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        }
                    };
                if want {
                    player_log.entries.push(item.message.to_string());
                }
            }
            game_log.entries.clear();
        }

        let log_size = player_log.entries.len() as u32;
        if log_size > MAX_LOG_SIZE {
            player_log.entries.drain(0..((log_size - MAX_LOG_SIZE) as usize));
        }
    }
}
