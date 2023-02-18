use specs::prelude::*;
use crate::components::{Viewshed, Position, Monster, Name};
use rltk::{field_of_view, Point, console};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (ReadStorage<'a, Monster>,
                       ReadStorage<'a, Viewshed>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, Name>,
                       ReadExpect<'a, Point>);
    fn run(&mut self, data: Self::SystemData) {
        let (monster, 
            viewshed, 
            pos,
            name,
            player_pos,) = data;
        for (_monster, viewshed, name, pos) in (&monster, &viewshed, &name, &pos).join() {
            if viewshed.visible_tiles.contains(&*player_pos){
                console::log(&format!("{} leers at you", name.name));
            }

        }
    }
}