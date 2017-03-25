use num_cpus;

use std::time::{ Duration, Instant };

use specs::{ World, Planner };

use engine::state::{ StateMachine, State };
use engine::tcod::{ Tcod };
use engine::time::{ Time, Stopwatch };

use components::appearance::{ Renderable };
use components::space::{ Position };

pub struct Application {
    planner: Planner<()>,
    state: StateMachine,
    tcod: Tcod,

    timer: Stopwatch,
    delta_time: Duration,
    fixed_step: Duration,
    last_fixed_update: Instant,
}

impl Application {
    pub fn new<T>(initial_state: T) -> Self where T: State + 'static {
        let mut world = World::new();

        let time = Time {
            delta_time: Duration::new(0, 0),
            fixed_step: Duration::new(0, 16666666),
            last_fixed_update: Instant::now(),
        };

        world.add_resource::<Time>(time);

        world.register::<Renderable>();
        world.register::<Position>();
        Application {
            planner: Planner::new(world, num_cpus::get()),
            state: StateMachine::new(initial_state),
            tcod: Tcod::new(),
            timer: Stopwatch::default(),
            delta_time: Duration::new(0, 0),
            fixed_step: Duration::new(0, 16666666),
            last_fixed_update: Instant::now(),
        }
    }

    pub fn run(&mut self) {
        self.initialize();
        while self.state.is_running() {
            self.timer.restart();
            self.step();
            self.timer.stop();
            self.delta_time = self.timer.elapsed();
        }
    }

    fn step(&mut self) {
        { // prepare world update
            let world = self.planner.mut_world();
            {
                let mut time = world.write_resource::<Time>();
                time.delta_time = self.delta_time;
                time.last_fixed_update = self.last_fixed_update;
                time.fixed_step = self.fixed_step;
            }

            self.state.handle_events(world);
            if self.last_fixed_update.elapsed() >= self.fixed_step {
                self.state.fixed_update(world);
                self.last_fixed_update += self.fixed_step;
            }
            self.state.update(world);
        }

        // execute world update
        self.planner.dispatch(());
        self.planner.wait();

        { // render world
            let world = &mut self.planner.mut_world();
            self.tcod.render(world);
        }
    }

    fn initialize(&mut self) {
        let world = self.planner.mut_world();
        self.state.start(world);
    }
}
