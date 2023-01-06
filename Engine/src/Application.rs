use crate::State::State;
use crate::StateMachine::StateMachine;

use std::cell::RefCell;
use std::rc::Rc;
type StateRef = Rc<RefCell<dyn State>>;

use crate::OpenGLContext::*;
// use spdlog::prelude::*;

pub struct App {
    _m_DeltaTime: f32,
    m_IsRunning: bool,
    m_StateMachine: Rc<RefCell<StateMachine>>,
    m_Context: Rc<RefCell<Context>>,
}

// pub struct Window {
//     _m_Width: f32,
//     _m_Height: f32,
// }

// pub trait Executable
// {
//     fn Create() -> App;
// }

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        App {
            _m_DeltaTime: 0.0,
            m_IsRunning: true,
            m_StateMachine: Rc::new(RefCell::new(StateMachine::new())),
            m_Context: Rc::new(RefCell::new(Context::new(title, width, height))),
        }
    }

    pub fn Update(&mut self) {
        // info!("App updating...");
        self.m_StateMachine.borrow_mut().Update();
    }

    pub fn IsRunning(&self) -> bool {
        self.m_IsRunning
    }

    pub fn GetStateMachine(&self) -> Rc<RefCell<StateMachine>> {
        self.m_StateMachine.clone()
    }

    pub fn Run(&mut self, initialState: StateRef) -> () {
        self.m_StateMachine.borrow_mut().PushState(initialState);

        loop {
            self.m_IsRunning = self.m_Context.borrow_mut().HandleInput(
                self.m_StateMachine
                    .borrow_mut()
                    .GetCurrentState()
                    .borrow_mut()
                    .GetInputController(),
            );

            let currentState = self.GetStateMachine().borrow_mut().GetCurrentState();

            if !self.IsRunning() || !currentState.borrow_mut().IsRunning() {
                currentState.borrow_mut().OnExit();
                break;
            }

            if currentState.borrow_mut().ShouldTransition() {
                // push next state
                let nextState = currentState.borrow_mut().GetNextState();

                let mut sm = self.m_StateMachine.borrow_mut();
                sm.PopState();
                sm.PushState(nextState);
            } else {
                self.Update();
            }
        }
    }
}
