use crate::components::*;
use crate::map::*;
use crate::state::*;
use crate::visibility_system::*;
use rltk::{GameState, Point, Rltk, RltkBuilder, VirtualKeyCode, RGB};
use specs::prelude::*;

pub mod components;
pub mod map;
pub mod player;
pub mod state;
pub mod visibility_system;
pub mod monster_ai_system;

fn main() -> rltk::BError {
    let context = RltkBuilder::simple80x50()
        .with_title("Rouge tutorial")
        .build()?;
    let mut gs = State { ecs: World::new(), runstate: RunState::Running };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    // Get the screen size
    let (screen_width, screen_height) = context.get_char_size();
    let (sx, sy) = (
        screen_width.try_into().unwrap(),
        screen_height.try_into().unwrap(),
    );
    let mut map = new_map(sx, sy);
    let mgc = default_map_config();
    let mut rng = rltk::RandomNumberGenerator::new();
    make_dungeon(&mgc, &mut rng, &mut map);
    let start = map.rooms[0].center(); 

    create_player(&mut gs, start.x, start.y);
    for (i,room ) in map.rooms.iter().skip(1).enumerate(){
        let (glyph , name)= match rng.roll_dice(1, 2) {
            1 => { 
                (rltk::to_cp437('g'),
                "goblin".to_string())
            },
            _ => { 
                (rltk::to_cp437('o'),
                "orgy hunter".to_string())
            }
        };
        let (x, y) = room.center().to_tuple();
        create_monster(
            &mut gs,
            x,
            y,
            glyph,
            RGB::named(rltk::RED),
            RGB::named(rltk::BLACK),
            format!("{} #{}",name, i)
        );
    }
    gs.ecs.insert(start);
    gs.ecs.insert(map);
    rltk::main_loop(context, gs)
}
