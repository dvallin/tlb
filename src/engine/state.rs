use specs::World;
use engine::tcod::{ Tcod };

pub enum Transition {
    None,
    Exit,
}

pub trait State {
    fn start(&mut self, tcod: &mut Tcod, world: &mut World);

    fn update(&mut self, _tcod: &mut Tcod, _world: &mut World) -> Transition {
        Transition::None
    }

    fn fixed_update(&mut self, _tcod: &mut Tcod, _world: &mut World) -> Transition {
        Transition::None
    }

    fn handle_events(&mut self, _world: &mut World) -> Transition {
        Transition::None
    }

    fn render(&mut self, tcod: &mut Tcod, world: &mut World);
}

pub struct StateMachine {
    running: bool,
    states: Vec<Box<State>>,
}

impl StateMachine {
    pub fn new<T>(initial_state: T) -> StateMachine where T: State + 'static{
        StateMachine {
            running: false,
            states: vec![Box::new(initial_state)],
        }
    }

    pub fn start(&mut self, tcod: &mut Tcod, world: &mut World) {
        if !self.running {
            let state = self.states.last_mut().unwrap();
            state.start(tcod, world);
            self.running = true;
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn update(&mut self, tcod: &mut Tcod, world: &mut World) {
        if self.running {
            let transition = match self.states.last_mut() {
                Some(state) => state.update(tcod, world),
                None => Transition::None,
            };
            self.transition(transition, world)
        }
    }

    pub fn fixed_update(&mut self, tcod: &mut Tcod, world: &mut World) {
        if self.running {
            let transition = match self.states.last_mut() {
                Some(state) => state.fixed_update(tcod, world),
                None => Transition::None,
            };
            self.transition(transition, world)
        }
    }

    pub fn render(&mut self, tcod: &mut Tcod, world: &mut World) {
        if self.running {
            let state = self.states.last_mut().unwrap();
            state.render(tcod, world);
        }
    }

    pub fn handle_events(&mut self, world: &mut World) {
        if self.running {
            let transition = match self.states.last_mut() {
                Some(state) => state.handle_events(world),
                None => Transition::None,
            };
            self.transition(transition, world)
        }
    }

    fn transition(&mut self, transition: Transition, world: &mut World) {
        if self.running {
            match transition {
                Transition::None => (),
                Transition::Exit => self.stop(world),
            }
        }
    }

    fn stop(&mut self, _world: &mut World) {
        self.running = false;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn success() {
        assert!(true);
    }
}
