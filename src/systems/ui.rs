use specs::{ System, RunArg, Join };

use tcod::input::{ self, KeyCode };
use ui::{ Ui, UiData };
use tcod::colors::{ self, Color };
use game_stats::{ GameStats };
use event_log::{ EventLog, LogEvent };
use components::player::{ Player, Equipment };
use components::space::{ Position, Vector, Viewport };
use components::inventory::{ Inventory };
use engine::input_handler::{ InputHandler };
use components::common::{ Active, InTurn, InTurnState, Description, Health, Damage, Range };

use maps::{ Map, Maps };

pub struct UiUpdater;
unsafe impl Sync for UiUpdater {}

impl System<()> for UiUpdater {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, players, positions, actives, in_turns, descriptions, health, equipments, damages, ranges, inventories,
             input, stats, log, viewport, mut maps, mut ui) = arg.fetch(|w| {
             (w.entities(),
              w.read::<Player>(),
              w.read::<Position>(),
              w.read::<Active>(),
              w.read::<InTurn>(),
              w.read::<Description>(),
              w.read::<Health>(),
              w.read::<Equipment>(),
              w.read::<Damage>(),
              w.read::<Range>(),
              w.read::<Inventory>(),
              w.read_resource::<InputHandler>(),
              w.read_resource::<GameStats>(),
              w.read_resource::<EventLog>(),
              w.read_resource::<Viewport>(),
              w.write_resource::<Maps>(),
              w.write_resource::<Ui>())
        });

        maps.clear_highlights();

        ui.update("time_left".into(), UiData::Text{ text: stats.time_left().to_string() });

        for (id, _, p, description, health, inventory, equipment) in (&entities, &players, &positions, &descriptions, &health, &inventories, &equipments).iter() {
            let active = actives.get(id);
            let in_turn = in_turns.get(id);

            if active.is_some() {
                // render player stats
                ui.update("active_player".into(), UiData::MultiLine { text: vec![
                    description.name.clone(),
                    health.health.to_string()
                ]});
                // render player inventory
                ui.update("inventory".into(), UiData::MultiLine {
                    text: inventory.items.iter()
                        .filter_map(|item| descriptions.get(*item))
                        .map(|description| description.name.clone())
                        .collect()
                });

                if let Some(turn) = in_turn {
                    match turn.state {
                        InTurnState::Idle => {
                            let pos_trans = viewport.inv_transform(input.mouse_pos);
                            if let Some(pos) = maps.screen_to_map(pos_trans) {
                                let dist = (Vector { x: p.x - pos.0 as f32, y: p.y - pos.1 as f32}).length();
                                if !input.ctrl {
                                    // render movement selection highlights
                                    let mut color = None;
                                    if turn.has_walked {
                                        if dist < 5.0 {
                                            color = Some(colors::LIGHT_ORANGE);
                                        }
                                    } else {
                                        if dist < 5.0 {
                                            color = Some(colors::LIGHT_GREEN);
                                        } else if dist < 10.0 {
                                            color = Some(colors::LIGHT_ORANGE);
                                        }
                                    }

                                    if viewport.visible(pos) {
                                        if let Some(c) = color {
                                            let path = maps.find_path(&id, (p.x as i32, p.y as i32), pos);
                                            maps.set_highlight_color(c);
                                            maps.add_highlights(path);
                                        }
                                    }
                                } else if dist > 0.0 {
                                    if let Some(entity) = equipment.active_item {
                                        if let Some(range) = ranges.get(entity) {
                                            let ray = maps.draw_ray((p.x as i32, p.y as i32), pos, range.range);
                                            maps.set_highlight_color(colors::LIGHT_RED);
                                            maps.add_highlights(ray);
                                        }
                                    }
                                }
                            }
                        },
                        _ => (),
                    }
                }
            }

            if active.is_none() {
                // render secondary player stats
                ui.update("inactive_player".into(), UiData::MultiLine { text: vec![
                    description.name.clone(),
                    health.health.to_string()
                ]});
            }
        }

        ui.update("event_log".into(), UiData::MultiLine {
            text: log.logs.iter()
                .map(|event| {
                    match *event {
                        LogEvent::FinishedTurn(id) => {
                            format!("{} finished turn", descriptions.get(id)
                                    .map(|d| d.name.clone())
                                    .unwrap_or("unknwon".into()))
                        }
                    }
                })
                .take(5)
                .collect::<Vec<String>>()
        });
    }
}
