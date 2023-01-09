use std::cell::RefCell;
use std::rc::Rc;

extern crate glfw;
use glfw::WindowEvent;

use self::glfw::{Action, Context, Key};

extern crate gl;

use std::sync::mpsc::Receiver;

use crate::Input::InputController;
use crate::State::State;
type StateRef = Rc<RefCell<dyn State>>;

pub struct NovaContext {
    m_GLFW: Rc<RefCell<glfw::Glfw>>,
    m_Window: Rc<RefCell<glfw::Window>>,
    m_Receiver: Rc<RefCell<Receiver<(f64, WindowEvent)>>>,
    m_Width: u32,
    m_Height: u32,
    m_Title: String,
}

impl NovaContext {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        // let sdl = InitSDL();
        // let win = InitWindow(title, width, height, sdl.clone());
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        // #[cfg(target_os = "macos")]
        // glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // gl: load all OpenGL function pointers
        // ---------------------------------------
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        //  unsafe {
        //         gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        //         // gl::Clear(gl::COLOR_BUFFER_BIT);
        //     }
        NovaContext {
            m_GLFW: Rc::new(RefCell::new(glfw)),
            m_Window: Rc::new(RefCell::new(window)),
            m_Receiver: Rc::new(RefCell::new(events)),
            m_Width: width,
            m_Height: height,
            m_Title: title.into(),
        }
    }

    pub fn HandleInput(&mut self, currentState: StateRef) -> bool {
        let mut running = true;
        // let ic = Rc::new(RefCell::new(inputController));
        // for (key, event) in inputController.
        self.m_GLFW.borrow_mut().poll_events();

        self.ProcessEvents(self.m_Window.clone(), self.m_Receiver.clone(), currentState);

        // match inputController
        // {
        //     Some(ic) => {

        //         // for( key, event) in ic.borrow_mut().
        //         ic.borrow_mut().HandleAxisEvents(self.m_Window.clone());
        //         running = self.ProcessEvents(self.m_Window.clone(), self.m_Receiver.clone(), ic.clone());
        //     },
        //     None => (),
        // }

        return running;
    }

    fn ProcessEvents(
        &mut self,
        window: Rc<RefCell<glfw::Window>>,
        events: Rc<RefCell<Receiver<(f64, glfw::WindowEvent)>>>,
        currentState: StateRef,
    ) -> bool {
        // self.m_Window.borrow_mut().get_key(Key::A)
        let mut running = true;
        for (_, event) in glfw::flush_messages(&events.borrow_mut()) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe { gl::Viewport(0, 0, width, height) }
                    self.m_Width = width as u32;
                    self.m_Height = height as u32;

                    println!("{} {},", width, height);
                }

                // glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                //     // self.m_Window.borrow_mut().set_should_close(true);
                //     running = false;
                // }

                // glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                //     // self.m_Window.borrow_mut().set_should_close(true);
                //     // running = false;
                //     println!("A was pressed here");
                // }
                glfw::WindowEvent::Key(key, _, action, modifiers) => {
                    // inputController.borrow_mut().HandleActionEvent(key, Action::Press, None)
                    currentState.borrow_mut().HandleKeyBoardInput(
                        window.clone(),
                        key,
                        action,
                        modifiers,
                    );
                }

                glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                    currentState.borrow_mut().HandleMouseInput(
                        window.clone(),
                        button,
                        action,
                        modifiers,
                    );
                }

                // glfw::WindowEvent::Key(key,_ ,Action::Release , _ ) =>{

                //     inputController.borrow_mut().HandleActionEvent(key, Action::Release, None)
                // }

                //    glfw::WindowEvent::Key(key,_ ,Action::Repeat , _ ) =>{

                //     inputController.borrow_mut().HandleActionEvent(key, Action::Repeat, None)
                //}

                // glfw::WindowEvent::Key(, , , )
                _ => {}
            }
        }
        return running;
    }

    pub fn GetWindow(&mut self) -> Rc<RefCell<glfw::Window>> {
        self.m_Window.clone()
    }

    pub fn SwapBuffers(&mut self) -> () {
        self.m_Window.borrow_mut().swap_buffers();
    }

    pub fn ClearWindow(&mut self) -> () {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn SetClearColor(&mut self, r: f32, g: f32, b: f32, a: f32) -> () {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }
}
