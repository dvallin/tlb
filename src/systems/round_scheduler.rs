use specs::{ System, RunArg, Join };

use tcod::input::{ KeyCode };
use game_state::{ GameState };

use components::player::{ Player };
use components::common::{ Active, WaitForTurn, TookTurn };
use engine::input_handler::{ InputHandler };

pub struct RoundScheduler;
unsafe impl Sync for RoundScheduler {}

impl System<()> for RoundScheduler {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, players, mut actives, mut waits, mut took_turns,
             mut state, input) = arg.fetch(|w| {
                 (w.entities(),
                  w.read::<Player>(),
                  w.write::<Active>(),
                  w.write::<WaitForTurn>(),
                  w.write::<TookTurn>(),
                  w.write_resource::<GameState>(),
                  w.read_resource::<InputHandler>())
        });

        if input.is_key_pressed(KeyCode::Spacebar) {
            state.is_turn_based = !state.is_turn_based;

            if state.is_turn_based {
                for (id, _, _) in (&entities, !&actives, &players).iter() {
                    waits.insert(id, WaitForTurn);
                }
            } else {
                waits.clear();
                took_turns.clear();
            }
        }

        if state.is_turn_based {
            if input.is_key_pressed(KeyCode::Enter) {
                // switch active to took turn
                if let Some ((id, _, _)) = (&entities, &actives, &players).iter().next() {
                    took_turns.insert(id, TookTurn);
                    actives.remove(id);
                }

                // if no one is waiting, put all in waiting
                if waits.iter().next().is_none() {
                    for (id, _, _) in (&entities, &took_turns, &players).iter() {
                        waits.insert(id, WaitForTurn);
                    }
                    took_turns.clear();
                }

                // take first from waiting into turn
                if let Some ((id, _, _)) = (&entities, &waits, &players).iter().next() {
                    actives.insert(id, Active);
                    waits.remove(id);
                }
            }
        } else {
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
                    if let Some ((id, _)) = (&entities, &players).iter().next() {
                        actives.insert(id, Active);
                    }
                }
            }
        }
    }
}
