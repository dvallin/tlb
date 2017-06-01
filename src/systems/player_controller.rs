use specs::{ System, RunArg, Join };

use components::space::{ Position, Vector, Viewport, mul };
use components::player::{ Player };
use components::common::{ Active, MoveToPosition };
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

const SCREEN_HEIGHT: i32 = 50;
const MAP_HEIGHT: i32 = 43;
const MAP_Y: i32 = SCREEN_HEIGHT - MAP_HEIGHT;
const PLAYER_SPEED: f32 = 4.0;
impl System<()> for PlayerController {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, players, actives, mut positions, mut inventories, mut move_to_positions,
             time, input, mut maps, viewport) = arg.fetch(|w| {
                 (w.entities(),
                  w.read::<Player>(),
                  w.read::<Active>(),
                  w.write::<Position>(),
                  w.write::<Inventory>(),
                  w.write::<MoveToPosition>(),
                  w.read_resource::<Time>(),
                  w.read_resource::<InputHandler>(),
                  w.write_resource::<Maps>(),
                  w.read_resource::<Viewport>())
        });

        let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

        for (id, p, _, _) in (&entities, &positions, &players, &actives).iter() {
            if input.is_mouse_pressed() {
                // create automatic movement
                let mut pos = input.mouse_pos();
                pos.y -= MAP_Y;
                let pos_trans = viewport.inv_transform(pos);
                if viewport.visible(pos_trans) {
                    // set the position to the middle of the cell to avoid twitching.
                    let pos_target = Position { x: pos_trans.x as f32 + 0.5,
                                                y: pos_trans.y as f32 + 0.5 };
                    move_to_positions.insert(id, MoveToPosition { position: pos_target, speed: PLAYER_SPEED });
                }
            } else {
                // player direct movement
                let delta = get_delta(&input);
                if delta.x != 0.0 || delta.y != 0.0 {
                    let np = *p + mul(delta.norm(), delta_time*PLAYER_SPEED);
                    move_to_positions.insert(id, MoveToPosition { position: np, speed: PLAYER_SPEED });
                }
            }
        }

        for (id, inventory, _, _) in (&entities, &mut inventories, &players, &actives).iter() {
            let p = positions.get(id).unwrap().clone();
            // player interaction
            if input.is_char_pressed('p') {
                if let Some(item_id) = maps.pop(Map::Item, p.x as i32, p.y as i32) {
                    inventory.push(item_id);
                    positions.remove(item_id);
                }
            } else if input.is_char_pressed('d') {
                if let Some(item_id) = inventory.pop() {
                    maps.push(Map::Item, &item_id, p.x as i32, p.y as i32);
                    positions.insert(item_id, p);
                }
            }
        }
    }
}
