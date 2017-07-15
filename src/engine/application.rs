use std::time::{ Duration, Instant };

use specs::{ World, Dispatcher };

use engine::state::{ StateMachine, State };
use engine::tcod::{ Tcod };
use engine::time::{ Time, Stopwatch };

use components::appearance::{ Renderable, Layer0, Layer1 };
use components::space::{ Position };

pub struct Application<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    world: World,
    state: StateMachine,
    tcod: Tcod,

    timer: Stopwatch,
    delta_time: Duration,
    fixed_step: Duration,
    last_fixed_update: Instant,
}

impl<'a, 'b> Application<'a, 'b> {
    pub fn new<T>(initial_state: T, mut world: World, dispatcher: Dispatcher<'a, 'b>) -> Self
        where T: State + 'static {
        {
            let time = Time {
                delta_time: Duration::new(0, 0),
                fixed_step: Duration::new(0, 16666666),
                last_fixed_update: Instant::now(),
            };

            world.add_resource::<Time>(time);

            world.register::<Renderable>();
            world.register::<Layer0>();
            world.register::<Layer1>();

            world.register::<Position>();
        }

        Application {
            dispatcher: dispatcher,
            world: world,
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
            {
                let mut time = self.world.write_resource::<Time>();
                time.delta_time = self.delta_time;
                time.last_fixed_update = self.last_fixed_update;
                time.fixed_step = self.fixed_step;
            }

            self.state.handle_events(&mut self.tcod, &mut self.world);
            if self.last_fixed_update.elapsed() >= self.fixed_step {
                self.state.fixed_update(&mut self.tcod, &mut self.world);
                self.last_fixed_update += self.fixed_step;
            }
            self.state.update(&mut self.tcod, &mut self.world);
        }

        // execute world update
        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();

        { // render world
            self.state.render(&mut self.tcod, &mut self.world);
        }
    }

    fn initialize(&mut self) {
        self.state.start(&mut self.tcod, &mut self.world);
    }
}
