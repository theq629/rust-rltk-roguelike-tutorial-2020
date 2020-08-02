use rltk::{Rltk, VirtualKeyCode, Point};
use specs::prelude::*;
use std::cmp::{max, min};
use super::{state::State, Position, Player, Map, RunState, Health, WantsToMelee, WantsToPickupItem, Item, gamelog::PlayerLog, TileType, systems::auto_movement_system, WantsToMove, Resting}; 

pub struct KeyState {
    pub requested_auto_move: bool
}

fn skip_turn(ecs: &mut World) -> RunState {
    let player_entity = ecs.fetch::<Entity>();
    let mut resting = ecs.write_storage::<Resting>();
    resting.insert(*player_entity, Resting {}).expect("Failed to insert resting.");
    RunState::PlayerTurn
}

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut auto_move = false;
    {
        let mut state = ecs.fetch_mut::<KeyState>();
        if state.requested_auto_move {
            state.requested_auto_move = false;
            auto_move = true;
        }
    }
    let player_entity = *ecs.fetch::<Entity>();
    if auto_move {
        auto_movement_system::start(ecs, player_entity, Point::new(delta_x, delta_y));
        return;
    } else {
        auto_movement_system::stop(ecs, player_entity);
    }

    let positions = ecs.read_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let health = ecs.read_storage::<Health>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let mut wants_to_moves = ecs.write_storage::<WantsToMove>();
    let entities = ecs.entities();
    let map = ecs.fetch::<Map>();

    for (_player, pos, entity) in (&mut players, &positions, &entities).join() {
        let dest_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        for potential_target in map.tile_content[dest_idx].iter() {
            let target = health.get(*potential_target);
            match target {
                None => {}
                Some(_t) => {
                    wants_to_melee.insert(entity, WantsToMelee{ target: *potential_target }).expect("Add target failed");
                    return;
                }
            }
        }
        if !map.blocked[dest_idx] {
            wants_to_moves.insert(entity, WantsToMove {
                source: Point::new(pos.x, pos.y),
                destination: Point::new(
                    min(map.width - 1, max(0, pos.x + delta_x)),
                    min(map.height - 1, max(0, pos.y + delta_y))
                )
            }).expect("Failed to insert wants move.");
        }
    }
}

fn get_item(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut player_log = ecs.fetch_mut::<PlayerLog>();

    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        None => player_log.entries.push("There is nothing here to pick up.".to_string()),
        Some(item) => {
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            pickup.insert(*player_entity, WantsToPickupItem{ collected_by: *player_entity, item }).expect("Unable to insert want to pickup");
        }
    }
}

fn try_next_level(ecs: &mut World) -> bool {
    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);
    if map.tiles[player_idx] == TileType::DownStairs {
        true
    } else {
        let mut player_log = ecs.fetch_mut::<PlayerLog>();
        player_log.insert(&"There is no way down from here.");
        false
    }
}

fn handle_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { return RunState::AwaitingInput },
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad9 |
            VirtualKeyCode::Y => try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 |
            VirtualKeyCode::U => try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 |
            VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 |
            VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),

            VirtualKeyCode::A => {
                let mut state = gs.ecs.fetch_mut::<KeyState>();
                state.requested_auto_move = true;
            }

            VirtualKeyCode::Period => {
                if try_next_level(&mut gs.ecs) {
                    return RunState::NextLevel;
                }
            },

            VirtualKeyCode::Numpad5 |
            VirtualKeyCode::Space => {
                return skip_turn(&mut gs.ecs)
            },

            VirtualKeyCode::G => get_item(&mut gs.ecs),

            VirtualKeyCode::I => return RunState::ShowInventory,

            VirtualKeyCode::D => return RunState::ShowDropItem,

            VirtualKeyCode::R => return RunState::ShowRemoveItem,

            VirtualKeyCode::M => return RunState::ShowDanceMenu,

            VirtualKeyCode::Escape => return RunState::SaveGame,

            _ => { return RunState::AwaitingInput }
        },
    }

    RunState::PlayerTurn
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    let newrunstate = handle_input(gs, ctx);
    let player_entity = *gs.ecs.fetch::<Entity>();
    match newrunstate {
        RunState::AwaitingInput => {
            if auto_movement_system::is_auto_moving(&gs.ecs, player_entity) {
                return RunState::PlayerTurn
            }
        }
        RunState::PlayerTurn => {}
        _ => {
            auto_movement_system::stop(&mut gs.ecs, player_entity);
        }
    }
    newrunstate
}
