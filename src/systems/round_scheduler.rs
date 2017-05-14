use specs::{ System, RunArg, Join };

use tcod::input::{ KeyCode };

use components::player::{ Player };
use components::common::{ Active };
use engine::input_handler::{ InputHandler };

pub struct RoundScheduler;
unsafe impl Sync for RoundScheduler {}

impl System<()> for RoundScheduler {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, players, mut actives, input) = arg.fetch(|w| {
                 (w.entities(),
                  w.read::<Player>(),
                  w.write::<Active>(),
                  w.read_resource::<InputHandler>())
        });

        if input.is_key_pressed(KeyCode::Tab) {
            // rotate players
            let mut take_first = true;
            let mut active_player_seen = false;
            for (id, _) in (&entities, &players).iter() {
                if active_player_seen {
                    actives.insert(id, Active);
                    take_first = false;
                    break;
                }
                if actives.remove(id).is_some() {
                    active_player_seen = true;
                }
            }
            if take_first {
                for (id, _) in (&entities, &players).iter() {
                    actives.insert(id, Active);
                    break;
                }
            }
        }
    }
}
