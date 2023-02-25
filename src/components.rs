use rltk::{RGB, Point};
use specs::prelude::*;
use specs_derive::Component;


#[derive(Component, Clone)]
pub struct Position {
    pub point : Point
}

#[derive(Component, Clone)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Clone)]
pub struct Name {
    pub name: String
}
#[derive(Component, Clone)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}
#[derive(Component, Debug)]
pub struct Monster {}
#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug, Clone)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}
#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component, Debug)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}


#[derive(Component, Debug)]
pub struct Spawner {
    rate: f32,
}


// We'll add this so we can track who spawned what
// And allow spawers to not spawn an unbounded number of thigns
// The simplest thing we can do is let the child update the parent
// to say i died. 
#[derive(Component, Debug)]
pub struct SpawnedBy {
    parent: Entity,
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage { amount: vec![amount] };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}

pub fn register_all_components(ecs:&mut World) {
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<Name>();
    ecs.register::<Viewshed>();
    ecs.register::<Monster>();
    ecs.register::<Player>();
    ecs.register::<CombatStats>();
    ecs.register::<WantsToMelee>();
    ecs.register::<SufferDamage>();
    ecs.register::<BlocksTile>();
}