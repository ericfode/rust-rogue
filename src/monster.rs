use rltk::{RandomNumberGenerator, RGB};
use rouge::FromSpec;

use crate::{
    components::{Name, Position, Renderable, CombatStats},
    map::Map,
    state::{create_monster, State, create_spawner},
};

#[derive(Clone, Debug)]
pub struct MonsterSpec {
    glyph: rltk::FontCharType,
    name: String,
    fg: RGB,
    bg: RGB,
    point: rltk::Point,
    combat_stats: Option<CombatStats>,
}

#[derive(Clone, Debug, FromSpec)]
pub struct SpawnerSpec {
    glyph: rltk::FontCharType,
    name: String,
    fg: RGB,
    bg: RGB,
    point: rltk::Point,
    combat_stats: CombatStats,
    spawn_spec: MonsterSpec,
    spawn_per: i32,
    spawn_max: i32,
}

impl From<MonsterSpec> for Position {
    fn from(val: MonsterSpec) -> Self {
        Position { point: val.point }
    }
}

impl From<MonsterSpec> for Renderable {
    fn from(val: MonsterSpec) -> Renderable {
        Renderable {
            glyph: val.glyph,
            fg: val.fg,
            bg: val.bg,
        }
    }
}

impl From<MonsterSpec> for Option<CombatStats> {
    fn from(val: MonsterSpec) -> Option<CombatStats> {
        val.combat_stats
    }
}

impl From<MonsterSpec> for Name {
    fn from(val: MonsterSpec) -> Name {
        Name { name: val.name }
    }
}

pub trait MonsterGenerator<T> {
    fn gen_one(&self, rng: &mut RandomNumberGenerator) -> T;
    fn gen_one_with_pos(&self, rng: &mut RandomNumberGenerator, x: i32, y: i32) -> T ;
}

fn default_monsters() -> Vec<MonsterSpec> {
    vec![
        MonsterSpec {
            glyph: rltk::to_cp437('r'),
            name: "Repressionist".to_string(),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            point: rltk::Point { x: 0, y: 0 },
            combat_stats: Some(CombatStats {
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 4})
        },
        MonsterSpec {
            glyph: rltk::to_cp437('o'),
            name: "Orgy Hunter".to_string(),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            point: rltk::Point { x: 0, y: 0 },
            combat_stats: Some(CombatStats {
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 4})
        },
    ]
}

fn default_spawners() -> Vec<SpawnerSpec>{
    vec![
    SpawnerSpec {
        glyph: rltk::to_cp437('P'),
        name: "Orc Spawner".to_string(),
        fg: RGB::named(rltk::RED),
        bg: RGB::named(rltk::BLACK),
        point: rltk::Point { x: 0, y: 0 },
        combat_stats: CombatStats {
            max_hp: 2,
            hp: 2,
            defense: 1,
            power: 1},
        spawn_max: 5,
        spawn_per: 1,
        spawn_spec:
        MonsterSpec {
            // I thought s was the right character, but it's not
            // instead this is a list of ideas:
            // simley face emoji (😀)
            glyph: rltk::to_cp437('p'),
            name: "Orc Spawnling".to_string(),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            point: rltk::Point { x: 0, y: 0 },
            combat_stats: Some(CombatStats {
                max_hp: 2,
                hp: 2,
                defense: 1,
                power: 1}),
        }
    },
       SpawnerSpec {
        glyph: rltk::to_cp437('C'),
        name: "Cow Spawner".to_string(),
        fg: RGB::named(rltk::RED),
        bg: RGB::named(rltk::BLACK),
        point: rltk::Point { x: 0, y: 0 },
        combat_stats: CombatStats {
            max_hp: 2,
            hp: 2,
            defense: 1,
            power: 1},
        spawn_max: 5,
        spawn_per: 1,
        spawn_spec:
        MonsterSpec {
            // I thought s was the right character, but it's not
            // instead this is a list of ideas:
            // simley face emoji ()
            glyph: rltk::to_cp437('c'),
            name: "Cow Spawnling".to_string(),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            point: rltk::Point { x: 0, y: 0 },
            combat_stats: Some(CombatStats {
                max_hp: 2,
                hp: 2,
                defense: 1,
                power: 1}),
        }
    }  
    ]
}

pub struct DefaultMonsterGenerator;

impl MonsterGenerator<MonsterSpec> for DefaultMonsterGenerator {
    fn gen_one(&self, rng: &mut RandomNumberGenerator) -> MonsterSpec {
        let len = (default_monsters().len() - 1) as i32;
        let i = rng.roll_dice(1, len);
        default_monsters()[i as usize].clone()
    }
    fn gen_one_with_pos(&self, rng: &mut RandomNumberGenerator, x: i32, y: i32) -> MonsterSpec {
        let mut spec = self.gen_one(rng);
        spec.point.x = x;
        spec.point.y = y;
        spec
    }
}

pub struct DefaultSpawnerGenerator;

impl MonsterGenerator<SpawnerSpec> for DefaultSpawnerGenerator {
    fn gen_one(&self, rng: &mut RandomNumberGenerator) -> SpawnerSpec{
        let len = (default_spawners().len() - 1) as i32;
        let i = rng.roll_dice(1, len);
        default_spawners()[i as usize].clone()
    }
    fn gen_one_with_pos(&self, rng: &mut RandomNumberGenerator, x: i32, y: i32) -> SpawnerSpec {
        let mut spec = self.gen_one(rng);
        spec.point.x = x;
        spec.point.y = y;
        spec
    }
}

pub fn generate_monsters(gs: &mut State, rng: &mut RandomNumberGenerator, map: &Map) {
    let gen = DefaultMonsterGenerator;
    let spawn_gen = DefaultSpawnerGenerator;

    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center().to_tuple();
        create_monster(gs, gen.gen_one_with_pos(rng, x, y));
        create_spawner(gs, spawn_gen.gen_one_with_pos(rng, x-1, y-1));
        // one out of 5 rooms add a spawner
        // if rng.roll_dice(1, 5) == 1 {
        //     // All rooms are bigger thin 3x3 so this will be fine :P
        //     spawn_gen.gen_one_with_pos(rng, x-1, y-1);
        // }
    }
}
