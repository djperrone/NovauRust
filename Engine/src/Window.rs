use std::{cell::RefCell, rc::Rc};



pub struct Window
{
    _window : Rc<RefCell<glfw::Window>>,
    width  : f32, //= 800.0f;
    height : f32, // = 600.0f;
    // AspectRatio = (float)Width / (float)Height;
}

impl Window
{
    pub fn GetAspectRatio(&self) -> f32
    {
        self.width / self.height
    }
}