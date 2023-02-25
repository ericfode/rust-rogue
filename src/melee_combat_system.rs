use crate::{components::{CombatStats, Name, SufferDamage, WantsToMelee}, gamelog::GameLog};
use rltk::console;
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, SufferDamage>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, 
            c_stats, 
            mut wants_melee, 
            mut inflict_damage, 
            mut game_log,
            names) = data;

        for (_entity, stats, wants_melee, name) in (&entities, &c_stats, &wants_melee, &names).join() {
            let target_stats = c_stats.get(wants_melee.target).unwrap();
            if target_stats.hp > 0 {
                let target_name = names.get(wants_melee.target).unwrap();    
                let damage = i32::max(0, stats.power - target_stats.defense);
                if damage == 0 {
                    game_log.entries.push(format!("{} is unable to hurt {}", &name.name, &target_name.name));
                } else {
                    game_log.entries.push(
                        format!("{} attacks {} for {} hit points.",
                        &name.name, &target_name.name, damage));
                    SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
                }
            }
        }
        wants_melee.clear();
    }
}
