use rltk::{Rltk, GameState, RGB, VirtualKeyCode, RltkBuilder, Point};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;
use crate::components::*;
use crate::map::*;
use crate::player::{player_input};

pub struct State {
   pub ecs: World
}


impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();
        let map = self.ecs.fetch::<Map>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.point.x, pos.point.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

pub fn create_actor(state: &mut State, x: i32, y: i32, glyph: rltk::FontCharType, fg: RGB, bg: RGB) {
    state
        .ecs 
        .create_entity()
        .with(Position {point: Point { x, y }})
        .with(Renderable {
            glyph,
            fg,
            bg,
        })
        .build();
}

pub fn create_player(state: &mut State, x: i32, y: i32) {
    state
        .ecs 
        .create_entity()
        .with(Player {})
        .with(Position {point: Point { x, y }})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();
}
