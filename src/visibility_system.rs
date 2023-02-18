use specs::prelude::*;
use crate::components::{Viewshed, Position, Player};
use crate::map::{Map, xy_idx};
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>);
    fn run(&mut self, data: Self::SystemData) {
        let (mut map, 
            entities,
            player,
            mut viewshed, 
            pos) = data;

        for (ent, viewshed, pos) in (&entities,&mut viewshed, &pos).join() {
            if !viewshed.dirty { continue; }
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(pos.point, viewshed.range, &*map);
            viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

            let p : Option<&Player> = player.get(ent);
            if let Some(p) = p {
                for t in map.visible_tiles.iter_mut() { *t= false;}
                for vis in viewshed.visible_tiles.iter() {
                    let idx = xy_idx(vis.x, vis.y);
                    map.revealed_tiles[idx] = true;
                    map.visible_tiles[idx] = true;
                }
            }
            viewshed.dirty = false;
       }
    }
}