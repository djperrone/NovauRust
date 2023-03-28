use std::cell::RefCell;
use std::rc::Rc;

use crate::Camera::Camera;
use crate::Renderer::IRenderer::IRenderer;
use crate::Renderer::SimpleRenderer::SimpleRenderer;
use crate::State::State;



type StateRef = Rc<RefCell<dyn State>>;

pub struct StateMachine {
    m_States: Vec<StateRef>,
}

impl StateMachine {
    pub fn new() -> Self {
        println!("sm new here");

        StateMachine {
            m_States: Vec::new(),
        }
    }

    pub fn PushState(&mut self, s: StateRef) -> () {
        self.m_States.push(s);
        match self.m_States.last() {
            Some(state) => state.borrow_mut().OnEnter(),
            None => false,
        };
    }

    pub fn PopState(&mut self) -> () {
        let last = self.m_States.pop();

        match last {
            Some(last) => last.borrow_mut().OnExit(),
            None => false,
        };
    }

    pub fn Update(&mut self, window: Rc<RefCell<glfw::Window>>, renderer : Rc<RefCell<SimpleRenderer>>, camera : &Camera, deltaTime : f64) -> () {
        for s in &self.m_States {
            s.borrow_mut().Update(window.clone());
            s.borrow_mut().Draw(window.clone(), renderer.clone(), camera, deltaTime);
        }
    }

    pub fn GetCurrentState(&mut self) -> StateRef {
        // unwrap probably shouldnt be used here
        self.m_States.last().unwrap().clone()
    }
}
