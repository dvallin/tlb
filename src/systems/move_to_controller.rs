use specs::{ System, RunArg, Join };

use components::space::{ Position, mul, Viewport };
use components::common::{ Active, MoveToPosition };
use engine::time::{ Time };

use maps::{ Map, Maps };

pub struct MoveToController;
unsafe impl Sync for MoveToController {}

impl System<()> for MoveToController {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (entities, mut positions, mut move_to_positions, time,
             actives,  mut maps, mut viewport) = arg.fetch(|w| {
                 (w.entities(),
                  w.write::<Position>(),
                  w.write::<MoveToPosition>(),
                  w.read_resource::<Time>(),
                  w.read::<Active>(),
                  w.write_resource::<Maps>(),
                  w.write_resource::<Viewport>())
             });

        let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

        let mut finished_entities = vec![];
        for (id, p, t) in (&entities, &mut positions, &mut move_to_positions).iter() {
            // map last to is_reached? using the walking function
            if t.path.last().map_or(false, |next_pos|
                if !p.approx_equal(&next_pos) {
                    let delta = *next_pos - *p;
                    let mut np = *p + mul(delta.norm(), delta_time*t.speed);

                    // do not overshoot!
                    let delta_new = *next_pos - np;
                    if delta.dot(&delta_new) < 0.0 {
                        np = *next_pos;
                    }

                    // actually walk to target
                    if !maps.is_impassable(&id, (np.x as i32, np.y as i32)) {
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
                t.path.pop();
            }
            if t.path.len() == 0 {
                finished_entities.push(id);
            }
        }
        for (p, _) in (&positions, &actives).iter() {
            // center at player
            viewport.center_at(*p);
        }
        for id in finished_entities {
            move_to_positions.remove(id);
        }
    }
}
