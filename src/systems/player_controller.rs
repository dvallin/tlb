use specs::{ System, RunArg, Join };

use game_state::{ GameState };

use components::space::{ Position, Vector, Viewport, mul };
use components::player::{ Player };
use components::common::{ Active, InTurn, InTurnState, MoveToPosition };
use components::inventory::{ Inventory };
use engine::input_handler::{ InputHandler };
use engine::time::{ Time };

use maps::{ Map, Maps };

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

const PLAYER_SPEED: f32 = 4.0;
impl System<()> for PlayerController {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, players, actives, mut positions, mut inventories, mut move_to_positions,
             mut in_turns, time, state, input, mut maps, viewport) = arg.fetch(|w| {
                 (w.entities(),
                  w.read::<Player>(),
                  w.read::<Active>(),
                  w.write::<Position>(),
                  w.write::<Inventory>(),
                  w.write::<MoveToPosition>(),
                  w.write::<InTurn>(),
                  w.read_resource::<Time>(),
                  w.read_resource::<GameState>(),
                  w.read_resource::<InputHandler>(),
                  w.write_resource::<Maps>(),
                  w.read_resource::<Viewport>())
        });

        let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

        if state.is_turn_based {
            if let Some ((id, p, _, in_turn)) = (&entities, &positions, &players, &mut in_turns).iter().next() {
                if input.is_mouse_pressed() {
                    // create automatic movement
                    let pos_trans = viewport.inv_transform(input.mouse_pos);
                    if let Some(pos) = maps.screen_to_map(pos_trans) {
                        if viewport.visible(pos_trans) {
                            // set the position to the middle of the cell to avoid twitching.
                            let path = maps.find_path(&id, (p.x as i32, p.y as i32), pos);
                            move_to_positions.insert(id, MoveToPosition { path: path,
                                                                        speed: PLAYER_SPEED });
                            in_turn.0 = InTurnState::Walking;
                        }
                    }
                }
            }
        } else {
            if let Some((id, p, _, _)) = (&entities, &positions, &players, &actives).iter().next() {
                if input.is_mouse_pressed() {
                    // create automatic movement
                    let pos_trans = viewport.inv_transform(input.mouse_pos);
                    if let Some(pos) = maps.screen_to_map(pos_trans) {
                        if viewport.visible(pos_trans) {
                            // set the position to the middle of the cell to avoid twitching.
                            let path = maps.find_path(&id, (p.x as i32, p.y as i32), pos);
                            move_to_positions.insert(id, MoveToPosition { path: path,
                                                                        speed: PLAYER_SPEED });
                        }
                    }
                } else {
                    // player direct movement
                    let delta = get_delta(&input);
                    if delta.x != 0.0 || delta.y != 0.0 {
                        let np = *p + mul(delta.norm(), delta_time*PLAYER_SPEED);
                        move_to_positions.insert(id, MoveToPosition { path: vec![np], speed: PLAYER_SPEED });
                    }
                }
            }

            if let Some((id, inventory, _, _)) =
                (&entities, &mut inventories, &players, &actives).iter().next() {
                let p = positions.get(id).unwrap().clone();
                // player interaction
                if input.is_char_pressed('p') {
                    if let Some(item_id) = maps.pop(Map::Item, (p.x as i32, p.y as i32)) {
                        inventory.push(item_id);
                        positions.remove(item_id);
                    }
                } else if input.is_char_pressed('d') {
                    if let Some(item_id) = inventory.pop() {
                        maps.push(Map::Item, &item_id, (p.x as i32, p.y as i32));
                        positions.insert(item_id, p);
                    }
                }
            }
        }
    }
}
