use std::cell::RefCell;
use std::rc::Rc;

use spdlog::prelude::*;


pub trait State {
    fn Update(&mut self) {
        info!("default update");
    }
    fn Draw(&mut self) {
        info!("default draw");
    }
    fn HandleInput(&mut self) {
        info!("default Handle input");
    }

    fn OnEnter(&mut self) -> bool {
        info!("entering");
        true
    }
    fn OnExit(&mut self) -> bool {
        info!("exiting");

        true
    }

    fn ShouldTransition(&self) -> bool {
        true
    }

     fn IsRunning(&self) -> bool {
        true
    }

    fn GetNextState(&self) -> StateRef {
        info!("default next state");
        Rc::new(RefCell::new(DefaultState::new()))
    }
}

type StateRef = Rc<RefCell<dyn State>>;

pub struct DefaultState {
    // name: String,
    m_ShouldTransition: bool,
}

impl DefaultState {
    pub fn new() -> Self {
        DefaultState {
            m_ShouldTransition: false,
        }
    }
}
impl State for DefaultState {
    fn Update(&mut self) {
        info!("Default state update");
    }

    fn ShouldTransition(&self) -> bool {
        self.m_ShouldTransition
    }

   
}
