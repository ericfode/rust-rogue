use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use crate::state::{State};
use crate::map::*;
use crate::components::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        // build the new position
        let x = min(78, max(0, pos.point.x + delta_x));
        let y = min(48, max(0, pos.point.y + delta_y));

        // check if the new position is walkable
        if map.tiles[xy_idx(x,y)] == TileType::Floor {
            pos.point.x = x;
            pos.point.y = y;
        }
    }
}


pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {} // Do nothing
        }
    }
}