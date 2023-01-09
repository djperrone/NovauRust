#![allow(non_snake_case)]

// use beryllium::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

extern crate glfw;

type Event = dyn Fn() -> ();

pub enum NovaEvent {
    Action {
        // isPressed: bool,
        // isProcessed: bool,
        event: Box<Event>,
    },
    Axis {
        event: Box<Event>,
    },
}

impl NovaEvent {
    pub fn newActionEvent(e: Box<Event>) -> NovaEvent {
        NovaEvent::Action {
            event: e,
        }
    }

    pub fn newAxisEvent(e: Box<Event>) -> NovaEvent {
        NovaEvent::Axis { event: e }
    }
}

// // temporary impl before introducing opengl
pub fn IsPressed(key: &String) -> bool {
    match key.as_str() {
        "test" => true,
        "test0" => true,
        "test1" => true,
        "test2" => true,
        "test123" => true,
        _ => false,
    }
}

pub struct InputController {
    m_Bindings: HashMap<glfw::Key, NovaEvent>,
    // m_ActionBindings: HashMap<Keycode, NovaEvent>,
}

impl InputController {
    pub fn new() -> InputController {
        InputController {
            // m_AxisBindings: HashMap::new(),
            m_Bindings: HashMap::new(),
        }
    }

    pub fn BindEvent(&mut self, key: glfw::Key, e: NovaEvent) {
        self.m_Bindings.insert(key, e);
        // match e
        // {
        // NovaEvent::Axis{event} => self.m_AxisBindings.insert(key, e),
        // NovaEvent::Axis{event} => self.m_AxisBindings.insert(key, e),
        // NovaEvent::Action{event, isPressed, isProcessed} => self.m_Bindings.insert(key, e),
        // _ => ()
        // };
    }

    pub fn HandleAxisEvents(&mut self, window: Rc<RefCell<glfw::Window>>)
    {
        for (key, event) in &self.m_Bindings
        {
            match event
            {
                NovaEvent::Axis { event } => 
                {
                    match window.borrow_mut().get_key(*key) 
                    {
                        glfw::Action::Press => event(),
                        _ => (),
                    }
                },

                _ => (),
            }
        }
    }

    pub fn HandleActionEvent(&mut self, key : glfw::Key, action : glfw::Action, modifiers : Option<glfw::Modifiers>) 
    {
        match self.m_Bindings.get(&key)
        {
            Some(k) => 
            {
                match k
                {
                    NovaEvent::Action { event }=>
                    {
                        event();
                    },

                    _ => (),
                }
            },

            None => (),
        }
       
    }

    pub fn Update(&self) {
        // self.keyBindings.iter().for_each(|(key, event)|{
        //     if IsPressed(key){
        //          event()}})

        // for (key, event) in &self.keyBindings {
        //     if IsPressed(key) {
        //         event();
        //     }
        // }
    }

    // pub fn HandleEvent(&mut self, keycode: Keycode, sdlPressed : bool) {

    //     self.m_Bindings.entry(keycode).and_modify(|binding| match binding{
    //         NovaEvent::Axis { event } => event(),
    //         NovaEvent::Action { isPressed, isProcessed, event } =>{
    //             // println!("testing");
    //             match isPressed {
    //                 false =>
    //                 {
    //                     *isPressed = true;
    //                     event();
    //                 },
    //                 true => (),
    //                 _ => ()
    //             }
    //         }
    //         _ => ()

    //     });
    // if self.keyBindings.contains_key(keycode)
    // match self.m_Bindings.get(&keycode) {
    //     Some(e) => {
    //         match e {
    //             NovaEvent::Axis{event,..} => event(),
    //             NovaEvent::Action{event, mut isPressed, isProcessed} =>
    //             {
    //                 if sdlPressed
    //                 {
    //                     event();
    //                     // *isProcessed = true;
    //                     isPressed = true;
    //                 }
    //             },
    //             _ => ()
    //         }
    //         },
    //     None => (),
    // }
    // match self.m_ActionBindings.get(&keycode) {
    //     Some(event) => {

    //         event()},
    //     None => (),
    // }
    // }
}

// }
