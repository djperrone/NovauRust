#![allow(non_snake_case)]

use beryllium::*;
use std::collections::HashMap;

type Event = dyn Fn() -> ();

pub enum NovaEvent{
    Action{ isPressed : bool, isProcessed : bool, event : Box<Event>},
    Axis {event : Box<Event>},
}

impl NovaEvent
{
    pub fn newActionEvent(e : Box<Event>) -> NovaEvent
    {
        NovaEvent::Action{
            isPressed : false,
            isProcessed : false,
            event : e
        }
    }

    pub fn newAxisEvent(e : Box<Event>) -> NovaEvent
    {
        NovaEvent::Axis{
            event : e
        }
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
    m_Bindings: HashMap<Keycode, NovaEvent>,
    // m_ActionBindings: HashMap<Keycode, NovaEvent>,
}

impl InputController {
    pub fn new() -> InputController {
        InputController {
            // m_AxisBindings: HashMap::new(),
            m_Bindings: HashMap::new(),
        }
    }

    pub fn BindEvent(&mut self, key: Keycode, e : NovaEvent) {
        self.m_Bindings.insert(key, e);
        // match e
        // {
            // NovaEvent::Axis{event} => self.m_AxisBindings.insert(key, e),
            // NovaEvent::Axis{event} => self.m_AxisBindings.insert(key, e),
            // NovaEvent::Action{event, isPressed, isProcessed} => self.m_Bindings.insert(key, e),
            // _ => ()
        // };
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

    pub fn HandleEvent(&mut self, keycode: Keycode, sdlPressed : bool) {
        // if self.keyBindings.contains_key(keycode)
        match self.m_Bindings.get(&keycode) {
            Some(e) => {
                match e {
                    NovaEvent::Axis{event,..} => event(),
                    NovaEvent::Action{event, mut isPressed, isProcessed} =>
                    {
                        if sdlPressed
                        {
                            event();
                            // *isProcessed = true;
                            isPressed = true;
                        }
                    },
                    _ => ()
                }
                },
            None => (),
        }
        // match self.m_ActionBindings.get(&keycode) {
        //     Some(event) => {
                
        //         event()},
        //     None => (),
        // }
    }
}

// }
