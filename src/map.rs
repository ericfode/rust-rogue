use std::{cmp::{max, min}};
use proptest::prelude::*;
use crate::state::*;
use rltk::{RGB, Rltk, Rect, LineAlg, RandomNumberGenerator};


#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    CorWall,
    Floor,
    Empty,
    Obstacle,
    Tree,
    Box
}

#[derive(Clone, Debug)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub depth: i32,
}

#[derive(Clone, Copy, Debug, PartialEq )]
pub struct MapGenConfig{
    pub max_room_width: i32,
    pub max_room_height: i32,
    pub min_room_width: i32,
    pub min_room_height: i32,
    pub min_room_x: i32,
    pub min_room_y: i32,
    pub max_room_x: i32,
    pub max_room_y: i32,
}
prop_compose! {
    fn arb_map_gen_config()(min_room_width in 1..=3,
                          min_room_height in 1..=3,
                          min_room_x in 0..=50,
                          min_room_y in 0..=50)
                        ( max_room_width in min_room_width+4..=100,
                          max_room_height in min_room_height+4..=100,
                          max_room_x in min_room_x+100..=500,
                          max_room_y in min_room_y+100..=500,
                          min_room_height in Just(min_room_height),
                          min_room_width in Just(min_room_width),
                          min_room_x in Just(min_room_x),
                          min_room_y in Just(min_room_y)) -> MapGenConfig {
        MapGenConfig{
            max_room_width,
            max_room_height,
            min_room_width,
            min_room_height,
            min_room_x,
            min_room_y,
            max_room_x,
            max_room_y,
        }
    }
}


pub fn default_map_config() -> MapGenConfig{
    MapGenConfig{
        max_room_width: 10,
        max_room_height: 10,
        min_room_width: 6,
        min_room_height: 6,
        min_room_x: 1,
        min_room_y: 1,
        max_room_x: 78,
        max_room_y: 48,
    }
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map(width: usize, height: usize) -> Map {
    Map {
        tiles: vec![TileType::Empty; width*height],
        width: width.try_into().unwrap(),
        height: height.try_into().unwrap(),
        revealed_tiles: vec![false; width*height],
        visible_tiles: vec![false; width*height],
        depth: 0,
    }
}

pub fn draw_map(map: &Map, ctx: &mut Rltk){
    let mut y = 0;
    let mut x = 0;
    let (screen_width, screen_height) = ctx.get_char_size();
    for tile in map.tiles.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.0, 0.5, 0.5), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437('.'));
            }
            TileType::Empty=> {
                ctx.set(x, y, RGB::from_f32(0.0, 0.5, 0.5), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437(' '));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437('#'));
            }
            TileType::CorWall => {
                ctx.set(x, y, RGB::from_f32(1.0, 1.0, 0.0), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437('#'));
            }
            TileType::Obstacle => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437('░'));
            }
            TileType::Tree => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437('♣'));
            }
            TileType::Box => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0.0, 0.0, 0.0), rltk::to_cp437('▓'));
            }
        }
        x += 1; // Move the cursor right
        if x > screen_width-1 { // If it reaches the end of the line
            x = 0; // Move it back to the start
            y += 1; // Move it down one line
        }
    }
}

pub fn make_forest(map: &mut Map, x: i32, y: i32, width: i32, height: i32, density: i32) {
    let mut rng = rltk::RandomNumberGenerator::new();
    for i in x..x+width {
        for j in y..y+height {
            if rng.roll_dice(1, density) == 1{
                map.tiles[xy_idx(i, j)] = TileType::Tree;
            }
        }
    }
}
pub fn build_room_rect(rng: &mut RandomNumberGenerator, mgc: MapGenConfig) -> Rect {
    let w = rng.range(mgc.min_room_width, mgc.max_room_width);
    let h = rng.range(mgc.min_room_height, mgc.max_room_height);
    let x = rng.range(mgc.min_room_x, mgc.max_room_x-w);
    let y = rng.range(mgc.min_room_y, mgc.max_room_y-h);
    let new_room = Rect::with_size(x, y, w, h);
    new_room
}

proptest! {
    #[test]
    fn test_build_room_rect(mgc in arb_map_gen_config()) {
        let mut rng = rltk::RandomNumberGenerator::seeded(0);
    
        let new_room = build_room_rect(&mut rng, mgc);
        prop_assert!(new_room.x1 >= mgc.min_room_x);
        prop_assert!(new_room.x2 <= mgc.max_room_x);
        prop_assert!(new_room.y1 >= mgc.min_room_y);
        prop_assert!(new_room.y2 <= mgc.max_room_y);
        prop_assert!(new_room.width() >= mgc.min_room_width);
        prop_assert!(new_room.width() <= mgc.max_room_width);
        prop_assert!(new_room.height() >= mgc.min_room_height);
        prop_assert!(new_room.height() <= mgc.max_room_height);
    }   
}

pub fn room_fits_in_map(room: &Rect, map: &Map) -> bool {
    room.x1 > 0 && room.x2 < map.width-1 && room.y1 > 0 && room.y2 < map.height-1
}

pub fn room_does_not_overlap(room: &Rect, rooms: &Vec<Rect>) -> bool {
    for other_room in rooms.iter() {
        if room.intersect(other_room) {return false}
    }
    true
}

pub fn room_works(room: &Rect, map: &Map, rooms: &Vec<Rect>) -> bool {
    room_fits_in_map(room, map) && room_does_not_overlap(room, rooms)
}
prop_compose!{
    fn arb_rect()
    (x1 in 0..100, 
                y1 in 0..100)
                (x2 in x1+1..100, 
                 y2 in y1+1..100,
                 x1 in Just(x1),
                 y1 in Just(y1)) -> Rect {
        Rect::with_size(x1, y1, x2, y2)
    }
}



proptest!(
    #[test]
    fn test_room_works(mgc in arb_map_gen_config(), room in arb_rect()) {
        let mut rng = rltk::RandomNumberGenerator::seeded(0);
        let map = new_map(100,100);
        let mut rooms = Vec::new();
        let room = build_room_rect(&mut rng, mgc);
        prop_assert!(room_works(&room, &map, &rooms));
    }
);

pub fn generate_rooms_and_corridors() -> (Vec<Rect>, Vec<Rect>){
    let mut rng = rltk::RandomNumberGenerator::new();
    let mut rooms: Vec<Rect> = Vec::new();
    let mut corridors: Vec<Rect> = Vec::new();
    let mut previous_room = Rect::with_size(0,0,0,0);
    // Generate the rooms
    for _i in 0..10 {
        let w = rng.range(3, 10);
        let h = rng.range(3, 10);
        let x = rng.roll_dice(1, 80-w-4)-1;
        let y = rng.roll_dice(1, 50-h-4)-1;
        let new_room = Rect::with_size(x, y, w, h);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {ok = false}
        }
        if ok {
            rooms.push(new_room)
        }
    }
    // generate the corridors,sometimes connecting rooms
    for (i, room) in rooms.iter().enumerate() {
        let (new_x, new_y) = room.center().to_tuple();
        let (prev_x, prev_y) = previous_room.center().to_tuple();
        let vert_corridor = Rect::with_size(prev_x, new_y, 3, i32::abs(new_y - prev_y));
        let horiz_corridor = Rect::with_size(new_x, prev_y, i32::abs(new_x - prev_x), 3);

        let mut ok = true;
        for other_room in rooms.iter() {
            if other_room == room {continue}
            if other_room == &previous_room {continue}
            if vert_corridor.intersect(other_room) {ok = false}
            if horiz_corridor.intersect(other_room) {ok = false}
            // check for out of bounds of screen
            if vert_corridor.x1 < 0 || vert_corridor.x2 > 80 || vert_corridor.y1 < 0 || vert_corridor.y2 > 50 {ok = false}
            if horiz_corridor.x1 < 0 || horiz_corridor.x2 > 80 || horiz_corridor.y1 < 0 || horiz_corridor.y2 > 50 {ok = false}
        }
        if ok {
            corridors.push(vert_corridor);
            corridors.push(horiz_corridor);
           
        }
        previous_room = *room;
    }
    (rooms, corridors)
}

pub fn make_map_of_rooms_and_corridors(map: &mut Map, rooms: Vec<Rect>, corridors: Vec<Rect>) {
   for corridor in corridors.iter() {
        for x in corridor.x1 .. corridor.x2 {
            for y in corridor.y1 .. corridor.y2 {
                map.tiles[xy_idx(x, y)] = TileType::Floor;
            }
        }
        // build the horizantal walls 
        for x in corridor.x1 .. corridor.x2+1 {
            map.tiles[xy_idx(x, corridor.y1)] = TileType::CorWall;
            map.tiles[xy_idx(x, corridor.y2)] = TileType::CorWall;
        }
        // build the vertical walls
        for y in corridor.y1 .. corridor.y2+1 {
            map.tiles[xy_idx(corridor.x1, y)] = TileType::CorWall;
            map.tiles[xy_idx(corridor.x2, y)] = TileType::CorWall;
        }
    }
    for room in rooms.iter() {
        // Build the floors
        for x in room.x1+1 .. room.x2 {
            for y in room.y1+1 .. room.y2 {
                map.tiles[xy_idx(x, y)] = TileType::Floor;
            }
        }
        // build the horizantal walls
        for x in room.x1 .. room.x2+1 {
            map.tiles[xy_idx(x, room.y1)] = TileType::Wall;
            map.tiles[xy_idx(x, room.y2)] = TileType::Wall;
        }
        // build the vertical walls
        for y in room.y1 .. room.y2+1 {
            map.tiles[xy_idx(room.x1, y)] = TileType::Wall;
            map.tiles[xy_idx(room.x2, y)] = TileType::Wall;
        }
    }
 
}

pub fn make_dungeon(map: &mut Map) {
    let (rooms, cors) = generate_rooms_and_corridors();
    make_map_of_rooms_and_corridors(map, rooms, cors)
}