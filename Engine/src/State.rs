use std::cell::RefCell;
use std::rc::Rc;

pub trait State {
    fn Update(&mut self) {
        println!("default update");
    }
    fn Draw(&mut self) {
        println!("default draw");
    }
    fn HandleInput(&mut self) {
        println!("default Handle input");
    }

    fn OnEnter(&mut self) -> bool {
        println!("entering");
        true
    }
    fn OnExit(&mut self) -> bool {
        println!("exiting");

        true
    }

    fn ShouldTransition(&mut self) -> bool {
        true
    }

    fn GetNextState(&self) -> StateRef {
        println!("default next state");
        Rc::new(RefCell::new(DefaultState::new()))
    }
}

type StateRef = Rc<RefCell<dyn State>>;

pub struct DefaultState {
    // name: String,
    m_ShouldTransition : bool,
}

impl DefaultState {
    pub fn new() -> Self {
        DefaultState {m_ShouldTransition : false}
    }
}
impl State for DefaultState {
    fn Update(&mut self) {
        println!("Default state update");
    }

    fn ShouldTransition(&mut self) -> bool
    {
        self.m_ShouldTransition
    }
}
