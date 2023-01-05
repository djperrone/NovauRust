use crate::State::State;
use crate::StateMachine::StateMachine;

use std::cell::RefCell;
use std::rc::Rc;
type StateRef = Rc<RefCell<dyn State>>;

pub struct App {
    _m_DeltaTime: f32,
    m_IsRunning: bool,
    _m_Window: Window,
    m_StateMachine: Rc<RefCell<StateMachine>>,
    _m_Title: String,
}

pub struct Window {
    _m_Width: f32,
    _m_Height: f32,
}

// pub trait Executable
// {
//     fn Create() -> App;
// }

impl App {
    pub fn new(title: &str, width: f32, height: f32) -> Self {
        App {
            _m_DeltaTime: 0.0,
            m_IsRunning: true,
            _m_Window: Window {
                _m_Width: width,
                _m_Height: height,
            },
            m_StateMachine: Rc::new(RefCell::new(StateMachine::new())),
            _m_Title: String::from(title),
        }
    }

    pub fn Update(&mut self) {
        println!("App updating...");
        self.m_StateMachine.as_ref().borrow_mut().Update();
    }

    pub fn IsRunning(&self) -> bool {
        self.m_IsRunning
    }

    pub fn GetStateMachine(&self) -> Rc<RefCell<StateMachine>> {
        self.m_StateMachine.clone()
    }

    pub fn Run(&mut self, initialState: StateRef) -> () {
        self.GetStateMachine().borrow_mut().PushState(initialState);

        // temp way to stop the running loop
        let mut n = 5;
        while self.IsRunning() {
            n -= 1;
            if n == 0 {
                break;
            }

            if self
                .GetStateMachine()
                .borrow_mut()
                .GetCurrentState()
                .borrow_mut()
                .ShouldTransition()
            {
                // push next state
                let nextState = self
                    .GetStateMachine()
                    .borrow_mut()
                    .GetCurrentState()
                    .borrow_mut()
                    .GetNextState();

                self.GetStateMachine().as_ref().borrow_mut().PopState();

                self.GetStateMachine().borrow_mut().PushState(nextState);
            } else {
                self.Update();
            }
        }
    }
}
