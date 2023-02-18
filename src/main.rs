use rltk::{Rltk, GameState, RGB, VirtualKeyCode, RltkBuilder, Point};
use specs::prelude::*;
use crate::components::*;
use crate::state::*;
use crate::map::*;

pub mod player;
pub mod components;
pub mod state;
pub mod map;



fn main() -> rltk::BError{
    let context = RltkBuilder::simple80x50()
        .with_title("Rouge tutorial")
        .build()?;
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    // Get the screen size
    let (screen_width, screen_height) = context.get_char_size();
    let (sx, sy) = (screen_width.try_into().unwrap(),screen_height.try_into().unwrap());
    let mut map = new_map(sx, sy);
    let mgc = default_map_config();
    let mut rng = rltk::RandomNumberGenerator::new();
    make_dungeon(&mgc, &mut rng, &mut map);
    let start = find_starting_position(&mut map);
    gs.ecs.insert(map);

    create_player(&mut gs, start.x, start.y);
    rltk::main_loop(context, gs)
}
