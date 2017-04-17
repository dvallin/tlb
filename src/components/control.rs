use specs::{ Component, HashMapStorage, World, System, RunArg, Join };

use components::space::{ Position, Vector, mul };
use engine::input_handler::{ InputHandler };
use engine::time::{ Time };


const PLAYER_SPEED: f32 = 4.0;

pub struct Player {
    pub active: bool,
    pub index: usize,
}

impl Component for Player {
    type Storage = HashMapStorage<Player>;
}

pub struct Fov {
    pub index: usize,
}

impl Component for Fov {
    type Storage = HashMapStorage<Fov>;
}


pub struct PlayerController;
unsafe impl Sync for PlayerController {}

impl System<()> for PlayerController {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (mut players, mut positions, time, input) = arg.fetch(|w| {
            (w.write::<Player>(),
             w.write::<Position>(),
             w.read_resource::<Time>(),
             w.read_resource::<InputHandler>())
        });

        let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

        // switch players
        let mut switch_player : Option<usize> = None;
        if input.is_pressed('1') {
            switch_player = Some(1);
        } else if input.is_pressed('2') {
            switch_player = Some(2);
        }
        if let Some(index) = switch_player {
            for player in (&mut players).iter() {
                player.active = player.index == index;
            }
        }

        // move players
        for (player, position) in (&players, &mut positions).iter() {
            if player.active {
                let mut delta = Vector { x: 0.0, y: 0.0 };
                if input.is_pressed('h') {
                    delta.x -= 1.0;
                }
                if input.is_pressed('j') {
                    delta.y += 1.0;
                }
                if input.is_pressed('k') {
                    delta.y -= 1.0;
                }
                if input.is_pressed('l') {
                    delta.x += 1.0;
                }
                *position += mul(delta.norm(), delta_time*PLAYER_SPEED)
            }
        }
    }
}
