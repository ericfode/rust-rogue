use rltk::Algorithm2D;
use specs::prelude::*;
use crate::components::{Position,  BlocksTile};
use crate::map::{Map };


pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, positions, blockers) = data;

        map.populate_blocked();
        map.clear_content_index();

        for (entity, pos) in (&entities, &positions).join() {
            let idx = map.point2d_to_index(pos.point);

            // if the tile is blocked, set the map tile to blocked
            let _p : Option<&BlocksTile> = blockers.get(entity);
            if let Some(_p) = _p {
                map.blocked[idx] = true;
            }

            // add the entity to the map tile content
            map.tile_content[idx].push(entity);
        }
    }

}