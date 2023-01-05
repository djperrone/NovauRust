// mod Engine::StateMachine;

use Engine::State::State;
use Engine::Input::InputController;

use std::cell::RefCell;
use std::rc::Rc;
use spdlog::prelude::*;


pub struct Level {
    difficulty: i32,
    m_ShouldTransition : bool,
    m_IsRunning : bool,
    m_InputController : Rc<RefCell<InputController>>,
    m_Count : i32,
}

impl Level {
    pub fn new(dif: i32) -> Self {
        Level { difficulty: dif,
                m_ShouldTransition : false,
                m_IsRunning : true,
                m_InputController : Rc::new(RefCell::new(InputController::new())),
                m_Count : 10000,
        }
    }
}
impl State for Level {
    fn Update(&mut self) {
        info!("Level update difficult {}", self.difficulty);
        self.m_Count -= 1;
        if self.m_Count == 0
        {
            self.m_IsRunning = false;
        }
    }

    fn OnEnter(&mut self) -> bool {
        self.m_InputController.borrow_mut().BindEvent("test".into(), Box::new(|| info!("this is a input test")));
        true
    }

    fn HandleInput(&mut self) {
        // self.m_InputController.borrow_mut().Update();
    }

    fn ShouldTransition(&self) -> bool {
        self.m_ShouldTransition
    }

    fn IsRunning(&self) -> bool {
        
        self.m_IsRunning
    }
}
