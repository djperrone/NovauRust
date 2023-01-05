use std::cell::RefCell;
use std::rc::Rc;

// use Engine::InputController::InputController;

// mod Input;
use crate::State::State;

type StateRef = Rc<RefCell<dyn State>>;


pub struct StateMachine {
    m_States: Vec<StateRef>,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            m_States: Vec::new(),
        }
    }

    pub fn PushState(&mut self, s: StateRef) -> () {
        // self.m_States.push(s);
        // self.m_States.last_mut().unwrap().clone().borrow_mut().OnEnter();

        self.m_States.push(s);
        let s2 = self.m_States.last();
        match s2 {
            Some(s2) => s2.borrow_mut().OnEnter(),
            None => false,
        };
    }

    pub fn PopState(&mut self) -> () {
        // if self.m_States.len() != 0 {
        //     let last = self.m_States.pop();
        //     // last.unwrap().borrow_mut().OnExit();
        // }

        let last = self.m_States.pop();

        match last {
            Some(last) => last.borrow_mut().OnExit(),
            None => false,
        };
    }

    pub fn Update(&mut self) -> () {
        for s in &self.m_States {
            // s.as_ref().borrow_mut().HandleInput();
            s.borrow_mut().Update();
            s.borrow_mut().Draw();
        }
    }

    pub fn GetCurrentState(&mut self) -> StateRef {
        self.m_States.last().unwrap().clone()
    }
}
