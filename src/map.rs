use std::{cmp::{max, min}};

use crate::state::*;
use rltk::{RGB, Rltk, Rect, LineAlg};


#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    CorWall,
    Floor,
    Empty,
    Obstacle,
    Tree,
    Box
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub depth: i32,
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

pub fn generate_rooms_and_corridors() -> (Vec<Rect>, Vec<Rect>){
    let mut rng = rltk::RandomNumberGenerator::new();
    let mut rooms: Vec<Rect> = Vec::new();
    let mut corridors: Vec<Rect> = Vec::new();
    let mut starting_position = (0,0);
    let mut previous_room = Rect::with_size(0,0,0,0);
    // Generate the rooms
    for _i in 0..10 {
        let w = rng.range(3, 10);
        let h = rng.range(3, 10);
        let x = rng.roll_dice(1, 80-w-1)-1;
        let y = rng.roll_dice(1, 50-h-1)-1;
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
        let mut min_x = previous_room.x1;
        let mut max_x = previous_room.x2;
        let mut min_y = previous_room.y1;
        let mut max_y = previous_room.y2;
        if room.x1 < min_x {min_x = room.x1}
        if room.x2 > max_x {max_x = room.x2}
        if room.y1 < min_y {min_y = room.y1}
        if room.y2 > max_y {max_y = room.y2}
        let (mut width, mut height) = (max_x-min_x, max_y-min_y);
        if width > height {
            width =  min(3, width); 
        } else {
            height =  min(3, height);
        }

        let corridor = Rect::with_size(min_x, min_y, width, height );
        // check to make sure the corridor doesn't intersect any other rooms
        let mut ok = true;
        for other_room in rooms.iter() {
            if other_room != room && other_room != &previous_room && corridor.intersect(other_room) {ok = false}
        }
   
        if ok {
            corridors.push(corridor);
        }
        previous_room = *room;
    }
    (rooms, corridors)
}

pub fn make_map_of_rooms_and_corridors(map: &mut Map, rooms: Vec<Rect>, corridors: Vec<Rect>) {
   for corridor in corridors.iter() {
        for x in corridor.x1+1 .. corridor.x2 {
            for y in corridor.y1+1 .. corridor.y2 {
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