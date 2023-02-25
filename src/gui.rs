use rltk::{Console, Point, Rltk, RGB};
use specs::prelude::*;

use crate::{components::{CombatStats, Player}, gamelog::GameLog};

pub struct UiConfig {
    pub fg: RGB,
    pub bg: RGB,
    pub bounds: Point,
    pub ui_origin: Point,
    pub ui_size: Point,
}

pub fn default_config() -> UiConfig {
    UiConfig{
        fg: RGB::named(rltk::WHITE),
        bg: RGB::named(rltk::BLACK),
        bounds: Point::new(79, 49),
        ui_origin: Point::new(0, 43),
        ui_size: Point::new(79, 6),
    }
}

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    let default = default_config();
    ctx.draw_box(
        default.ui_origin.x,
        default.ui_origin.y,
        default.ui_size.x,
        default.ui_size.y,
        default.bg,
        default.fg,
    );

    let combat_stats = ecs.read_storage::<CombatStats>();
    let player = ecs.read_storage::<Player>();
    let game_log = ecs.fetch::<GameLog>();

    draw_player(player, combat_stats, ctx, &default);
    draw_log(game_log, ctx, &default);

    let mouse_pos = ctx.mouse_pos();

    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::MAGENTA));
}

fn draw_log(game_log: specs::shred::Fetch<GameLog>, ctx: &mut Rltk, default: &UiConfig) {
    let mut y = default.ui_origin.y + 2;
    for s in game_log.entries.iter().rev() {
        if y < default.ui_origin.y + default.ui_size.y {
            ctx.print_color(
                default.ui_origin.x + 2,
                y,
                default.fg,
                default.bg,
                s.to_string(),
            );
        }
        y += 1;
    }
}

fn draw_player(player: Storage<Player, specs::shred::Fetch<specs::storage::MaskedStorage<Player>>>, combat_stats: Storage<CombatStats, specs::shred::Fetch<specs::storage::MaskedStorage<CombatStats>>>, ctx: &mut Rltk, default: &UiConfig) {
    for (_player, stats) in (&player, &combat_stats).join() {
        ctx.print_color(
            default.ui_origin.x + 12,
            default.ui_origin.y ,
            default.fg,
            default.bg,
            format!("HP: {} / {}", stats.hp, stats.max_hp),
        );

        ctx.draw_bar_horizontal(default.ui_origin.x+28, 
            default.ui_origin.y, 
            51,
            stats.hp, 
            stats.max_hp, 
            RGB::named(rltk::RED), 
            RGB::named(rltk::BLACK))
    }
}
