// mod Engine::StateMachine;

use Engine::Input::InputController;
use Engine::Input::NovaEvent;
use Engine::State::State;

use spdlog::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
// use Berillium::Keycode;
// use beryllium::Keycode;
extern crate glfw;
use glfw::WindowEvent;
use std::sync::mpsc::Receiver;

pub struct Level {
    difficulty: i32,
    m_ShouldTransition: bool,
    m_IsRunning: bool,
    m_InputController: Rc<RefCell<InputController>>,
    m_Count: i32,
}

impl Level {
    pub fn new(dif: i32) -> Self {
        Level {
            difficulty: dif,
            m_ShouldTransition: false,
            m_IsRunning: true,
            m_InputController: Rc::new(RefCell::new(InputController::new())),
            m_Count: 1000000,
        }
    }
}
impl State for Level {
    fn Update(&mut self) {
        self.m_Count -= 1;
        if self.m_Count == 0 {
            info!("Level update difficult {}", self.difficulty);
            self.m_IsRunning = false;
        }
    }

    fn OnEnter(&mut self) -> bool {
        //self.m_InputController
        //.borrow_mut()
        //.BindEvent(glfw::Key::A, Box::new(|| info!("A was pressed")));
        self.m_InputController.borrow_mut().BindEvent(
            glfw::Key::W,
            NovaEvent::newAxisEvent(Box::new(|| info!("W was pressed axis"))),
        );
        self.m_InputController.borrow_mut().BindEvent(
            glfw::Key::S,
            NovaEvent::newActionEvent(Box::new(|| info!("S was pressed action"))),
        );
        // self.m_InputController.borrow_mut().BindEvent(Keycode::W, Box::new(|| info!("W was pressed")));
        // self.m_InputController.borrow_mut().BindEvent(Keycode::S, Box::new(|| info!("S was pressed")));
        true
    }

    // fn HandleInput(&mut self, window: Rc<RefCell<glfw::Window>>, event: glfw::WindowEvent) {
    //     // self.m_InputController.borrow_mut().Update();
    // }

    fn HandleKeyBoardInput(
        &mut self,
        window: Rc<RefCell<glfw::Window>>,
        key: glfw::Key,
        action: glfw::Action,
        modifiers: glfw::Modifiers,
    ) {
        match window.borrow_mut().get_key(glfw::Key::W) {
            glfw::Action::Press => println!("w was pressed axis"),
            _ => (),
        }

        match key {
            glfw::Key::A => match action {
                glfw::Action::Press => println!("A was pressed action"),
                _ => (),
            },
            _ => (),
        }
    }

    fn HandleMouseInput(
        &mut self,
        window: Rc<RefCell<glfw::Window>>,
        button: glfw::MouseButton,
        action: glfw::Action,
        modifiers: glfw::Modifiers,
    ) {
    }

    fn ShouldTransition(&self) -> bool {
        self.m_ShouldTransition
    }

    fn IsRunning(&self) -> bool {
        self.m_IsRunning
    }

    fn GetInputController(&self) -> Option<Rc<RefCell<InputController>>> {
        Some(self.m_InputController.clone())
    }
}
