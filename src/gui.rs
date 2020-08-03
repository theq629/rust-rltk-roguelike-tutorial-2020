use rltk::{RGB, Rltk, Point, VirtualKeyCode, Rect};
use super::{Health, Player, gamelog::PlayerLog, Map, Name, state::State, InBackpack, Viewshed, RunState, Equipped, Poise, drawing, dancing, text::capitalize, Stamina, cellinfo::cell_info};
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection { NewGame, LoadGame, Quit }

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult {
    NoSelection{ selected: MainMenuSelection },
    Selected{ selected: MainMenuSelection }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel, NoResponse, Selected
}

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    draw_stats(ecs, ctx);
    draw_log(ecs, ctx);
    draw_tooltips(ecs, ctx);
}

fn draw_stats(ecs: &World, ctx: &mut Rltk) {
    let (screen_width, screen_height) = ctx.get_char_size();
    let players = ecs.read_storage::<Player>();

    let bg = RGB::from_u8(64, 64, 64);
    let values_fg = RGB::from_u8(192, 192, 192);
    let y = screen_height as i32 - 8;

    for x in 0..screen_width {
        ctx.set_bg(x, y, bg);
    }

    let mut x = 1;
    let health = ecs.read_storage::<Health>();
    for (_player, health) in (&players, &health).join() {
        x = 1 + draw_stat(capitalize(&Health::NAME), health.health, health.max_health, x, y, Health::colour(), values_fg, bg, ctx);
    }
    let stamina = ecs.read_storage::<Stamina>();
    for (_player, stamina) in (&players, &stamina).join() {
        x = 1 + draw_stat(capitalize(&Stamina::NAME), stamina.stamina, stamina.max_stamina, x, y, Stamina::colour(), values_fg, bg, ctx);
    }
    let poise = ecs.read_storage::<Poise>();
    for (_player, poise) in (&players, &poise).join() {
        x = draw_stat(capitalize(&Poise::NAME), poise.poise, poise.max_poise, x, y, Poise::colour(), values_fg, bg, ctx);
    }
}

fn draw_stat<S: ToString>(name: S, value: i32, max_value: i32, x: i32, y: i32, name_fg: RGB, value_fg: RGB, bg: RGB, ctx: &mut Rltk) -> i32 {
    let name = name.to_string();
    let name_len = name.len() as i32;
    let mut x = x;
    ctx.print_color(x, y, name_fg, bg, name);
    x += name_len + 1;
    let value_text = format!("{} / {}", value, max_value);
    let value_text_len = value_text.len() as i32;
    ctx.print_color(x, y, value_fg, bg, value_text);
    x += i32::max(7, value_text_len);
    x

}

fn draw_log(ecs: &World, ctx: &mut Rltk) {
    let (screen_width, screen_height) = ctx.get_char_size();
    let fg = RGB::from_u8(128, 128, 128);
    let odd_bg = RGB::from_u8(24, 24, 24);
    let even_bg = RGB::from_u8(16, 16, 16);
    let log = ecs.fetch::<PlayerLog>();
    let avail_y = 7;
    let mut y = screen_height - 1;
    for (i, msg) in log.entries.iter().enumerate().rev().take(avail_y as usize) {
        let bg = if i % 2 == 0 { even_bg } else { odd_bg };
        for x in 0..screen_width {
            ctx.set_bg(x, y, bg);
        }
        ctx.print_color(0, y, fg, bg, msg);
        y -= 1;
    }
}

fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let (screen_width, screen_height) = ctx.get_char_size();

    let map = ecs.fetch::<Map>();

    let bg = RGB::from_u8(64, 64, 64);
    let fg = RGB::from_u8(192, 192, 192);
    let arrow_fg = RGB::from_u8(255, 255, 255);

    let (mouse_x, mouse_y) = ctx.mouse_pos();
    if mouse_y >= (screen_height - 8) as i32 {
        return;
    }

    let world_mouse_pos = drawing::screen_to_world_point(Point::new(mouse_x, mouse_y), &ecs, ctx);
    if !map.point_valid(&world_mouse_pos) {
        return;
    }

    let tooltip = cell_info(&world_mouse_pos, ecs);

    if !tooltip.is_empty() {
        let mut width: i32 = 0;
        for s in tooltip.iter() {
            if width < s.len() as i32 {
                width = s.len() as i32;
            }
        }
        width += 3;

        let arrow_pos: Point;
        let arrow: String;
        let box_x: i32;
        let text_x: i32;
        if mouse_x > (screen_width / 2) as i32 {
            arrow_pos = Point::new(mouse_x - 2, mouse_y);
            arrow = "->".to_string();
            box_x = mouse_x - width;
            text_x = mouse_x - width;
        } else {
            arrow_pos = Point::new(mouse_x + 1, mouse_y);
            arrow = "<-".to_string();
            box_x = mouse_x + 1;
            text_x = mouse_x + 4;
        }
        ctx.fill_region(Rect::with_size(box_x, mouse_y, width - 1, (tooltip.len() - 1) as i32), rltk::to_cp437(' '), fg, bg);
        let mut y = mouse_y;
        for s in tooltip.iter() {
            ctx.print_color(text_x, y, fg, bg, s);
            y += 1;
        }
        ctx.print_color(arrow_pos.x, arrow_pos.y, arrow_fg, bg, &arrow);
    }
}

pub fn show_inventory(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    inventory_menu::<InBackpack>(gs, ctx, "Use which item?".to_string(), "nothing in inventory".to_string(), &|item: &InBackpack| item.owner == *player_entity)
}

pub fn drop_item_menu(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    inventory_menu::<InBackpack>(gs, ctx, "Drop which item?".to_string(), "nothing in inventory".to_string(), &|item: &InBackpack| item.owner == *player_entity)
}

pub fn remove_item_menu(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    inventory_menu::<Equipped>(gs, ctx, "Remove which item?".to_string(), "nothing equipped".to_string(), &|item: &Equipped| item.owner == *player_entity)
}

pub fn dance_menu(ctx: &mut Rltk) -> (ItemMenuResult, Option<&dancing::Dance>) {
    let items = dancing::ALL.iter().map(|dance| {
        (capitalize(&dance.name()), dance)
    }).collect();
    menu::<&dancing::Dance>(ctx, "Do which dance?".to_string(), "you can't do any dances".to_string(), items)
}

fn inventory_menu<C: Component>(gs: &State, ctx: &mut Rltk, title: String, empty_text: String, filter: &dyn Fn(&C) -> bool) -> (ItemMenuResult, Option<Entity>) {
    let entities = gs.ecs.entities();
    let names = gs.ecs.read_storage::<Name>();
    let backpack = gs.ecs.read_storage::<C>();

    let items = (&entities, &backpack, &names).join().filter(|(_, bp, _)|
        filter(bp)
    ).map(|(e, _, n)|
        (n.name.to_string(), e.clone())
    ).collect();

    menu::<Entity>(ctx, title, empty_text, items)
}

fn menu<T: Clone>(ctx: &mut Rltk, title: String, empty_text: String, items: Vec<(String, T)>) -> (ItemMenuResult, Option<T>) {
    let (screen_width, screen_height) = ctx.get_char_size();
    let count = items.len();

    let bg = RGB::from_u8(64, 64, 64);
    let title_fg = RGB::from_u8(255, 255, 255);
    let items_fg = RGB::from_u8(192, 192, 192);
    let key_fg = RGB::from_u8(192, 192, 64);

    let space_for_items = if count > 0 { count } else { 1 };
    let start_x = (screen_width / 4 - 2) as i32;
    let start_y = ((screen_height / 2) - (space_for_items / 2) as u32) as i32;
    let width = (2 * screen_width / 4) as i32;
    let height = (space_for_items + 3) as i32;

    ctx.fill_region(Rect::with_size(start_x, start_y, width, height), rltk::to_cp437(' '), items_fg, bg);
    ctx.print_color(start_x, start_y, title_fg, bg, title);
    ctx.print_color(start_x, start_y + height, title_fg, bg, "Escape to cancel".to_string());

    let mut y = start_y + 2;
    if count > 0 {
        let mut j = 0;
        for (name, _) in items.iter() {
            ctx.set(start_x, y, key_fg, bg, 97+j as rltk::FontCharType);
            ctx.print_color(start_x + 2, y, items_fg, bg, &name.to_string());
            y += 1;
            j += 1;
        }
    } else {
        ctx.print_color(start_x + 2, y, items_fg, bg, empty_text);
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { (ItemMenuResult::Cancel, None) }
                _ => {
                    let selection = rltk::letter_to_option(key);
                    if selection > -1 && selection < count as i32 {
                        return (ItemMenuResult::Selected, Some(items[selection as usize].1.clone()));
                    }
                    (ItemMenuResult::NoResponse, None)
                }
            }
        }
    }
}

pub fn ranged_target(gs: &mut State, ctx: &mut Rltk, range: i32) -> (ItemMenuResult, Option<Point>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let player_pos = gs.ecs.fetch::<Point>();
    let viewsheds = gs.ecs.read_storage::<Viewshed>();

    let title_fg = RGB::from_u8(255, 255, 255);
    let title_bg = RGB::from_u8(0, 0, 0);
    let avail_bg = RGB::from_u8(0, 0, 192);
    let target_valid_bg = RGB::from_u8(192, 0, 0);
    let target_invalid_bg = RGB::from_u8(192, 192, 192);

    ctx.print_color(5, 0, title_fg, title_bg, "Select Target:");

    let visible = viewsheds.get(*player_entity);
    let available_world_cells =
        if let Some(visible) = visible {
            visible.visible_tiles.iter().filter(|tile| {
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, **tile);
                distance <= range as f32
            }).map(|t| t.clone()).collect()
        } else {
            return (ItemMenuResult::Cancel, None);
        };
    let available_screen_cells = drawing::world_to_screen_points(&available_world_cells, &gs.ecs, ctx);

    for pos in available_screen_cells.iter() {
        ctx.set_bg(pos.x, pos.y, avail_bg);
    }

    let (mouse_x, mouse_y) = ctx.mouse_pos();
    let world_mouse_pos = drawing::screen_to_world_point(Point::new(mouse_x, mouse_y), &gs.ecs, ctx);
    let mut valid_target = false;
    for tile in available_world_cells.iter() {
        if tile.x == world_mouse_pos.x && tile.y == world_mouse_pos.y {
            valid_target = true;
            break;
        }
    }
    if valid_target {
        ctx.set_bg(mouse_x, mouse_y, target_valid_bg);
        if ctx.left_click {
            return (ItemMenuResult::Selected, Some(world_mouse_pos));
        }
    } else {
        ctx.set_bg(mouse_x, mouse_y, target_invalid_bg);
        if ctx.left_click {
            return (ItemMenuResult::Cancel, None);
        }
    }

    (ItemMenuResult::NoResponse, None)
}

pub fn main_menu(gs: &mut State, ctx: &mut Rltk) -> MainMenuResult {
    let (_, screen_height) = ctx.get_char_size();

    let save_exists = super::saveload_system::does_save_exist();
    let runstate = gs.ecs.fetch::<RunState>();

    let title_bg = RGB::from_u8(0, 0, 0);
    let unsel_bg = RGB::from_u8(0, 0, 0);
    let sel_bg = RGB::from_u8(64, 64, 64);
    let title_fg = RGB::from_u8(255, 255, 255);
    let continue_game_fg = Health::colour();
    let new_game_fg = Stamina::colour();
    let quit_fg = Poise::colour();

    let title_y = (screen_height / 4) as i32;
    let items_y = (screen_height / 2 - 2) as i32;

    ctx.print_color_centered(title_y, title_fg, title_bg, "LUNAR DANCE WAR WITH VAMPIRES");

    if let RunState::MainMenu{ menu_selection: selection } = *runstate {
        let mut selection = selection;
        if selection == MainMenuSelection::LoadGame && !save_exists {
            selection = MainMenuSelection::NewGame;
        }

        let mut y = items_y;

        let mut fg = continue_game_fg;
        if !save_exists {
            fg = fg.to_greyscale();
        }
        let bg = if selection == MainMenuSelection::LoadGame { sel_bg } else { unsel_bg };
        ctx.print_color_centered(y, fg, bg, "Continue Game");
        y += 1;

        let bg = if selection == MainMenuSelection::NewGame { sel_bg } else { unsel_bg };
        ctx.print_color_centered(y, new_game_fg, bg, "New Game");
        y += 1;

        let bg = if selection == MainMenuSelection::Quit { sel_bg } else { unsel_bg };
        ctx.print_color_centered(y, quit_fg, bg, "Quit");

        match ctx.key {
            None => return MainMenuResult::NoSelection{ selected: selection },
            Some(key) => {
                match key {
                    VirtualKeyCode::Escape => { return MainMenuResult::NoSelection{ selected: MainMenuSelection::Quit } }
                    VirtualKeyCode::Up | VirtualKeyCode::K => {
                        let mut newselection = selection;
                        match selection {
                            MainMenuSelection::LoadGame => {},
                            MainMenuSelection::NewGame => newselection = MainMenuSelection::LoadGame,
                            MainMenuSelection::Quit => newselection = MainMenuSelection::NewGame
                        }
                        return MainMenuResult::NoSelection{ selected: newselection }
                    }
                    VirtualKeyCode::Down | VirtualKeyCode::J => {
                        let mut newselection = selection;
                        match selection {
                            MainMenuSelection::LoadGame => newselection = MainMenuSelection::NewGame,
                            MainMenuSelection::NewGame => newselection = MainMenuSelection::Quit,
                            MainMenuSelection::Quit => {}
                        }
                        return MainMenuResult::NoSelection{ selected: newselection }
                    }
                    VirtualKeyCode::Return => { return MainMenuResult::Selected{ selected: selection } }
                    _ => return MainMenuResult::NoSelection{ selected: selection }
                }
            }
        }
    }

    MainMenuResult::NoSelection { selected: MainMenuSelection::NewGame }
}

#[derive(PartialEq, Copy, Clone)]
pub enum GameOverResult { NoSelection, QuitToMenu }

pub fn game_over(ctx: &mut Rltk) -> GameOverResult {
    ctx.print_color_centered(15, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Your journey has ended!");
    ctx.print_color_centered(17, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Press any key to return to the menu.");

    match ctx.key {
        None => GameOverResult::NoSelection,
        Some(_) => GameOverResult::QuitToMenu
    }
}
