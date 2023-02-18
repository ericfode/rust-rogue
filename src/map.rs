use std::{cmp::{max, min}};
use std::collections::HashSet;
use rand::seq::{SliceRandom, IteratorRandom};
#[cfg(test)]
use proptest::{prelude::*, sample::subsequence};
use specs::{World, WorldExt, Join};
use crate::{state::*, components::{Viewshed, Player}};
use rltk::{RGB, Rltk, Rect, LineAlg, RandomNumberGenerator, Point, Algorithm2D, BaseMap};


#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall,
    CorWall,
    Floor,
    Empty,
}

#[derive(Clone, Debug)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub rooms: Vec<Rect>,
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
    pub num_rooms: usize,
    pub corridor_size: i32,
    pub room_max_connections: usize,
}

#[cfg(test)]
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
                          num_rooms in Just(10),
                          min_room_y in Just(min_room_y),
                          corridor_size in Just(3),
                          room_max_connections in Just(3)
                        ) -> MapGenConfig {
        MapGenConfig{
            max_room_width,
            max_room_height,
            min_room_width,
            min_room_height,
            min_room_x,
            min_room_y,
            max_room_x,
            max_room_y,
            num_rooms,
            corridor_size,
            room_max_connections
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
        num_rooms: 10,
        corridor_size: 3,
        room_max_connections: 2
    }
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map(width: usize, height: usize) -> Map {
    Map {
        tiles: vec![TileType::Empty; width*height],
        width: width.try_into().unwrap(),
        rooms: Vec::new(),
        height: height.try_into().unwrap(),
        revealed_tiles: vec![false; width*height],
        visible_tiles: vec![false; width*height],
        depth: 0,
    }
}

pub fn draw_map(map: &Map, ecs: &World, ctx: &mut Rltk){

    let (screen_width, _screen_height) = ctx.get_char_size();
    
        let mut y = 0;
        let mut x = 0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        // Render a tile depending upon the tile type
        if map.revealed_tiles[idx]{
            let glyph;
            let mut fg;
            match tile {
                    TileType::Floor => {
                        glyph = rltk::to_cp437('.');
                        fg = RGB::from_f32(0.0, 0.5, 0.5);
                    }
                    TileType::Empty => {
                        glyph = rltk::to_cp437(' ');
                        fg = RGB::from_f32(0.0, 0.5, 0.5);
                    }
                    TileType::Wall => {
                        glyph = rltk::to_cp437('#');
                        fg = RGB::from_f32(0.0, 1.0, 0.0);
                    }
                    TileType::CorWall => {
                        glyph = rltk::to_cp437('#');
                        fg = RGB::from_f32(1.0, 1.0, 0.0);
                    }
                }
            if !map.visible_tiles[idx] { fg = fg.to_greyscale()}
            ctx.set(x, y, fg, RGB::from_f32(0.0, 0.0, 0.0), glyph);
        }
        x += 1; // Move the cursor right
        if x > screen_width-1 { // If it reaches the end of the line
            x = 0; // Move it back to the start
            y += 1; // Move it down one line
        }
    }
}


pub fn build_room_rect(rng: &mut RandomNumberGenerator, mgc: &MapGenConfig) -> Rect {
    let w = rng.range(mgc.min_room_width, mgc.max_room_width);
    let h = rng.range(mgc.min_room_height, mgc.max_room_height);
    let x = rng.range(mgc.min_room_x, mgc.max_room_x-w);
    let y = rng.range(mgc.min_room_y, mgc.max_room_y-h);
    let new_room = Rect::with_size(x, y, w, h);
    new_room
}

#[cfg(test)]
proptest! {
    #[test]
    fn test_build_room_rect(mgc in arb_map_gen_config()) {
        let mut rng = rltk::RandomNumberGenerator::seeded(0);
    
        let new_room = build_room_rect(&mut rng, &mgc);
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

pub fn room_fits_in_map(room: &Rect, mgc: &MapGenConfig) -> bool {
    room.x1 > 0 && room.x2 < mgc.max_room_x && room.y1 > 0 && room.y2 < mgc.max_room_y
}

pub fn room_does_not_overlap(room: &Rect, rooms: &Vec<Rect>) -> bool {
    for other_room in rooms.iter() {
        if room == other_room {continue}
        if room.intersect(other_room) {return false}
    }
    true
}

pub fn room_works(room: &Rect, mgc: &MapGenConfig, rooms: &Vec<Rect>) -> bool {
    room_fits_in_map(room, mgc) && room_does_not_overlap(room, rooms)
}


pub fn generate_some_rooms(rng: &mut RandomNumberGenerator, mgc: &MapGenConfig) -> Vec<Rect> {
    let mut rooms: Vec<Rect> = vec![];
    while rooms.len() < mgc.num_rooms.try_into().unwrap() {
        let new_room = build_room_rect(rng, mgc);
        if room_works(&new_room, mgc, &rooms) {
            rooms.push(new_room);
        }
    }
    rooms
}

#[derive(Clone, Debug )]
struct RoomCase{
    rooms: Vec<Rect>,
    mgc: MapGenConfig,
}

#[cfg(test)]
fn arb_rooms(mgc: &MapGenConfig, rng: &mut rltk::RandomNumberGenerator) 
        -> impl Strategy<Value = RoomCase> {
        let rooms = subsequence( generate_some_rooms(rng, mgc),
                                                    mgc.num_rooms);

        let new_mgc = mgc.clone();
        rooms.prop_flat_map(move |rooms|{
            Just(RoomCase {rooms, mgc:new_mgc})
        } )
}

#[cfg(test)]
// Test generate_some_rooms
proptest! {
    #[test]
    fn test_generate_some_rooms(room_case in arb_rooms(&default_map_config(), &mut rltk::RandomNumberGenerator::seeded(0))) {
        prop_assert!(room_case.rooms.len() == room_case.mgc.num_rooms.try_into().unwrap());
        for room in room_case.rooms.iter() {
            prop_assert!(room_fits_in_map(room, &room_case.mgc));
            prop_assert!(room_does_not_overlap(room, &room_case.rooms));
        }
    }
}

pub fn order_points(p1: Point, p2: Point) -> (Point, Point) {
    // If p1 is above p2, p1 is the top most
    if p1.y < p2.y {
        (p1,p2)
    // if p1 is below p2, p2 is the top most
    } else if p1.y > p2.y {
        (p2,p1)
    // if p1 and p2 are on the same y, then the left most is origin
    } else {
        if p1.x < p2.x {
            (p1,p2)
        } else {
            // This will also get returned if they are equal
            (p2,p1)
        }
    }
}
#[cfg(test)]
prop_compose! {
    fn arb_point()(x in 0..100, y in 0..100) -> Point {
        Point::new(x, y)
    }
}

#[cfg(test)]
proptest! {
    #[test]
    fn test_order_points(p1 in arb_point(), p2 in arb_point()) {
        let (top_most, other) = order_points(p1, p2);
        prop_assert!(top_most==p1 && other ==p2 || top_most==p2 && other ==p1);
        prop_assert!(top_most.y <= other.y || top_most.y == other.y && top_most.x <= other.x);
        }
    }

pub fn cononical_rect(p1: Point, p2: Point) -> Rect {
   match p1.x > p2.x {
       true => Rect::with_exact(p2.x, p1.y,p1.x,p2.y),
       false => Rect::with_exact(p1.x, p1.y, p2.x, p2.y),
   }
} 

#[cfg(test)]
proptest! {
    #[test]
    fn test_conical_rect(p1 in arb_point(), p2 in arb_point()) {
        let (p1p, p2p) = order_points(p1, p2);
        let r = cononical_rect(p1p, p2p);
        prop_assert!(r.x1 <= r.x2);
        prop_assert!(r.y1 <= r.y2);
    }
}
pub fn all_edges(paths: &Rect, mpg: &MapGenConfig) -> Vec<Rect>{
    vec![Rect::with_size(paths.x1, paths.y1, paths.width()+mpg.corridor_size, mpg.corridor_size),
        Rect::with_size(paths.x2, paths.y1, mpg.corridor_size, paths.height()+mpg.corridor_size),
        Rect::with_size(paths.x1, paths.y2, paths.width()+mpg.corridor_size, mpg.corridor_size),
        Rect::with_size(paths.x1, paths.y1, mpg.corridor_size, paths.height()+mpg.corridor_size) 
    ]
}

pub fn fw_paths(path_vec: Vec<Rect>) -> Vec<Vec<Rect>>{
    vec![vec![path_vec[0], path_vec[1]], vec![path_vec[3], path_vec[2]]]
}

pub fn bw_paths(path_vec: Vec<Rect>) -> Vec<Vec<Rect>>{
    vec![vec![path_vec[1], path_vec[2]], vec![path_vec[0], path_vec[3]]]
}

pub fn connect_two_rooms(mpg: &MapGenConfig, rng: &mut RandomNumberGenerator, room1: &Rect, room2: &Rect) -> Vec<Rect>{
    let (x1, y1) = room1.center().to_tuple();
    let (x2, y2) = room2.center().to_tuple();
    let (p1, p2) = order_points(room1.center(), room2.center());
    let paths = cononical_rect(p1,p2);
    let edges = all_edges(&paths, mpg);
    // Start at the first corner of the rectangle then pick an edge to move along
    // Then build the horizontal and vertical corridors in either order
    match p1.x > p2.x {
        true => {
          bw_paths(edges)
        }
        false => {
          fw_paths(edges)
        }
    }.iter()
        .filter(|rects| rects.iter().all(|r| room_fits_in_map(r, mpg)))
        .choose(rng.get_rng())
        .unwrap_or(&vec![])
        .to_vec()
}

pub fn connect_some_rooms(mgc: &MapGenConfig, rng: &mut RandomNumberGenerator, origin: &Rect ,rooms: &Vec<Rect>) -> Vec<Rect> {
    let connects = rng.range(1,mgc.room_max_connections);
    let mut shuffled = rooms.clone();
    shuffled.shuffle(rng.get_rng());
    shuffled.iter()
        .filter(|r| *r != origin)
        .take(connects)
        .map(|target| {
           Vec::from(connect_two_rooms(mgc, rng, origin, target))
        })
        .flatten()
        .filter(|r| room_fits_in_map(r, mgc))
        .collect()
}

pub fn generate_rooms_and_corridors(mgc: &MapGenConfig, rng: &mut RandomNumberGenerator) -> (Vec<Rect>, Vec<Rect>){
    let rooms  = generate_some_rooms(rng, mgc);
    // generate the corridors,sometimes connecting rooms
    let corridors =  rooms.iter()
        .map(|room|{
            connect_some_rooms(mgc, rng, room, &rooms)
        })
        .flatten()
        .collect();
    (rooms, corridors)
}

fn add_rect_to_map(rect: &Rect, map: &mut Map, floorType: TileType, wallType: TileType) {
    rect.point_set().iter().for_each(|p|{
        // Build the floors where the point is not on and edge
        // and walls where it is by using a match
        match (p.x, p.y) {
            (x, y) if x == rect.x1 || x == rect.x2-1 || y == rect.y1 || y == rect.y2-1 => {

                map.tiles[xy_idx(p.x, p.y)] = wallType;
            },
            _ => {
                map.tiles[xy_idx(p.x, p.y)] = floorType;
            }
       }});
}

pub fn make_map_of_rooms_and_corridors(map: &mut Map, rooms: Vec<Rect>, corridors: Vec<Rect>) {
   corridors.iter().for_each(|corridor| {
        add_rect_to_map(corridor, map, TileType::Floor, TileType::Wall);
   });
   rooms.iter().for_each(|room| {
        add_rect_to_map(room, map, TileType::Floor, TileType::Wall);
   });
}

pub fn add_doors(map: &mut Map) {
    // For each row (starting one in from the edge)
    for y in 1..map.height-1 {
        // For each slot in the row (starting one in from the edge )
        for x in 1..map.width-1 {
            // If the tile is a wall
                // If the tile to the left and right are floors
                // Self, Left, Right, Up, Down
            match(map.tiles[xy_idx(x, y)],map.tiles[xy_idx(x-1, y)], map.tiles[xy_idx(x+1, y)], map.tiles[xy_idx(x, y-1)],map.tiles[xy_idx(x, y+1)]) {
                (TileType::Wall,TileType::Floor, TileType::Floor, TileType::Wall, TileType::Wall) => {
                    // Make it a door
                    map.tiles[xy_idx(x, y)] = TileType::Floor;
                }
                (TileType::Wall,TileType::Wall, TileType::Wall, TileType::Floor, TileType::Floor) => {
                    // Make it a door
                    map.tiles[xy_idx(x, y)] = TileType::Floor;
                }
                _ => {}
            }
        }
    }
}

pub fn find_starting_position(map: &mut Map) -> Point {
    let mut starting_position = Point::zero();
    for (i, tile) in map.tiles.iter().enumerate() {
        if *tile == TileType::Floor {
            // This is werid becuase of how the array is packed
            starting_position = Point::new(i as i32 % map.width, i as i32 / map.width);
            break;
        }
    }
    starting_position
}

pub fn make_dungeon(mgc: &MapGenConfig, rng: &mut RandomNumberGenerator,map: &mut Map) {
    let (rooms, cors) = generate_rooms_and_corridors(mgc,rng);
    map.rooms = rooms.clone();
    make_map_of_rooms_and_corridors(map, rooms, cors);
    add_doors(map);
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    fn point2d_to_index(&self, pt: Point) -> usize {
        let bounds = self.dimensions();
        ((pt.y * bounds.x) + pt.x)
            .try_into()
            .expect("Not a valid usize. Did something go negative?")
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        let bounds = self.dimensions();
        let w: usize = bounds
            .x
            .try_into()
            .expect("Not a valid usize. Did something go negative?");
        Point::new(idx % w, idx / w)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        let bounds = self.dimensions();
        pos.x >= 0 && pos.x < bounds.x && pos.y >= 0 && pos.y < bounds.y
    }

}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}