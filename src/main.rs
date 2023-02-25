use crate::map::{default_map_config, make_dungeon, new_map};
use crate::state::{RunState, State, create_player};
use components::{register_all_components};
use gamelog::GameLog;
use monster::generate_monsters;
use rltk::{RltkBuilder};
use specs::prelude::*;
pub mod components;
pub mod gamelog;
pub mod map;
pub mod player;
pub mod state;
pub mod visibility_system;
pub mod monster_ai_system;
pub mod monster;
pub mod map_index_system;
pub mod damage_system;
pub mod melee_combat_system;
pub mod gui;
pub mod spawner;

fn main() -> rltk::BError {
    let mut context = RltkBuilder::simple80x50()
        .with_title("Rouge tutorial")
        .build()?;
    context.with_post_scanlines(true);
    let mut gs = State { ecs: World::new()};
    register_all_components(&mut gs.ecs);
    gs.ecs.insert(rtlk::RandomNumberGenerator::new());
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(GameLog{
        entries: vec!["Welcome to your nightmare".to_string()]
    });
    

    // Get the screen size
    let (screen_width, screen_height) = context.get_char_size();
    let (sx, sy) = (
        screen_width.try_into().unwrap(),
        (screen_height-7).try_into().unwrap(),
    );
    let mut map = new_map(sx, sy);
    let mut mgc = default_map_config();
    mgc.max_room_x = screen_width.try_into().unwrap();
    mgc.max_room_y = screen_height.try_into().unwrap();

    // And add some padding
    mgc.max_room_y = mgc.max_room_y-9;
    mgc.max_room_x = mgc.max_room_x-2;
    let mut rng = rltk::RandomNumberGenerator::new();
    make_dungeon(&mgc, &mut rng, &mut map);
    let start = map.rooms[0].center(); 

    generate_monsters(&mut gs, &mut rng, &map);
    create_player(&mut gs, start.x, start.y);

    gs.ecs.insert(start);
    gs.ecs.insert(map);
    gui::draw_ui(&gs.ecs, &mut context);
    rltk::main_loop(context, gs)
}
