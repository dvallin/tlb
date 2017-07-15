use specs::{ System, ReadStorage, Fetch, FetchMut, Entities, WriteStorage, Join };

use tcod::input::{ KeyCode };
use game_state::{ GameState };
use event_log::{ EventLog, LogEvent };

use components::player::{ Player };
use components::common::{ Active, InTurn, InTurnState, WaitForTurn, MoveToPosition };
use engine::input_handler::{ InputHandler };

pub struct RoundScheduler;
unsafe impl Sync for RoundScheduler {}

#[derive(SystemData)]
pub struct RoundSchedulerData<'a> {
    entities: Entities<'a>,
    players: ReadStorage<'a, Player>,
    actives: WriteStorage<'a, Active>,
    in_turns: WriteStorage<'a, InTurn>,
    waits: WriteStorage<'a, WaitForTurn>,
    move_to_positions: ReadStorage<'a, MoveToPosition>,
    log: FetchMut<'a, EventLog>,
    state: FetchMut<'a, GameState>,
    input: Fetch<'a, InputHandler>,
}

impl<'a> System<'a> for RoundScheduler {
    type SystemData = RoundSchedulerData<'a>;

    fn run(&mut self, mut data: RoundSchedulerData) {
        if data.input.is_key_pressed(KeyCode::Spacebar) {
            data.state.is_turn_based = !data.state.is_turn_based;

            if data.state.is_turn_based {
                for (id, _) in (&*data.entities, &data.players).join() {
                    data.waits.insert(id, WaitForTurn);
                }
            } else {
                data.waits.clear();
                data.in_turns.clear();
            }
        }

        if data.state.is_turn_based {
            let mut active_became_waiting = false;
            let mut took_turns = vec![];
            if data.input.is_key_pressed(KeyCode::Enter) {
                if let Some((id, _, _, _)) = (&*data.entities, &data.in_turns,
                                              &data.actives, &data.players).join().next() {
                    active_became_waiting = true;
                    took_turns.push(id);
                }
            }

            for (id, in_turn) in (&*data.entities, &mut data.in_turns).join() {
                match in_turn.state {
                    InTurnState::Walking => {
                        if data.move_to_positions.get(id).is_none() {
                            in_turn.action_done();
                        }
                    },
                    _ => {}
                }
                if in_turn.is_done() {
                    took_turns.push(id);
                    if data.actives.get(id).is_some() {
                        active_became_waiting = true;
                    }
                }
            }

            // switch the took turns to wait for turn
            for id in took_turns {
                data.log.log(LogEvent::FinishedTurn(id));
                data.waits.insert(id, WaitForTurn);
                data.in_turns.remove(id);
            }

            // if no one is in turn, put all in turn
            if data.in_turns.join().next().is_none() {
                for (id, _) in (&*data.entities, &data.waits).join() {
                    data.in_turns.insert(id, InTurn::default());
                }
                data.waits.clear();
            }

            // if active became waiting, activate first in turn
            if active_became_waiting {
                if let Some ((id, _)) = (&*data.entities, &data.in_turns).join().next() {
                    if data.players.get(id).is_some() {
                        data.actives.clear();
                        data.actives.insert(id, Active);
                    }
                }
            }
        }

        if data.input.is_key_pressed(KeyCode::Tab) {
            // rotate players
            let mut take_first = true;
            let mut active_player_seen = false;
            for (id, _) in (&*data.entities, &data.players).join() {
                if active_player_seen {
                    data.actives.insert(id, Active);
                    take_first = false;
                    break;
                }
                if data.actives.remove(id).is_some() {
                    active_player_seen = true;
                }
            }
            if take_first {
                if let Some ((id, _)) = (&*data.entities, &data.players).join().next() {
                    data.actives.insert(id, Active);
                }
            }
        }
    }
}
