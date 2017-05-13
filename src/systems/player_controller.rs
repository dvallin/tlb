use specs::{ System, RunArg, Join };

use components::space::{ Position, Vector, Viewport, mul };
use components::player::{ Player };
use engine::input_handler::{ InputHandler };
use engine::time::{ Time };
use tilemap::{ TileMap };

const PLAYER_SPEED: f32 = 4.0;

pub struct PlayerController;
unsafe impl Sync for PlayerController {}

impl System<()> for PlayerController {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (mut players, mut positions, time, input, tilemap, mut viewport) = arg.fetch(|w| {
            (w.write::<Player>(),
             w.write::<Position>(),
             w.read_resource::<Time>(),
             w.read_resource::<InputHandler>(),
             w.read_resource::<TileMap>(),
             w.write_resource::<Viewport>())
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
        let mut current_player_position = None;
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
                let new_position = *position + mul(delta.norm(), delta_time*PLAYER_SPEED);
                if !tilemap.is_blocking(new_position.x as i32, new_position.y as i32) {
                    *position = new_position;
                }
                current_player_position = Some(position);
            }
        }

        if let Some(p) = current_player_position {
            viewport.center_at(*p);
        }
    }
}
