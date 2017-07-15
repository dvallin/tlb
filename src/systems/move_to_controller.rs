use specs::{ System, ReadStorage, Fetch, FetchMut, Entities, WriteStorage, Join };

use components::space::{ Position, Level, mul, Viewport };
use components::common::{ Active, MoveToPosition };
use engine::time::{ Time };

use tower::{ Tower };
use maps::{ Map };

pub struct MoveToController;
unsafe impl Sync for MoveToController {}

#[derive(SystemData)]
pub struct MoveToControllerData<'a> {
    entities: Entities<'a>,
    positions: WriteStorage<'a, Position>,
    levels: ReadStorage<'a, Level>,
    move_to_positions: WriteStorage<'a, MoveToPosition>,
    actives: ReadStorage<'a, Active>,
    tower: FetchMut<'a, Tower>,
    viewport: FetchMut<'a, Viewport>,
    time: Fetch<'a, Time>,
}

fn move_to(pos: &Position, next_pos: &Position, movement: &MoveToPosition, delta_time: f32) -> Position {
    let delta = *next_pos - *pos;
    let mut np = *pos + mul(delta.norm(), delta_time*movement.speed);

    // do not overshoot!
    let delta_new = *next_pos - np;
    if delta.dot(&delta_new) < 0.0 {
        np = *next_pos;
    }
    np
}

impl<'a> System<'a> for MoveToController {
    type SystemData = MoveToControllerData<'a>;

    fn run(&mut self, mut data: MoveToControllerData) {
        let delta_time = data.time.delta_time.subsec_nanos() as f32 / 1.0e9;

        let mut finished_entities = vec![];
        for (id, p, level, t) in (&*data.entities, &mut data.positions, &data.levels, &mut data.move_to_positions).join() {
            let maps = data.tower.get_mut(level).unwrap();
            // map last to is_reached? using the walking function
            if t.path.front().map_or(false, |next_pos|
                if !p.approx_equal(&next_pos) {
                    let np = move_to(p, next_pos, t, delta_time);
                    // actually walk to target
                    if !maps.is_not_planable(&id, (np.x as i32, np.y as i32)) {
                        maps.move_entity(Map::Character, &id,
                                        (p.x as i32, p.y as i32),
                                        (np.x as i32, np.y as i32));
                        *p = np;
                        false
                    } else {
                        // target is unreachable
                        true
                    }
                } else {
                    // target is reached
                    true
                }
            ) {
                t.path.pop_front();
            }
            if t.path.len() == 0 {
                finished_entities.push(id);
            }
        }

        if let Some((p, _)) = (&data.positions, &data.actives).join().next() {
            // center at player
            data.viewport.center_at(*p);
        }

        for id in finished_entities {
            data.move_to_positions.remove(id);
        }
    }
}
