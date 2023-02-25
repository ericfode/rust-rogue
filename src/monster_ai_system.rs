use crate::{
    components::{Monster, Name, Position, Viewshed, WantsToMelee},
    map::{xy_idx, Map},
    state::RunState,
};
use rltk::{console, Algorithm2D, Point};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, RunState>,
        ReadExpect<'a, Entity>, // The player entity
        WriteExpect<'a, Map>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Name>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, WantsToMelee>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            runstate,
            player,
            mut map,
            monsters,
            mut viewshed,
            mut pos,
            name,
            player_pos,
            mut wants_to_melee,
        ) = data;

        if *runstate != RunState::MonsterTurn {
            return;
        }

        for (ent, monster, mut viewshed, name, mut pos) in
            (&entities, &monsters, &mut viewshed, &name, &mut pos).join()
        {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} leers at you", name.name));
                // Check to see if the player is close enough to attack
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(pos.point, *player_pos);

                // diagonal attack works too
                if distance < 1.5 {
                    wants_to_melee
                        .insert(ent, WantsToMelee { target: *player })
                        .expect("Unagle to insert attack");
                }

                if !monster.mobile  { continue;}

                let path = rltk::a_star_search(
                    xy_idx(pos.point.x, pos.point.y),
                    xy_idx(player_pos.x, player_pos.y),
                    &mut *map,
                );

                if path.success && path.steps.len() > 2 {
                    let next_point = map.index_to_point2d(path.steps[1]);
                    let last_point = map.point2d_to_index(pos.point);
                    map.blocked[last_point] = false;
                    pos.point.x = next_point.x;
                    pos.point.y = next_point.y;
                    map.blocked[path.steps[1]] = true;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
