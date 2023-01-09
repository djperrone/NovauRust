use std::cell::RefCell;
use std::rc::Rc;

use spdlog::prelude::*;


extern crate glfw;
// use glfw::WindowEvent;

// use self::glfw::{Action, Context, Key};

extern crate gl;

use std::sync::mpsc::Receiver;

use crate::Input::InputController;


pub trait State {
    fn Update(&mut self) {
        // info!("default update");
    }
    fn Draw(&mut self) {
        // info!("default draw");
    }
    fn HandleKeyBoardInput(&mut self, window: Rc<RefCell<glfw::Window>>, key : glfw::Key, action : glfw::Action, modifiers : glfw::Modifiers) {
        // info!("default Handle input");
        
    }

    fn HandleMouseInput(&mut self, window: Rc<RefCell<glfw::Window>>, button : glfw::MouseButton, action : glfw::Action, modifiers : glfw::Modifiers) {
        // info!("default Handle input");
        
    }

    fn OnEnter(&mut self) -> bool {
        // info!("entering");
        true
    }
    fn OnExit(&mut self) -> bool {
        // info!("exiting");

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

    fn GetInputController(&self) -> Option<Rc<RefCell<InputController>>>
    {
        None
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
