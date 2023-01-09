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
    m_Context: Rc<RefCell<NovaContext>>,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        App {
            _m_DeltaTime: 0.0,
            m_IsRunning: true,
            m_StateMachine: Rc::new(RefCell::new(StateMachine::new())),
            m_Context: Rc::new(RefCell::new(NovaContext::new(title, width, height))),
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
        self.m_Context
            .borrow_mut()
            .SetClearColor(0.2, 0.3, 0.3, 1.0);

        self.m_StateMachine.borrow_mut().PushState(initialState);

        loop {
            let currentState = self.GetStateMachine().borrow_mut().GetCurrentState();

            self.m_Context
                .borrow_mut()
                .HandleInput(currentState.clone());

            if !self.IsRunning()
                || !currentState.borrow_mut().IsRunning()
                || self
                    .m_Context
                    .borrow_mut()
                    .GetWindow()
                    .borrow_mut()
                    .should_close()
            {
                println!("shutting down...");
                currentState.borrow_mut().OnExit();
                self.m_Context
                    .borrow_mut()
                    .GetWindow()
                    .borrow_mut()
                    .set_should_close(true);
                break;
            }

            if currentState.borrow_mut().ShouldTransition() {
                // push next state
                let nextState = currentState.borrow_mut().GetNextState();
                currentState.borrow_mut().OnExit();
                let mut sm = self.m_StateMachine.borrow_mut();
                sm.PopState();
                sm.PushState(nextState);
            } else {
                self.m_Context.borrow_mut().ClearWindow();

                self.Update();

                self.m_Context.borrow_mut().SwapBuffers();

                // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
                // -------------------------------------------------------------------------------
            }
        }
    }
}
