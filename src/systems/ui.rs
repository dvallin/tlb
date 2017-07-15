use specs::{ System, ReadStorage, Fetch, FetchMut, Entities, Join };

use ui::{ Ui, UiData };
use tcod::colors::{ self, Color };
use game_stats::{ GameStats };
use event_log::{ EventLog, LogEvent };
use components::player::{ Player, Equipment };
use components::space::{ Position, Level, Viewport };
use components::inventory::{ Inventory };
use engine::input_handler::{ InputHandler };
use components::common::{ Active, InTurn, InTurnState, Description, CharacterStats, ItemStats };

use tower::{ Tower };

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

#[derive(SystemData)]
pub struct UiUpdaterData<'a> {
    entities: Entities<'a>,
    players: ReadStorage<'a, Player>,
    positions: ReadStorage<'a, Position>,
    levels: ReadStorage<'a, Level>,
    actives: ReadStorage<'a, Active>,
    in_turns: ReadStorage<'a, InTurn>,
    descriptions: ReadStorage<'a, Description>,
    equipments: ReadStorage<'a, Equipment>,
    item_stats: ReadStorage<'a, ItemStats>,
    char_stats: ReadStorage<'a, CharacterStats>,
    inventories: ReadStorage<'a, Inventory>,
    input: Fetch<'a, InputHandler>,
    stats: Fetch<'a, GameStats>,
    viewport: Fetch<'a, Viewport>,
    tower: FetchMut<'a, Tower>,
    ui: FetchMut<'a, Ui>,
    log: Fetch<'a, EventLog>,
}

impl<'a> System<'a> for UiUpdater {
    type SystemData = UiUpdaterData<'a>;

    fn run(&mut self, mut data: UiUpdaterData) {
        data.tower.clear_highlights();

        data.ui.update("time_left".into(), UiData::Text{ text: data.stats.time_left().to_string() });

        for (id, _, p, level, description, stats, inventory, equipment) in (&*data.entities, &data.players, &data.positions, &data.levels, &data.descriptions, &data.char_stats, &data.inventories, &data.equipments).join() {
            let active = data.actives.get(id);
            let in_turn = data.in_turns.get(id);

            if active.is_some() {
                // render player stats
                data.ui.update("active_player".into(), UiData::MultiLine { text: vec![
                    description.name.clone(),
                    stats.health.to_string()
                ]});

                // render player inventory
                let inventory_text = inventory.items.iter()
                    .filter_map(|item| data.descriptions.get(*item))
                    .map(|description| description.name.clone())
                    .collect();
                data.ui.update("inventory".into(), UiData::MultiLine { text: inventory_text });

                let mut highlights = None;
                {
                    let maps = data.tower.get(level).unwrap();
                    if let Some(turn) = in_turn {
                        match turn.state {
                            InTurnState::Idle => {
                                let pos_trans = data.viewport.inv_transform(data.input.mouse_pos);
                                if let Some(pos) = maps.screen_to_map(pos_trans) {
                                    if !data.input.ctrl {
                                        // render movement selection highlights
                                        if data.viewport.visible(pos) {
                                            let path = maps.find_path(&id, (p.x as i32, p.y as i32), pos);
                                            let color = distance_color(path.len(), &turn);
                                            if let Some(c) = color {
                                                highlights = Some((c, path))
                                            }
                                        }
                                    } else {
                                        if let Some(entity) = equipment.active_item {
                                            if let Some(item_stat) = data.item_stats.get(entity) {
                                                let ray = maps.draw_ray((p.x as i32, p.y as i32), pos, item_stat.range);
                                                highlights = Some((colors::LIGHT_RED, ray));
                                            }
                                        }
                                    }
                                }
                            },
                            _ => (),
                        }
                    }
                }
                if let Some((color, path)) = highlights {
                    data.tower.set_highlight_color(color);
                    data.tower.add_highlights(path);
                }
            }

            if active.is_none() {
                // render secondary player stats
                data.ui.update("inactive_player".into(), UiData::MultiLine { text: vec![
                    description.name.clone(),
                    stats.health.to_string()
                ]});
            }
        }


        let log_text = data.log.logs.iter()
                .map(|event| {
                    match *event {
                        LogEvent::FinishedTurn(id) => {
                            format!("{} finished turn", data.descriptions.get(id)
                                    .map(|d| d.name.clone())
                                    .unwrap_or("unknwon".into()))
                        }
                        LogEvent::Died(id) => {
                            format!("{} died!", data.descriptions.get(id)
                                    .map(|d| d.name.clone())
                                    .unwrap_or("unknwon".into()))
                        }
                        LogEvent::DidDamage(source, _target, damage) => {
                            format!("{} did {} dmg", data.descriptions.get(source)
                                    .map(|d| d.name.clone())
                                    .unwrap_or("unknwon".into()),
                                    damage,
                            )
                        }
                    }
                })
                .take(5)
                .collect::<Vec<String>>();
        data.ui.update("event_log".into(), UiData::MultiLine { text: log_text });
    }
}
