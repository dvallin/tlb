use specs::{ System, RunArg, Join };

use components::space::{ Position, Vector, Viewport, mul };
use components::player::{ Player };
use components::common::{ Active };
use components::inventory::{ Inventory };
use engine::input_handler::{ InputHandler };
use engine::time::{ Time };

use maps::{ Maps };

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
        let (entities, players, actives, mut positions, mut inventories,
             time, input, mut maps, mut viewport) = arg.fetch(|w| {
                 (w.entities(),
                  w.read::<Player>(),
                  w.read::<Active>(),
                  w.write::<Position>(),
                  w.write::<Inventory>(),
                  w.read_resource::<Time>(),
                  w.read_resource::<InputHandler>(),
                  w.write_resource::<Maps>(),
                  w.write_resource::<Viewport>())
        });

        let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

        for (p, _, _) in (&mut positions, &players, &actives).iter() {
            let np = move_player(p, &input, delta_time);
            if !maps.is_blocking(np.x as i32, np.y as i32) {
                *p = np;
            }
            // center at player
            viewport.center_at(*p);
        }

        for (id, inventory, _, _) in (&entities, &mut inventories, &players, &actives).iter() {
            let p = positions.get(id).unwrap().clone();
            // player interaction
            if input.is_char_pressed('p') {
                if let Some(item_id) = maps.pop_item(p.x as i32, p.y as i32) {
                    inventory.push(item_id);
                    positions.remove(item_id);
                }
            } else if input.is_char_pressed('d') {
                if let Some(item_id) = inventory.pop() {
                    maps.push_item(&item_id, p.x as i32, p.y as i32);
                    positions.insert(item_id, p);
                }
            }
        }
    }
}
