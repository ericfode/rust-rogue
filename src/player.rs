use crate::components::*;
use crate::map::*;
use crate::state::{RunState, State};
use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut point = ecs.write_resource::<Point>();
    let entities = ecs.entities();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _player, pos, viewshed) in
        (&entities, &mut players, &mut positions, &mut viewsheds).join()
    {
        // build the new position
        let x = min(78, max(0, pos.point.x + delta_x));
        let y = min(48, max(0, pos.point.y + delta_y));

        let dest_idx = xy_idx(x, y);

        for potential_target in map.tile_content[dest_idx].iter() {
            let target = combat_stats.get(*potential_target);
            match target {
                None => {}
                Some(_t) => {
                    wants_to_melee
                        .insert(
                            entity,
                            WantsToMelee {
                                target: *potential_target,
                            },
                        )
                        .expect("cannont insert damage");
                    return;
                }
            }
        }

        // check if the new position is walkable
        if !map.tiles[dest_idx].is_blocker() {
            // update the player position
            pos.point.x = x;
            pos.point.y = y;
            // Update the global player position cache
            point.x = x;
            point.y = y;
            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => return RunState::AwaitingInput, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::U => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::Y => try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::M => try_move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::N => try_move_player(-1, 1, &mut gs.ecs),
            VirtualKeyCode::R => return RunState::RevealMap,
            _ => return RunState::AwaitingInput, // Do nothing
        },
    }
    RunState::PlayerTurn
}
