// mod Engine::StateMachine;

use Engine::State::State;

pub struct Level {
    difficulty: i32,
}

impl Level {
    pub fn new(dif: i32) -> Self {
        Level { difficulty: dif }
    }
}
impl State for Level {
    fn Update(&mut self) {
        println!("Level update difficult {}", self.difficulty);
    }
}
