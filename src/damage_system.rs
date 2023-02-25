use rltk::{console, GameState};
use specs::prelude::*;
use crate::{components::{CombatStats, SufferDamage, Player}, gamelog::GameLog, state::RunState};

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut inflict_damage) = data;

        for (stats, damage) in (&mut stats, &inflict_damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
            console::log(&format!("Damage: {}", damage.amount.iter().sum::<i32>()));
        }
        inflict_damage.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let mut dead : Vec<Entity> = Vec::new();
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let entities = ecs.entities();
        let players = ecs.read_storage::<Player>();
        let mut gamelog = ecs.fetch_mut::<GameLog>();
        let mut runstate = ecs.fetch_mut::<RunState>();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 { 
                let player = players.get(entity);
                match player {
                    None => dead.push(entity),
                    Some(_) => { gamelog.entries.push("You died!".to_string());
                    *runstate = RunState::GameOver
                }
                }
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}