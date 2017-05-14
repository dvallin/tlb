use specs::World;
use engine::tcod::{ Tcod };

pub enum Transition {
    None,
    Pop,
    Push(Box<State>),
    Switch(Box<State>),
    Exit,
}

pub trait State {
    fn start(&mut self, _tcod: &mut Tcod, _world: &mut World) {}
    fn stop(&mut self, _tcod: &mut Tcod, _world: &mut World) {}
    fn pause(&mut self, _tcod: &mut Tcod, _world: &mut World) {}
    fn resume(&mut self, _tcod: &mut Tcod, _world: &mut World) {}

    fn update(&mut self, _tcod: &mut Tcod, _world: &mut World) -> Transition {
        Transition::None
    }

    fn fixed_update(&mut self, _tcod: &mut Tcod, _world: &mut World) -> Transition {
        Transition::None
    }

    fn handle_events(&mut self, _tcod: &mut Tcod, _world: &mut World) -> Transition {
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
            self.transition(transition, tcod, world)
        }
    }

    pub fn fixed_update(&mut self, tcod: &mut Tcod, world: &mut World) {
        if self.running {
            let transition = match self.states.last_mut() {
                Some(state) => state.fixed_update(tcod, world),
                None => Transition::None,
            };
            self.transition(transition, tcod, world)
        }
    }

    pub fn render(&mut self, tcod: &mut Tcod, world: &mut World) {
        if self.running {
            let state = self.states.last_mut().unwrap();
            state.render(tcod, world);
        }
    }

    pub fn handle_events(&mut self, tcod: &mut Tcod, world: &mut World) {
        if self.running {
            let transition = match self.states.last_mut() {
                Some(state) => state.handle_events(tcod, world),
                None => Transition::None,
            };
            self.transition(transition, tcod, world)
        }
    }

    fn transition(&mut self, transition: Transition, tcod: &mut Tcod, world: &mut World) {
        if self.running {
            match transition {
                Transition::None => (),
                Transition::Pop => self.pop(tcod, world),
                Transition::Push(state) => self.push(tcod, world, state),
                Transition::Switch(state) => self.switch(tcod, world, state),
                Transition::Exit => self.stop(tcod, world),
            }
        }
    }

    fn push(&mut self, tcod: &mut Tcod, world: &mut World, state: Box<State>) {
        if self.running {
            if let Some(mut state) = self.states.last_mut() {
                state.pause(tcod, world);
            }
            self.states.push(state);
            let state = self.states.last_mut().unwrap();
            state.start(tcod, world);
        }
    }

    fn switch(&mut self, tcod: &mut Tcod, world: &mut World, state: Box<State>) {
        if self.running {
            if let Some(mut state) = self.states.pop() {
                state.pause(tcod, world);
            }
            self.states.push(state);
            let state = self.states.last_mut().unwrap();
            state.start(tcod, world);
        }
    }

    fn pop(&mut self, tcod: &mut Tcod, world: &mut World) {
        if self.running {
            if let Some(mut state) = self.states.pop() {
                state.stop(tcod, world);
            }
            if let Some(mut state) = self.states.last_mut() {
                state.resume(tcod, world);
            } else {
                self.running = false;
            }
        }
    }

    fn stop(&mut self, tcod: &mut Tcod, world: &mut World) {
        if self.running {
            while let Some(mut state) = self.states.pop() {
                state.stop(tcod, world);
            }
            self.running = false;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn success() {
        assert!(true);
    }
}
