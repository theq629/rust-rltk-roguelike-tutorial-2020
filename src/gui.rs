use rltk::{RGB, Rltk, Point, VirtualKeyCode, Rect};
use super::{Health, Player, gamelog::PlayerLog, Map, Name, state::State, InBackpack, Viewshed, RunState, Equipped, Poise, drawing, dancing, text::capitalize, Stamina, cellinfo::cell_info, CanDoDances, HasAggroedMosters};
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
    let entities = ecs.entities();
    let players = ecs.read_storage::<Player>();
    let has_agroed = ecs.read_storage::<HasAggroedMosters>();

    let bg = RGB::from_u8(64, 64, 64);
    let values_fg = RGB::from_u8(192, 192, 192);
    let y = screen_height as i32 - 8;

    for x in 0..screen_width {
        ctx.set_bg(x, y, bg);
    }

    let mut x = 1;
    let stamina = ecs.read_storage::<Stamina>();
    for (_player, stamina) in (&players, &stamina).join() {
        x = 1 + draw_stat(capitalize(&Stamina::NAME), stamina.stamina, stamina.max_stamina, x, y, Stamina::colour(), values_fg, bg, ctx);
    }
    let poise = ecs.read_storage::<Poise>();
    for (_player, poise) in (&players, &poise).join() {
        x = 1 + draw_stat(capitalize(&Poise::NAME), poise.poise, poise.max_poise, x, y, Poise::colour(), values_fg, bg, ctx);
    }
    let health = ecs.read_storage::<Health>();
    for (entity, _player, health) in (&entities, &players, &health).join() {
        if let Some(_) = has_agroed.get(entity) {
            x = draw_stat(capitalize(&Health::NAME), health.health, health.max_health, x, y, Health::colour(), values_fg, bg, ctx);
        }
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

pub fn dance_menu(gs: &State, ctx: &mut Rltk) -> (ItemMenuResult, Option<dancing::Dance>) {
    let player_entity = gs.ecs.fetch::<Entity>();
    let can_do_dances = gs.ecs.read_storage::<CanDoDances>();
    let items = 
        if let Some(can_dance) = can_do_dances.get(*player_entity) {
            can_dance.dances.iter().map(|dance| {
                let name = format!("{} ({} steps)", capitalize(&dance.name()), dance.steps().len());
                (name, dance)
            }).collect()
        } else {
            Vec::new()
        };
    let (result, dance) = menu::<&dancing::Dance>(ctx, "Do which dance?".to_string(), "you can't do any dances".to_string(), items);
    (result, dance.map(|d| d.clone()))
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

        #[cfg(not(target_arch="wasm32"))]
        {
            let mut fg = continue_game_fg;
            if !save_exists {
                fg = fg.to_greyscale();
            }
            let bg = if selection == MainMenuSelection::LoadGame { sel_bg } else { unsel_bg };
            ctx.print_color_centered(y, fg, bg, "Continue Game");
            y += 1;
        }

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

pub fn game_over(won: bool, message: &String, ctx: &mut Rltk) -> GameOverResult {
    let (screen_width, screen_height) = ctx.get_char_size();

    let title_fg = RGB::from_u8(255, 255, 255);
    let message_fg = RGB::from_u8(192, 192, 192);
    let won_bg = RGB::from_u8(64, 64, 96);
    let lost_bg = RGB::from_u8(96, 64, 64);

    let won_lost_msg = if won { "won" } else { "lost" };
    let bg = if won { won_bg } else { lost_bg };

    let y = (screen_height - 8) / 2 - 3;
    ctx.fill_region(Rect::with_size(0, y - 1, screen_width, 6), rltk::to_cp437(' '), title_fg, bg);
    ctx.print_color_centered(y, title_fg, bg, format!("You {}!", won_lost_msg));
    ctx.print_color_centered(y + 2, message_fg, bg, message);
    ctx.print_color_centered(y + 4, title_fg, bg, "Press escape to return to the menu.");

    match ctx.key {
        Some(VirtualKeyCode::Escape) => GameOverResult::QuitToMenu,
        _ => GameOverResult::NoSelection
    }
}

pub fn show_full_log(gs: &mut State, ctx: &mut Rltk) -> (ItemMenuResult, Option<()>) {
    let (screen_width, screen_height) = ctx.get_char_size();

    let title_fg = RGB::from_u8(255, 255, 255);
    let title_bg = RGB::from_u8(0, 0, 0);
    let bg = RGB::from_u8(0, 0, 0);
    let fg = RGB::from_u8(128, 128, 128);
    let odd_bg = RGB::from_u8(24, 24, 24);
    let even_bg = RGB::from_u8(16, 16, 16);

    let log = gs.ecs.fetch::<PlayerLog>();

    ctx.fill_region(Rect::with_size(0, 0, screen_width, screen_height), rltk::to_cp437(' '), title_fg, bg);
    ctx.print_color(0, 0, title_fg, title_bg, "Log");
    ctx.print_color(0, screen_height - 1, title_fg, title_bg, "Escape to return to game");

    let avail_y = screen_height - 2;
    let end_i = log.entries.len() as i32;
    let start_i = i32::max(0, end_i - avail_y as i32);

    let mut y = 1;
    for i in start_i..end_i {
        let msg = &log.entries[i as usize];
        let bg = if i % 2 == 0 { even_bg } else { odd_bg };
        for x in 0..screen_width {
            ctx.set_bg(x, y, bg);
        }
        ctx.print_color(0, y, fg, bg, msg);
        y += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { (ItemMenuResult::Cancel, None) }
                _ => (ItemMenuResult::NoResponse, None)
            }
        }
    }
}

pub fn show_keys(ctx: &mut Rltk) -> (ItemMenuResult, Option<()>) {
    let keys = vec![
        ("escape", "return to main menu"),
        ("mouse", "look around"),
        ("mouse", "target ranged item (on use)"),
        ("arrows", "move"),
        ("hjklyubn", "move"),
        ("a + movement", "auto move"),
        (".", "skip a turn"),
        ("g", "get item"),
        ("i", "see inventory and use item"),
        ("d", "drop item"),
        ("r", "unequip item"),
        ("z", "do dance"),
        ("space", "go down stairs"),
        ("m", "show message log"),
        ("/", "show this help"),
    ];
    show_pairs(ctx, "Help".to_string(), &keys)
}

fn show_pairs<S: ToString>(ctx: &mut Rltk, title: String, items: &Vec<(S, S)>) -> (ItemMenuResult, Option<()>) {
    let items: Vec<(String, String)> = items.iter().map(|(key, value)| {
        (key.to_string(), value.to_string())
    }).collect();

    let (screen_width, screen_height) = ctx.get_char_size();
    let count = items.len();

    let bg = RGB::from_u8(64, 64, 64);
    let title_fg = RGB::from_u8(255, 255, 255);
    let key_fg = RGB::from_u8(255, 255, 255);
    let values_fg = RGB::from_u8(192, 192, 192);

    let space_for_items = if count > 0 { count } else { 1 };
    let start_x = (screen_width / 4 - 2) as i32;
    let start_y = ((screen_height / 2) - (space_for_items / 2) as u32) as i32;
    let width = (2 * screen_width / 4) as i32;
    let height = (space_for_items + 3) as i32;

    ctx.fill_region(Rect::with_size(start_x, start_y, width, height), rltk::to_cp437(' '), values_fg, bg);
    ctx.print_color(start_x, start_y, title_fg, bg, title);
    ctx.print_color(start_x, start_y + height, title_fg, bg, "Escape to return to game".to_string());

    let key_width = items.iter().map(|(k, _)| k.len() as i32).max().unwrap();

    let mut y = start_y + 2;
    for (key, value) in items.iter() {
        ctx.print_color(start_x, y, key_fg, bg, key);
        ctx.print_color(start_x + key_width + 1, y, values_fg, bg, value);
        y += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { (ItemMenuResult::Cancel, None) }
                _ => (ItemMenuResult::NoResponse, None)
            }
        }
    }
}
