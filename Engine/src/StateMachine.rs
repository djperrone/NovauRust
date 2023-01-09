use crate::State::State;
use std::cell::RefCell;
use std::rc::Rc;

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

    pub fn Update(&mut self) -> () {
        for s in &self.m_States {
            s.borrow_mut().Update();
            s.borrow_mut().Draw();
        }
    }

    pub fn GetCurrentState(&mut self) -> StateRef {
        // unwrap probably shouldnt be used here
        self.m_States.last().unwrap().clone()
    }
}
