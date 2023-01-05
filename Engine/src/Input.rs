
#![allow(non_snake_case)]



// mod Input{
use std::collections::HashMap;
type Event = dyn Fn() -> ();
// // temporary impl before introducing opengl
pub fn IsPressed(key : &String) -> bool
{
    match key.as_str(){
        "test" => true,
        "test0" => true,
        "test1" => true,
        "test2" => true,
        "test123" => true,
        _ => false
    }
}

pub struct InputController {
    keyBindings: HashMap<String, Box<Event>>,
}

impl InputController {
    pub fn new() -> InputController {
        InputController {
            keyBindings: HashMap::new(),
        }
    }

    pub fn BindEvent(&mut self, key: String, event: Box<Event>) {
        self.keyBindings.insert(key, event);
    }

    pub fn Update(&self) {
        // self.keyBindings.iter().for_each(|(key, event)|{
        //     if IsPressed(key){
        //          event()}})

        for (key, event) in &self.keyBindings {
            if IsPressed(key) {
                event();
            }
        }
    }
}


// }