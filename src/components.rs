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

#[derive(Component, Debug)]
pub struct Player {}