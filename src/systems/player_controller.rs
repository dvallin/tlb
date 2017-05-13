use specs::{ System, RunArg, Join };

use tcod::input::{ KeyCode };

use components::space::{ Position, Vector, Viewport, mul };
use components::player::{ Player };
use components::inventory::{ Inventory };
use engine::input_handler::{ InputHandler };
use engine::time::{ Time };
use tilemap::{ TileMap };
use itemmap::{ ItemMap };

const PLAYER_SPEED: f32 = 4.0;

pub struct PlayerController;
unsafe impl Sync for PlayerController {}

fn move_player(position: &mut Position, input: &InputHandler, delta_time: f32) -> Position {
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
    *position + mul(delta.norm(), delta_time*PLAYER_SPEED)
}

impl System<()> for PlayerController {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, mut players, mut positions, mut inventories,
             time, input, tile_map, mut item_map, mut viewport) = arg.fetch(|w| {
                 (w.entities(),
                  w.write::<Player>(),
                  w.write::<Position>(),
                  w.write::<Inventory>(),
                  w.read_resource::<Time>(),
                  w.read_resource::<InputHandler>(),
                  w.read_resource::<TileMap>(),
                  w.write_resource::<ItemMap>(),
                  w.write_resource::<Viewport>())
        });

        let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

        let mut active_player = None;
        if input.is_key_pressed(KeyCode::Tab) {
            // rotate players
            let mut take_first = true;
            let mut active_player_seen = false;
            for (id, player) in (&entities, &mut players).iter() {
                if active_player_seen {
                    player.active = true;
                    active_player = Some(id);
                    take_first = false;
                    break;
                }
                if player.active {
                    player.active = false;
                    active_player_seen = true;
                }
            }
            if take_first {
                for (id, player) in (&entities, &mut players).iter() {
                    player.active = true;
                    active_player = Some(id);
                    break;
                }
            }
        } else {
            for (id, player) in (&entities, &mut players).iter() {
                if player.active {
                    active_player = Some(id);
                    break;
                }
            }
        }

        if let Some(id) = active_player {
            let mut position = None;
            if let Some(p) = positions.get_mut(id) {
                let np = move_player(p, &input, delta_time);
                if !tile_map.is_blocking(np.x as i32, np.y as i32) {
                    *p = np;
                }
                position = Some(p.clone());
            }

            if let Some(p) = position {
                // center at player
                viewport.center_at(p);

                if let Some(inventory) = inventories.get_mut(id) {
                    // player interaction
                    if input.is_char_pressed('p') {
                        if let Some(item_id) = item_map.pop(p.x as i32, p.y as i32) {
                            inventory.push(item_id);
                            positions.remove(item_id);
                        }
                    } else if input.is_char_pressed('d') {
                        if let Some(item_id) = inventory.pop() {
                            item_map.push(&item_id, p.x as i32, p.y as i32);
                            positions.insert(item_id, p);
                        }
                    }
                }
            }

        }
    }

}
