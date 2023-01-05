#![allow(non_snake_case)]

// use Engine::InputController::InputController;

use std::cell::RefCell;
use std::rc::Rc;
use Engine::Application;

mod Sandbox;
use crate::Sandbox::Level::Level;

fn main() {
    let mut app = Application::App::new("test", 800.0, 600.0);
    
    app.Run(Rc::new(RefCell::new(Level::new(5))));

}
