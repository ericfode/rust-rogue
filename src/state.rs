use crate::components::*;
use crate::damage_system::delete_the_dead;
use crate::damage_system::DamageSystem;
use crate::gui::draw_ui;
use crate::map::*;
use crate::map_index_system::MapIndexingSystem;
use crate::melee_combat_system::MeleeCombatSystem;
use crate::monster::MonsterSpec;
use crate::monster::SpawnerSpec;
use crate::monster_ai_system::MonsterAI;
use crate::player::player_input;
use crate::visibility_system::VisibilitySystem;
use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    PreRun,
    // In awaiting input the player only declares their intent
    AwaitingInput,
    // PlayerTurn is where the changes actually propogate into the game
    PlayerTurn,
    MonsterTurn,
    // A hack, to allow me to show the whole map easily. Probably should make
    // a debug mode instead.
    RevealMap,
    GameOver,
}
pub struct State {
    pub ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }
        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx); 
            }
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::RevealMap => {
                let mut map = self.ecs.write_resource::<Map>();
                map.reveal_map();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate= RunState::AwaitingInput;
            },
            RunState::GameOver => {
                // Do nothing the game is over for the moment

            },
        }
        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }


        delete_the_dead(&mut self.ecs);
        let map = self.ecs.fetch::<Map>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            if !map.visible_tiles[xy_idx(pos.point.x, pos.point.y)] {
                continue;
            }
            ctx.set(pos.point.x, pos.point.y, render.fg, render.bg, render.glyph);
        }
        draw_ui(&self.ecs, ctx);
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut monster = MonsterAI {};
        monster.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);
        let mut melee_combat_system = MeleeCombatSystem {};
        melee_combat_system.run_now(&self.ecs);
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

pub fn create_spawner(state: &mut State, ss:SpawnerSpec) {
    state
        .ecs
        .create_entity()
        .with(BlocksTile {})
        .with(Position::from(ss.clone()))
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name::from(ss.clone()))
        .with(Monster {mobile: false})
        .maybe_with(Option::<CombatStats>::from(ss.clone()))
        .with(Renderable::from(ss.clone()))
        .build();
}

pub fn create_monster(state: &mut State, ms: MonsterSpec) {
    state
        .ecs
        .create_entity()
        .with(BlocksTile {})
        .with(Position::from(ms.clone()))
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name::from(ms.clone()))
        .with(Monster {mobile: true})
        .maybe_with(Option::<CombatStats>::from(ms.clone()))
        .with(Renderable::from(ms.clone()))
        .build();
}

pub fn create_player(state: &mut State, x: i32, y: i32) {
    let player = state
        .ecs
        .create_entity()
        .with(BlocksTile {})
        .with(Player {})
        .with(Position {
            point: Point { x, y },
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();
    state.ecs.insert(player)
}
