extern crate serde;
use rltk::{Rltk, GameState};
use specs::prelude::*;

mod components;
pub use components::*;
mod dancing;
mod liquids;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::*;
mod drawing;
mod gui;
mod gamelog;
mod stuff;
mod spawner;
mod saveload_system;
mod random_table;
mod systems;
use systems::damage_system::{delete_the_dead};
mod state;
use state::{Turn};
mod text;
mod factions;
mod cellinfo;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
    ShowDropItem,
    ShowTargeting { range: i32, item: Entity },
    ShowRemoveItem,
    ShowDanceMenu,
    ShowLog,
    MainMenu { menu_selection: gui::MainMenuSelection },
    SaveGame,
    NextLevel,
    GameOver
}

impl state::State {
    fn draw_world(&mut self, ctx: &mut Rltk) {
        drawing::draw_world(&self.ecs, ctx);
        gui::draw_ui(&self.ecs, ctx);
    }
}

impl GameState for state::State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        systems::particle_system::cull_dead_particles(&mut self.ecs, ctx);
        
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        match newrunstate {
            RunState::MainMenu {..} => {},
            _ => { self.draw_world(ctx); }
        }

        match newrunstate {
            RunState::PreRun => {
                self.reset_world();
                self.setup_world();
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                let mut busy = false;
                {
                    let player = *self.ecs.fetch::<Entity>();
                    let dancers = self.ecs.write_storage::<Dancing>();
                    if let Some(_) = dancers.get(player) {
                        busy = true;
                    }
                }
                if busy {
                    newrunstate = RunState::PlayerTurn;
                } else {
                    newrunstate = player_input(self, ctx);
                }
            }
            RunState::PlayerTurn => {
                self.next_turn();
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::ShowInventory => {
                let result = gui::show_inventory(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let is_ranged = self.ecs.read_storage::<Ranged>();
                        let is_item_ranged = is_ranged.get(item_entity);
                        if let Some(is_item_ranged) = is_item_ranged {
                            newrunstate = RunState::ShowTargeting{ range: is_item_ranged.range, item: item_entity };
                        } else {
                            let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                            intent.insert(*self.ecs.fetch::<Entity>(), WantsToUseItem{ item: item_entity, target: None }).expect("Unable to insert intent");
                            newrunstate = RunState::PlayerTurn;
                        }
                    }
                }
            }
            RunState::ShowDropItem => {
                let result = gui::drop_item_menu(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToDropItem>();
                        intent.insert(*self.ecs.fetch::<Entity>(), WantsToDropItem{ item: item_entity }).expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowTargeting{range, item} => {
                let result = gui::ranged_target(self, ctx, range);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                        intent.insert(*self.ecs.fetch::<Entity>(), WantsToUseItem{ item, target: result.1 }).expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowRemoveItem => {
                let result = gui::remove_item_menu(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToRemoveItem>();
                        intent.insert(*self.ecs.fetch::<Entity>(), WantsToRemoveItem{ item: item_entity }).expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowDanceMenu => {
                let result = gui::dance_menu(ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let dance = result.1.unwrap().clone();
                        let mut intent = self.ecs.write_storage::<WantsToDance>();
                        intent.insert(*self.ecs.fetch::<Entity>(), WantsToDance{
                            dance: dance,
                            repetitions: 1 as u32
                        }).expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowLog => {
                let result = gui::show_full_log(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => newrunstate = RunState::AwaitingInput
                }
            }
            RunState::MainMenu{..} => {
                let result = gui::main_menu(self, ctx);
                match result {
                    gui::MainMenuResult::NoSelection{ selected } => newrunstate = RunState::MainMenu{ menu_selection: selected },
                    gui::MainMenuResult::Selected{ selected } => {
                        match selected {
                            gui::MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
                            gui::MainMenuSelection::LoadGame => {
                                saveload_system::load_game(&mut self.ecs);
                                newrunstate = RunState::AwaitingInput;
                                saveload_system::delete_save();
                            }
                            gui::MainMenuSelection::Quit => { ::std::process::exit(0); }
                        }
                    }
                }
            }
            RunState::SaveGame => {
                saveload_system::save_game(&mut self.ecs);
                newrunstate = RunState::MainMenu{ menu_selection: gui::MainMenuSelection::LoadGame };
            },
            RunState::NextLevel => {
                self.goto_next_level();
                newrunstate = RunState::PreRun;
            },
            RunState::GameOver => {
                let result = gui::game_over(ctx);
                match result {
                    gui::GameOverResult::NoSelection => {}
                    gui::GameOverResult::QuitToMenu => {
                        self.game_over_cleanup();
                        newrunstate = RunState::MainMenu{ menu_selection: gui::MainMenuSelection::NewGame };
                    }
                }
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }

        delete_the_dead(&mut self.ecs);
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = state::State::new();
    setup_ecs(&mut gs.ecs);
    gs.ecs.insert(KeyState{ requested_auto_move: false });
    gs.ecs.insert(RunState::MainMenu { menu_selection: gui::MainMenuSelection::LoadGame });
    gs.ecs.insert(rltk::RandomNumberGenerator::new());
    gs.ecs.insert(systems::particle_system::ParticleBuilder::new());
    gs.ecs.insert(gamelog::PlayerLog::new());
    gs.ecs.insert(gamelog::GameLog::new());
    gs.setup_world();
    rltk::main_loop(context, gs)
}
