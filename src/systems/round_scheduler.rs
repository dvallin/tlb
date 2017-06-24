use specs::{ System, RunArg, Join };

use tcod::input::{ KeyCode };
use game_state::{ GameState };

use components::player::{ Player };
use components::common::{ Active, InTurn, InTurnState, WaitForTurn, TookTurn, MoveToPosition };
use engine::input_handler::{ InputHandler };

pub struct RoundScheduler;
unsafe impl Sync for RoundScheduler {}

impl System<()> for RoundScheduler {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, players, mut actives, mut in_turns, mut waits, mut took_turns,
             move_to_positions, mut state, input) = arg.fetch(|w| {
                 (w.entities(),
                  w.read::<Player>(),
                  w.write::<Active>(),
                  w.write::<InTurn>(),
                  w.write::<WaitForTurn>(),
                  w.write::<TookTurn>(),
                  w.read::<MoveToPosition>(),
                  w.write_resource::<GameState>(),
                  w.read_resource::<InputHandler>())
        });

        if input.is_key_pressed(KeyCode::Spacebar) {
            state.is_turn_based = !state.is_turn_based;

            if state.is_turn_based {
                if let Some ((id, _, _)) = (&entities, &actives, &players).iter().next() {
                    in_turns.insert(id, InTurn::default());
                }
                for (id, _, _) in (&entities, !&in_turns, &players).iter() {
                    waits.insert(id, WaitForTurn);
                }
            } else {
                waits.clear();
                took_turns.clear();
                in_turns.clear();
            }
        }

        if state.is_turn_based {
            let actives_players_turn = (&in_turns, &actives, &players).iter().next().is_some();

            let mut took_turn = false;
            if actives_players_turn && input.is_key_pressed(KeyCode::Enter) {
                // 'skip turn'.
                took_turn = true;
            }

            if let Some ((id, in_turn)) = (&entities, &mut in_turns).iter().next() {
                match in_turn.state {
                    InTurnState::Walking => {
                        if move_to_positions.get(id).is_none() {
                            in_turn.action_done();
                        }
                    },
                    _ => {}
                }

                took_turn = in_turn.is_done();
            }

            if took_turn {
                // switch active to took turn
                if let Some ((id, _)) = (&entities, &in_turns).iter().next() {
                    took_turns.insert(id, TookTurn);
                    in_turns.remove(id);
                }

                // if no one is waiting, put all in waiting
                if waits.iter().next().is_none() {
                    for (id, _) in (&entities, &took_turns).iter() {
                        waits.insert(id, WaitForTurn);
                    }
                    took_turns.clear();
                }

                // take first from waiting into turn
                if let Some ((id, _)) = (&entities, &waits).iter().next() {
                    in_turns.insert(id, InTurn::default());
                    if players.get(id).is_some() {
                       actives.clear();
                       actives.insert(id, Active);
                    }
                    waits.remove(id);
                }
            }
        }

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
