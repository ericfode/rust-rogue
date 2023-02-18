use rltk::{RGB, Point};
use specs::prelude::*;
use specs_derive::Component;


#[derive(Component)]
pub struct Position {
    pub point : Point
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Name {
    pub name: String
}
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}
#[derive(Component, Debug)]
pub struct Monster {}
#[derive(Component, Debug)]
pub struct Player {}