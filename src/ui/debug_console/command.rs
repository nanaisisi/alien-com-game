use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use super::types::DebugConsoleState;
use crate::camera::MapCamera;
use crate::faction::FactionResources;
use crate::map::hex::HexCoord;
use crate::map::{MapGrid, HEX_RADIUS};

#[derive(SystemParam)]
pub struct CommandContext<'w, 's> {
    pub resources: ResMut<'w, FactionResources>,
    pub map_grid: Res<'w, MapGrid>,
    pub camera_query: Query<'w, 's, &'static mut MapCamera>,
}

pub fn execute_command(
    raw_cmd: &str,
    state: &mut DebugConsoleState,
    ctx: &mut CommandContext,
) {
    let trimmed = raw_cmd.trim();
    if trimmed.is_empty() {
        return;
    }

    // 実行コマンドをログにエコー
    state.add_log(format!("> {}", trimmed), Color::srgb(0.9, 0.9, 0.9));

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let cmd = parts[0].to_lowercase();
    let args = &parts[1..];

    match cmd.as_str() {
        "help" => {
            state.add_log("=== AVAILABLE COMMANDS ===", Color::srgb(0.25, 0.85, 0.75));
            state.add_log("  help                  : Show this help message", Color::WHITE);
            state.add_log("  clear                 : Clear console log", Color::WHITE);
            state.add_log("  turn <count>          : Advance turns (default 1)", Color::WHITE);
            state.add_log("  energy <amount>       : Add/subtract energy", Color::WHITE);
            state.add_log("  prod <amount>         : Add/subtract production", Color::WHITE);
            state.add_log("  sci <amount>          : Add/subtract science", Color::WHITE);
            state.add_log("  food <amount>         : Add/subtract food", Color::WHITE);
            state.add_log("  res <e> <p> <s> <f>   : Set all 4 resources at once", Color::WHITE);
            state.add_log("  goto <q> <r>          : Jump camera to HexCoord (q, r)", Color::WHITE);
            state.add_log("  cam <q> <r>           : Alias for goto", Color::WHITE);
        }
        "clear" | "cls" => {
            state.logs.clear();
        }
        "turn" => {
            let count: u32 = if let Some(first) = args.first() {
                match first.parse() {
                    Ok(val) => val,
                    Err(_) => {
                        state.add_log("Usage: turn <number>", Color::srgb(0.9, 0.3, 0.3));
                        return;
                    }
                }
            } else {
                1
            };

            for _ in 0..count {
                ctx.resources.turn += 1;
                ctx.resources.energy += ctx.resources.energy_per_turn;
                ctx.resources.production += ctx.resources.production_per_turn;
                ctx.resources.science += ctx.resources.science_per_turn;
                ctx.resources.food += ctx.resources.food_per_turn;
            }
            state.add_log(
                format!("Advanced {} turns. Current turn: {}", count, ctx.resources.turn),
                Color::srgb(0.3, 0.9, 0.4),
            );
        }
        "energy" => {
            if let Some(arg) = args.first() {
                if let Ok(val) = arg.parse::<i32>() {
                    ctx.resources.energy += val;
                    state.add_log(
                        format!("Energy changed by {}. Current: {}", val, ctx.resources.energy),
                        Color::srgb(0.3, 0.9, 0.4),
                    );
                } else {
                    state.add_log("Invalid amount. Usage: energy <amount>", Color::srgb(0.9, 0.3, 0.3));
                }
            } else {
                state.add_log("Usage: energy <amount>", Color::srgb(0.9, 0.3, 0.3));
            }
        }
        "prod" | "production" => {
            if let Some(arg) = args.first() {
                if let Ok(val) = arg.parse::<i32>() {
                    ctx.resources.production += val;
                    state.add_log(
                        format!("Production changed by {}. Current: {}", val, ctx.resources.production),
                        Color::srgb(0.3, 0.9, 0.4),
                    );
                } else {
                    state.add_log("Invalid amount. Usage: prod <amount>", Color::srgb(0.9, 0.3, 0.3));
                }
            } else {
                state.add_log("Usage: prod <amount>", Color::srgb(0.9, 0.3, 0.3));
            }
        }
        "sci" | "science" => {
            if let Some(arg) = args.first() {
                if let Ok(val) = arg.parse::<i32>() {
                    ctx.resources.science += val;
                    state.add_log(
                        format!("Science changed by {}. Current: {}", val, ctx.resources.science),
                        Color::srgb(0.3, 0.9, 0.4),
                    );
                } else {
                    state.add_log("Invalid amount. Usage: sci <amount>", Color::srgb(0.9, 0.3, 0.3));
                }
            } else {
                state.add_log("Usage: sci <amount>", Color::srgb(0.9, 0.3, 0.3));
            }
        }
        "food" => {
            if let Some(arg) = args.first() {
                if let Ok(val) = arg.parse::<i32>() {
                    ctx.resources.food += val;
                    state.add_log(
                        format!("Food changed by {}. Current: {}", val, ctx.resources.food),
                        Color::srgb(0.3, 0.9, 0.4),
                    );
                } else {
                    state.add_log("Invalid amount. Usage: food <amount>", Color::srgb(0.9, 0.3, 0.3));
                }
            } else {
                state.add_log("Usage: food <amount>", Color::srgb(0.9, 0.3, 0.3));
            }
        }
        "res" | "resources" => {
            if args.len() >= 4 {
                let e = args[0].parse::<i32>();
                let p = args[1].parse::<i32>();
                let s = args[2].parse::<i32>();
                let f = args[3].parse::<i32>();

                if let (Ok(e), Ok(p), Ok(s), Ok(f)) = (e, p, s, f) {
                    ctx.resources.energy += e;
                    ctx.resources.production += p;
                    ctx.resources.science += s;
                    ctx.resources.food += f;
                    state.add_log(
                        format!(
                            "Resources added: E:{:+}, P:{:+}, S:{:+}, F:{:+}",
                            e, p, s, f
                        ),
                        Color::srgb(0.3, 0.9, 0.4),
                    );
                } else {
                    state.add_log("Usage: res <energy> <production> <science> <food>", Color::srgb(0.9, 0.3, 0.3));
                }
            } else {
                state.add_log("Usage: res <energy> <production> <science> <food>", Color::srgb(0.9, 0.3, 0.3));
            }
        }
        "goto" | "cam" => {
            if args.len() >= 2 {
                let q_res = args[0].parse::<i32>();
                let r_res = args[1].parse::<i32>();
                if let (Ok(q), Ok(r)) = (q_res, r_res) {
                    let target_coord = HexCoord::new(q, r);
                    let world_pos = target_coord.to_world_pos(HEX_RADIUS);

                    for mut map_cam in &mut ctx.camera_query {
                        map_cam.target_focal_point = Vec3::new(world_pos.x, 0.0, world_pos.z);
                    }

                    let map_w = if ctx.map_grid.width > 0 {
                        ctx.map_grid.width
                    } else {
                        crate::map::GRID_WIDTH
                    };
                    let (col, row) = target_coord.to_col_row_with_width(map_w);

                    state.add_log(
                        format!(
                            "Camera focused on Hex (q:{}, r:{}) [col:{}, row:{}] at ({:.1}, {:.1})",
                            q, r, col, row, world_pos.x, world_pos.z
                        ),
                        Color::srgb(0.3, 0.9, 0.4),
                    );
                } else {
                    state.add_log("Usage: goto <q> <r>", Color::srgb(0.9, 0.3, 0.3));
                }
            } else {
                state.add_log("Usage: goto <q> <r>", Color::srgb(0.9, 0.3, 0.3));
            }
        }
        unknown => {
            state.add_log(
                format!("Unknown command: '{}'. Type 'help' for command list.", unknown),
                Color::srgb(0.9, 0.3, 0.3),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn test_debug_console_state_logs() {
        let mut state = DebugConsoleState::default();
        let initial_count = state.logs.len();
        state.add_log("Test message", Color::WHITE);
        assert_eq!(state.logs.len(), initial_count + 1);
        assert_eq!(state.logs.last().unwrap().0, "Test message");
    }

    #[test]
    fn test_debug_console_command_parsing() {
        let mut app = App::new();
        app.init_resource::<FactionResources>()
            .init_resource::<MapGrid>();

        app.world_mut().run_system_once(|
            mut ctx: CommandContext,
        | {
            let mut state = DebugConsoleState::default();
            execute_command("help", &mut state, &mut ctx);
            assert!(state.logs.iter().any(|(msg, _)| msg.contains("AVAILABLE COMMANDS")));

            // energy コマンドのテスト
            let initial_energy = ctx.resources.energy;
            execute_command("energy 500", &mut state, &mut ctx);
            assert_eq!(ctx.resources.energy, initial_energy + 500);

            // turn コマンドのテスト
            let initial_turn = ctx.resources.turn;
            execute_command("turn 3", &mut state, &mut ctx);
            assert_eq!(ctx.resources.turn, initial_turn + 3);
        }).unwrap();
    }
}

