use specs::{ System, RunArg, Join };

use ui::{ Ui, UiData };
use tcod::colors::{ self, Color };
use game_stats::{ GameStats };
use event_log::{ EventLog, LogEvent };
use components::player::{ Player, Equipment };
use components::space::{ Position, Vector, Viewport };
use components::inventory::{ Inventory };
use engine::input_handler::{ InputHandler };
use components::common::{ Active, InTurn, InTurnState, Description, CharacterStats, ItemStats };

use maps::{ Maps };

pub struct UiUpdater;
unsafe impl Sync for UiUpdater {}

fn distance_color(dist: usize, turn: &InTurn) -> Option<Color> {
    if turn.has_walked {
        if dist < 5 {
            return Some(colors::LIGHT_ORANGE);
        }
    } else {
        if dist < 5 {
            return Some(colors::LIGHT_GREEN);
        } else if dist < 10 {
            return Some(colors::LIGHT_ORANGE);
        }
    }
    None
}

impl System<()> for UiUpdater {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, players, positions, actives, in_turns, descriptions,
             equipments, item_stats, char_stats, inventories,
             input, stats, log, viewport, mut maps, mut ui) = arg.fetch(|w| {
             (w.entities(),
              w.read::<Player>(),
              w.read::<Position>(),
              w.read::<Active>(),
              w.read::<InTurn>(),
              w.read::<Description>(),
              w.read::<Equipment>(),
              w.read::<ItemStats>(),
              w.read::<CharacterStats>(),
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

        for (id, _, p, description, stats, inventory, equipment) in (&entities, &players, &positions, &descriptions, &char_stats, &inventories, &equipments).iter() {
            let active = actives.get(id);
            let in_turn = in_turns.get(id);

            if active.is_some() {
                // render player stats
                ui.update("active_player".into(), UiData::MultiLine { text: vec![
                    description.name.clone(),
                    stats.health.to_string()
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
                                if !input.ctrl {
                                    // render movement selection highlights
                                    if viewport.visible(pos) {
                                        let path = maps.find_path(&id, (p.x as i32, p.y as i32), pos);
                                        let color = distance_color(path.len(), &turn);

                                        if let Some(c) = color {
                                            maps.set_highlight_color(c);
                                            maps.add_highlights(path);
                                        }
                                    }
                                } else {
                                    if let Some(entity) = equipment.active_item {
                                        if let Some(item_stat) = item_stats.get(entity) {
                                            let ray = maps.draw_ray((p.x as i32, p.y as i32), pos, item_stat.range);
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
                    stats.health.to_string()
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
                        LogEvent::Died(id) => {
                            format!("{} died!", descriptions.get(id)
                                    .map(|d| d.name.clone())
                                    .unwrap_or("unknwon".into()))
                        }
                        LogEvent::DidDamage(source, _target, damage) => {
                            format!("{} did {} dmg",
                                    descriptions.get(source)
                                    .map(|d| d.name.clone())
                                    .unwrap_or("unknwon".into()),
                                    damage,
                            )
                        }
                    }
                })
                .take(5)
                .collect::<Vec<String>>()
        });
    }
}
