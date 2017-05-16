use specs::{ System, RunArg, Join };

use components::space::{ Position, mul, Viewport };
use components::common::{ Active, MoveToPosition };
use engine::time::{ Time };

use maps::{ Map, Maps };

const PLAYER_SPEED: f32 = 4.0;

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
            if p.x as i32 != t.position.x as i32 || p.y as i32 != t.position.y as i32 {
                let delta = t.position - *p;
                let np = *p + mul(delta.norm(), delta_time*PLAYER_SPEED);
                if !maps.is_impassable(&id, np.x as i32, np.y as i32) {
                    maps.move_entity(Map::Character, &id,
                                     p.x as i32, p.y as i32, np.x as i32, np.y as i32);
                    *p = np;
                } else {
                    finished_entities.push(id);
                }
            } else {
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
