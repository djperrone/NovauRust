

use Engine::State::State;

use spdlog::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
extern crate glfw;

pub struct Level {
    difficulty: i32,
    m_ShouldTransition: bool,
    m_IsRunning: bool,
    m_Count: i32,
}

impl Level {
    pub fn new(dif: i32) -> Self {
        Level {
            difficulty: dif,
            m_ShouldTransition: false,
            m_IsRunning: true,
            m_Count: 1000000,
        }
    }
}
impl State for Level 
{
    fn Update(&mut self, window : Rc<RefCell<glfw::Window>>)
    {
         // axis events
         match window.borrow_mut().get_key(glfw::Key::W) {
            glfw::Action::Press => println!("w was pressed axis"),
            _ => (),
        }

        self.m_Count -= 1;

        if self.m_Count == 0 {
            info!("Level update difficult {}", self.difficulty);
            self.m_IsRunning = false;
        }
    }

    fn OnEnter(&mut self) -> bool {
       
        true
    }
     fn OnExit(&mut self) -> bool {
       
        true
    }

    fn HandleKeyBoardInput(
        &mut self,
        _window: Rc<RefCell<glfw::Window>>,
        key: glfw::Key,
        action: glfw::Action,
        _modifiers: glfw::Modifiers,
    ) {
       
        // action events
        match key
        {
            glfw::Key::Escape => match action 
            {
                glfw::Action::Press => self.m_IsRunning = false,
                _ => ()
            }
            glfw::Key::A => match action {
                glfw::Action::Press => println!("A was pressed action"),
                glfw::Action::Release => println!("A was released action"),
                _ => (),
            },
            glfw::Key::B => match action {
                glfw::Action::Release => println!("B was released action"),
                _ => (),
            },

            _ => (),
        }
    }

    fn HandleMouseInput(
        &mut self,
        _window: Rc<RefCell<glfw::Window>>,
        _button: glfw::MouseButton,
        _action: glfw::Action,
        _modifiers: glfw::Modifiers,
    ) 
    {
        // no mouse input currently used
    }

    fn ShouldTransition(&self) -> bool {
        self.m_ShouldTransition
    }

    fn IsRunning(&self) -> bool {
        self.m_IsRunning
    }
}
