
use rltk::{Rect, RandomNumberGenerator};
use interval::ops::Range;
use interval::interval_set::*;

use petgraph::graph::{Graph, NodeIndex, Node};
use petgraph::visit::{EdgeRef};
use petgraph::{Direction};

use crate::map::*;

/*
The problem is to generate a map with a given number of rooms and corridors.
The map is a grid of cells, each cell can be a wall or a floor, empty.


A room is a rectangle of cells, all of which are floors, surrounded by walls

A room never overlaps another room

A corridor is a line of cells, all of which are floors, surrounded by walls connecting two rooms

Corridors can intersect

The default value for a cell is empty

the map is the union of all rooms and corridors and the default value for the rest of the cells
 */

/*
 Using a constraint solver might have been the wrong approach for this problem.
 there are to many valid answers
 ... Maybe
 also i can't find a good one for rust

 So instead i will try a construcive approach
 
 We will determine the number of rooms on the map
 
 Then we will connect each room to some number of other rooms. 
 Call this "Connectedness" which is a number between 0 and 1
 the ratio of connected rooms to total rooms
 
 these will form the edges of the graph


then we will elaborate the graph by adding nodes for halls and corners
and connecting them to the rooms

every room node will have a hall, corner, hall nodes between the room and the next room

Then we generate a spanning tree of the graph

then using the spanning tree validate that it's possible to generate this map the bounds of the map

then we elaborate the graph by adding noodes for each cell in the defined map

then generate the map using the graph
*/


#[derive(PartialEq, Copy, Clone, Debug)]
pub enum AbstractMapNodeP1 {
    Room,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum AbstarctMapNodeP2 {
    Hall(Rect),
    Room(Rect)
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum AbstarctMapNodeP3 {
    Hall(Rect, Rect),
    Room(Rect, Rect),
}

fn MapPhase1(rooms: i32, room_con: i32 , rng: RandomNumberGenerator ,mpg: MapGenConfig) -> Graph<AbstractMapNodeP1, i32> {
    let mut g = Graph::<AbstractMapNodeP1, i32>::new();
    let mut rooms_vec = Vec::<NodeIndex<u32>>::new();
    for i in 1..rooms{
        rooms_vec.push(g.add_node(AbstractMapNodeP1::Room));
    }
    for origin in rooms_vec{
        for i in 1..room_con {
            let target = rng.random_slice_entry(&rooms_vec);
            match target {
                Some(target) if *target != origin => Some(g.add_edge(origin, *target, 1)),
                Some(target) => None,
                None => None
            };
        }
    }
    g
}

fn MapPhase2(g: Graph<AbstractMapNodeP1, i32>, rng: RandomNumberGenerator, mpg: MapGenConfig) -> Graph<AbstarctMapNodeP2, i32> {
    let mut g2 = Graph::<AbstarctMapNodeP2, i32>::new();
    let mut rooms_vec = Vec::<NodeIndex<u32>>::new();
    let mut halls_vec = Vec::<NodeIndex<u32>>::new();
    for node in g.node_indices(){
        match g.node_weight(node){
            Some(AbstractMapNodeP1::Room) => {
                let room = Rect::with_size(rng.roll_dice(1, mpg.max_room_width- 2) as i32, 
                                                rng.roll_dice(1, mpg.max_room_height- 2) as i32, 
                                                rng.roll_dice(1, 5) as i32, 
                                                rng.roll_dice(1, 5) as i32);
                rooms_vec.push(g2.add_node(AbstarctMapNodeP2::Room(room)));
            }
            None => {}
        }
    }
    for origin in rooms_vec{
        for edge in g.edges(origin){
            let target = edge.target();
            match g.node_weight(target){
                Some(AbstractMapNodeP1::Room) => {
                    let hall = Rect::with_size(rng.roll_dice(1, mpg.width - 2) as i32, rng.roll_dice(1, mpg.height - 2) as i32, rng.roll_dice(1, 5) as i32, rng.roll_dice(1, 5) as i32);
                    halls_vec.push(g2.add_node(AbstarctMapNodeP2::Hall(hall)));
                }
                None => {}
            }
        }
    }
    g2
}




