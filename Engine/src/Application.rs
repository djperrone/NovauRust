use crate::State::State;
use crate::StateMachine::StateMachine;

use std::cell::RefCell;
use std::rc::Rc;
type StateRef = Rc<RefCell<dyn State>>;

use crate::OpenGLContext::*;
// use spdlog::prelude::*;

pub struct App {
    _m_DeltaTime: f32,
    m_IsRunning: bool,
    m_StateMachine: Rc<RefCell<StateMachine>>,
    m_Context: Rc<RefCell<NovaContext>>,
}

// pub struct Window {
//     _m_Width: f32,
//     _m_Height: f32,
// }

// pub trait Executable
// {
//     fn Create() -> App;
// }

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        App {
            _m_DeltaTime: 0.0,
            m_IsRunning: true,
            m_StateMachine: Rc::new(RefCell::new(StateMachine::new())),
            m_Context: Rc::new(RefCell::new(NovaContext::new(title, width, height))),
        }
    }

    pub fn Update(&mut self) {
        // info!("App updating...");
        self.m_StateMachine.borrow_mut().Update();
    }

    pub fn IsRunning(&self) -> bool {
        self.m_IsRunning
    }

    pub fn GetStateMachine(&self) -> Rc<RefCell<StateMachine>> {
        self.m_StateMachine.clone()
    }

    pub fn Run(&mut self, initialState: StateRef) -> () {
       
        self.m_Context.borrow_mut().SetClearColor(0.2, 0.3, 0.3, 1.0);

        self.m_StateMachine.borrow_mut().PushState(initialState);

        loop {
            self.m_IsRunning = self.m_Context.borrow_mut().HandleInput(
                self.m_StateMachine
                    .borrow_mut()
                    .GetCurrentState()
            );

            let currentState = self.GetStateMachine().borrow_mut().GetCurrentState();

            if !self.IsRunning()
                || !currentState.borrow_mut().IsRunning()
                || self
                    .m_Context
                    .borrow_mut()
                    .GetWindow()
                    .borrow_mut()
                    .should_close()
            {
                println!("shutting down...");
                currentState.borrow_mut().OnExit();
                self.m_Context
                    .borrow_mut()
                    .GetWindow()
                    .borrow_mut()
                    .set_should_close(true);
                break;
            }

            if currentState.borrow_mut().ShouldTransition() {
                // push next state
                let nextState = currentState.borrow_mut().GetNextState();
                currentState.borrow_mut().OnExit();
                let mut sm = self.m_StateMachine.borrow_mut();
                sm.PopState();
                sm.PushState(nextState);
            } else {

                self.m_Context.borrow_mut().ClearWindow();
             
                self.Update();

                self.m_Context.borrow_mut().SwapBuffers();

                // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
                // -------------------------------------------------------------------------------
            }
        }
    }
}

// extern crate glfw;
// use self::glfw::{Action, Context, Key};

// extern crate gl;

// use std::sync::mpsc::Receiver;

// // settings
// const SCR_WIDTH: u32 = 800;
// const SCR_HEIGHT: u32 = 600;

// pub fn Run() {
//     // glfw: initialize and configure
//     // ------------------------------
//     let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
//     glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
//     glfw.window_hint(glfw::WindowHint::OpenGlProfile(
//         glfw::OpenGlProfileHint::Core,
//     ));
//     // #[cfg(target_os = "macos")]
//     // glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

//     // glfw window creation
//     // --------------------
//     let (mut window, events) = glfw
//         .create_window(
//             SCR_WIDTH,
//             SCR_HEIGHT,
//             "LearnOpenGL",
//             glfw::WindowMode::Windowed,
//         )
//         .expect("Failed to create GLFW window");

//     window.make_current();
//     window.set_key_polling(true);
//     window.set_framebuffer_size_polling(true);

//     // gl: load all OpenGL function pointers
//     // ---------------------------------------
//     gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
//  unsafe {
//             gl::ClearColor(0.2, 0.3, 0.3, 1.0);
//             gl::Clear(gl::COLOR_BUFFER_BIT);
//         }

//     // render loop
//     // -----------
//     while !window.should_close() {
//         // events
//         // -----
//         process_events(&mut window, &events);

//         // render
//         // ------
//         unsafe {
//             gl::ClearColor(0.2, 0.3, 0.3, 1.0);
//             gl::Clear(gl::COLOR_BUFFER_BIT);
//         }

//         // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
//         // -------------------------------------------------------------------------------
//         window.swap_buffers();
//          unsafe {
//             gl::ClearColor(0.2, 0.3, 0.3, 1.0);
//             gl::Clear(gl::COLOR_BUFFER_BIT);
//         }

//         glfw.poll_events();
//          unsafe {
//             gl::ClearColor(0.2, 0.3, 0.3, 1.0);
//             gl::Clear(gl::COLOR_BUFFER_BIT);
//         }

//     }
// }

// // NOTE: not the same version as in common.rs!
// fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
//     for (_, event) in glfw::flush_messages(events) {
//         match event {
//             glfw::WindowEvent::FramebufferSize(width, height) => {
//                 // make sure the viewport matches the new window dimensions; note that width and
//                 // height will be significantly larger than specified on retina displays.
//                 unsafe { gl::Viewport(0, 0, width, height) }
//             }
//             glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
//                 window.set_should_close(true)
//             }
//             _ => {}
//         }
//     }
// }
