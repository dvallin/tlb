use std::collections::VecDeque;
use specs::{ System, ReadStorage, Fetch, FetchMut, Entities, WriteStorage, Join };

use game_state::{ GameState };

use geometry::{ Rect };

use components::space::{ Position, Level, Vector, Viewport, mul };
use components::player::{ Player, Equipment };
use components::common::{ Active, InTurn, MoveToPosition, CharacterStats, ItemStats };
use components::inventory::{ Inventory };
use components::interaction::{ Interactable, Interaction };
use engine::input_handler::{ InputHandler };
use engine::time::{ Time };

use event_log::{ EventLog, LogEvent };
use tower::{ Tower };
use maps::{ Map };

pub struct PlayerController;
unsafe impl Sync for PlayerController {}

fn get_delta(input: &InputHandler) -> Vector {
    // move players
    let mut delta = Vector { x: 0.0, y: 0.0 };
    if input.is_char_down('h') {
        delta.x -= 1.0;
    }
    if input.is_char_down('j') {
        delta.y += 1.0;
    }
    if input.is_char_down('k') {
        delta.y -= 1.0;
    }
    if input.is_char_down('l') {
        delta.x += 1.0;
    }
    delta
}

fn distance_cost(dist: usize, turn: &InTurn) -> Option<i32> {
    if dist < 5 {
        return Some(1);
    } else if dist < 10 && !turn.has_walked {
        return Some(2);
    }
    None
}

#[derive(SystemData)]
pub struct PlayerControllerData<'a> {
    entities: Entities<'a>,
    players: ReadStorage<'a, Player>,
    actives: ReadStorage<'a, Active>,
    levels: ReadStorage<'a, Level>,
    positions: WriteStorage<'a, Position>,
    interactions: WriteStorage<'a, Interaction>,
    interactables: WriteStorage<'a, Interactable>,
    inventories: WriteStorage<'a, Inventory>,
    move_to_positions: WriteStorage<'a, MoveToPosition>,
    equipments: WriteStorage<'a, Equipment>,
    char_stats: WriteStorage<'a, CharacterStats>,
    item_stats: WriteStorage<'a, ItemStats>,
    in_turns: WriteStorage<'a, InTurn>,
    time: Fetch<'a, Time>,
    state: Fetch<'a, GameState>,
    input: Fetch<'a, InputHandler>,
    log: FetchMut<'a, EventLog>,
    tower: FetchMut<'a, Tower>,
    viewport: Fetch<'a, Viewport>,
}

const PLAYER_SPEED: f32 = 4.0;
impl<'a> System<'a> for PlayerController {
    type SystemData = PlayerControllerData<'a>;

    fn run(&mut self, mut data: PlayerControllerData) {
        let delta_time = data.time.delta_time.subsec_nanos() as f32 / 1.0e9;

        if data.state.is_turn_based {
            if let Some ((id, p, _, _, turn, equipment, level)) = (&* data.entities, &data.positions, &data.actives, &data.players, &mut data.in_turns, &data.equipments, &data.levels).join().next() {
                if data.input.is_mouse_pressed() {
                    let pos_trans = data.viewport.inv_transform(data.input.mouse_pos);
                    let maps = data.tower.get(level).unwrap();
                    if let Some(pos) = maps.screen_to_map(pos_trans) {
                        if data.viewport.visible(pos_trans) {
                            if !data.input.ctrl {
                                let path = maps.find_path(&id, (p.x as i32, p.y as i32), pos);
                                let cost = distance_cost(path.len(), &turn);
                                if let Some(c) = cost {
                                    data.move_to_positions.insert(id, MoveToPosition { path: path, speed: PLAYER_SPEED });
                                    turn.walk(c);
                                }
                            } else {
                                if let Some(entity) = equipment.active_item {
                                    if let Some(item_stat) = data.item_stats.get(entity) {
                                        let characters = maps.collect_characters_with_ray((p.x as i32, p.y as i32), pos, item_stat.range);
                                        if let Some(target) = characters.front() {
                                            if let Some(character_stat) = data.char_stats.get_mut(*target) {
                                                let damage = character_stat.apply_damage(item_stat);
                                                data.log.log(LogEvent::DidDamage(id, *target, damage));
                                                turn.fight();
                                                turn.action_done();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if let Some((id, p, level, _, _)) = (&*data.entities, &data.positions, &data.levels, &data.players, &data.actives).join().next() {
                let maps = data.tower.get(level).unwrap();
                if data.input.is_mouse_pressed() {
                    let pos_trans = data.viewport.inv_transform(data.input.mouse_pos);
                    if let Some(pos) = maps.screen_to_map(pos_trans) {
                        if data.viewport.visible(pos_trans) {
                            // set the position to the middle of the cell to avoid twitching.
                            let path = maps.find_path(&id, (p.x as i32, p.y as i32), pos);
                            data.move_to_positions.insert(id, MoveToPosition { path: path,
                                                                          speed: PLAYER_SPEED });
                        }
                    }
                } else {
                    let delta = get_delta(&data.input);
                    if delta.x != 0.0 || delta.y != 0.0 {
                        let np = *p + mul(delta.norm(), delta_time*PLAYER_SPEED);
                        let mut path = VecDeque::new();
                        path.push_back(np);
                        data.move_to_positions.insert(id, MoveToPosition { path: path,
                                                                      speed: PLAYER_SPEED });
                    }

                    if data.input.is_char_pressed('e') {
                        let pos = (p.x as i32, p.y as i32);
                        let targets = maps.collect_characters_with_shape(
                            Rect::new(pos.0 - 1, pos.1 - 1, 3, 3));

                        let first_interactable_id = targets.into_iter()
                            .filter(|i| data.interactables.get(*i).is_some())
                            .next();
                        if let Some(target_id) = first_interactable_id {
                            data.interactions.insert(target_id, Interaction { actor: id });
                        }
                    }
                }
            }
        }
        if let Some((id, inventory, equipment, level, _, _)) = (&*data.entities, &mut data.inventories, &mut data.equipments, &data.levels, &data.players, &data.actives).join().next() {
            let p = data.positions.get(id).unwrap().clone();
            let maps = data.tower.get_mut(level).unwrap();
            // player interaction
            if data.input.is_char_pressed('p') {
                if let Some(entry) = maps.pop(Map::Item, (p.x as i32, p.y as i32)) {
                    inventory.push(entry.0);
                    data.positions.remove(entry.0);
                }
            } else if data.input.is_char_pressed('d') {
                if let Some(item_id) = inventory.pop() {
                    maps.push(Map::Item, &item_id, (p.x as i32, p.y as i32));
                    maps.set_blocking(Map::Item, &item_id, (p.x as i32, p.y as i32), false);
                    maps.set_sight_blocking(Map::Item, &item_id, (p.x as i32, p.y as i32), false);
                    data.positions.insert(item_id, p);
                }
            } else if let Some(digit) = data.input.pressed_digit {
                if let Some(item) = inventory.get(((digit + 9) % 10) as usize) {
                    equipment.active_item = Some(*item);
                }
            }
        }
    }
}
