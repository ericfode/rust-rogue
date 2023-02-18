use rltk::{Rltk, GameState, RGB, VirtualKeyCode, RltkBuilder, Point};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;
use crate::visibility_system::VisibilitySystem;
use crate::monster_ai_system::MonsterAI;
use crate::components::*;
use crate::map::*;
use crate::player::{player_input};


#[derive(PartialEq, Copy, Clone)]
pub enum RunState {Paused, Running}
pub struct State {
   pub ecs: World,
   pub runstate: RunState
}


impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
           self.runstate = player_input(self, ctx);
        }


        let map = self.ecs.fetch::<Map>();
        draw_map(&map, &self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            if !map.visible_tiles[xy_idx(pos.point.x, pos.point.y)] { continue; }
            ctx.set(pos.point.x, pos.point.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut monster = MonsterAI{};
        monster.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

pub fn create_monster(state: &mut State, 
    x: i32, 
    y: i32, 
    glyph: rltk::FontCharType, 
    fg: RGB, 
    bg: RGB,
    name: String) {
    state
        .ecs 
        .create_entity()
        .with(Position {point: Point { x, y }})
        .with(Viewshed{ visible_tiles:Vec::new(), range:8, dirty: true})
        .with(Name {name})
        .with(Monster {})
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
        .with(Name { name: "Player".to_string() })
        .with(Viewshed{ visible_tiles:Vec::new(), range:8, dirty: true})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();
}
