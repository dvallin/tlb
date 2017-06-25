use specs::{ System, RunArg, Join };

use tcod::input::{ KeyCode };
use game_state::{ GameState };

use components::player::{ Player };
use components::common::{ Active, InTurn, InTurnState, WaitForTurn, MoveToPosition };
use engine::input_handler::{ InputHandler };

pub struct RoundScheduler;
unsafe impl Sync for RoundScheduler {}

impl System<()> for RoundScheduler {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, players, mut actives, mut in_turns, mut waits,
             move_to_positions, mut state, input) = arg.fetch(|w| {
                 (w.entities(),
                  w.read::<Player>(),
                  w.write::<Active>(),
                  w.write::<InTurn>(),
                  w.write::<WaitForTurn>(),
                  w.read::<MoveToPosition>(),
                  w.write_resource::<GameState>(),
                  w.read_resource::<InputHandler>())
        });

        if input.is_key_pressed(KeyCode::Spacebar) {
            state.is_turn_based = !state.is_turn_based;

            if state.is_turn_based {
                for (id, _) in (&entities, &players).iter() {
                    waits.insert(id, WaitForTurn);
                }
            } else {
                waits.clear();
                in_turns.clear();
            }
        }

        if state.is_turn_based {
            let mut active_became_waiting = false;
            let mut took_turns = vec![];
            if input.is_key_pressed(KeyCode::Enter) {
                if let Some((id, _, _, _)) = (&entities, &in_turns,
                                              &actives, &players) .iter().next() {
                    active_became_waiting = true;
                    took_turns.push(id);
                }
            }

            for (id, in_turn) in (&entities, &mut in_turns).iter() {
                match in_turn.state {
                    InTurnState::Walking => {
                        if move_to_positions.get(id).is_none() {
                            in_turn.action_done();
                        }
                    },
                    _ => {}
                }
                if in_turn.is_done() {
                    took_turns.push(id);
                    if actives.get(id).is_some() {
                        active_became_waiting = true;
                    }
                }
            }

            // switch the took turns to wait for turn
            for id in took_turns {
                waits.insert(id, WaitForTurn);
                in_turns.remove(id);
            }

            // if no one is in turn, put all in turn
            if in_turns.iter().next().is_none() {
                for (id, _) in (&entities, &waits).iter() {
                    in_turns.insert(id, InTurn::default());
                }
                waits.clear();
            }

            // if active became waiting, activate first in turn
            if active_became_waiting {
                if let Some ((id, _)) = (&entities, &in_turns).iter().next() {
                    if players.get(id).is_some() {
                        actives.clear();
                        actives.insert(id, Active);
                    }
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
