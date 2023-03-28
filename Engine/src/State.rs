use std::cell::{RefCell, Ref};
use std::rc::Rc;

use spdlog::prelude::*;

use crate::Camera::Camera;
use crate::Renderer::IRenderer::IRenderer;
use crate::Renderer::SimpleRenderer::SimpleRenderer;

extern crate glfw;
extern crate gl;

pub trait State {
    fn Update(&mut self, window : Rc<RefCell<glfw::Window>>) {
        // info!("default update");
    }
    fn Draw(&mut self, window : Rc<RefCell<glfw::Window>>, renderer : Rc<RefCell<SimpleRenderer>>,camera : &Camera, deltaTime : f64) {
        // info!("default draw");
    }
    fn HandleKeyBoardInput(&mut self, _window: Rc<RefCell<glfw::Window>>, _key : glfw::Key, _action : glfw::Action, _modifiers : glfw::Modifiers) {
        // info!("default Handle input");
        
    }

    fn HandleMouseInput(&mut self, _window: Rc<RefCell<glfw::Window>>, _button : glfw::MouseButton, _action : glfw::Action, _modifiers : glfw::Modifiers) {
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
    fn Update(&mut self, window : Rc<RefCell<glfw::Window>>) {
        info!("Default state update");
    }

    fn ShouldTransition(&self) -> bool {
        self.m_ShouldTransition
    }
}
