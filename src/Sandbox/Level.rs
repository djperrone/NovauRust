// mod Engine::StateMachine;

use Engine::State::State;
use Engine::Input::InputController;
use Engine::Input::NovaEvent;

use std::cell::RefCell;
use std::rc::Rc;
use spdlog::prelude::*;
// use Berillium::Keycode;
use beryllium::Keycode;



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
                m_Count : 1000000,
        }
    }
}
impl State for Level {
    fn Update(&mut self) {
        self.m_Count -= 1;
        if self.m_Count == 0
        {
            info!("Level update difficult {}", self.difficulty);
            self.m_IsRunning = false;
        }
    }

    fn OnEnter(&mut self) -> bool {
        self.m_InputController.borrow_mut().BindEvent(Keycode::A, NovaEvent::newAxisEvent(Box::new(|| info!("A was pressed"))));
        self.m_InputController.borrow_mut().BindEvent(Keycode::W, NovaEvent::newAxisEvent(Box::new(|| info!("W was pressed"))));
        self.m_InputController.borrow_mut().BindEvent(Keycode::S, NovaEvent::newActionEvent(Box::new(|| info!("S was pressed"))));
        // self.m_InputController.borrow_mut().BindEvent(Keycode::W, Box::new(|| info!("W was pressed")));
        // self.m_InputController.borrow_mut().BindEvent(Keycode::S, Box::new(|| info!("S was pressed")));
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

      fn GetInputController(&self) -> Option<Rc<RefCell<InputController>>>
    {
        Some(self.m_InputController.clone())
    }
}
