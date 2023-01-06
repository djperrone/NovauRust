#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use beryllium::fermium::SDL_Event;
use beryllium::*;
// use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
// use ogl33::*;

use crate::Input::InputController;

pub struct Context {
    m_SDL: Rc<RefCell<SDL>>,
    _m_Window: Rc<RefCell<GlWindow>>,
    _m_Width: u32,
    _m_Height: u32,
    _m_Title: String,
}

pub fn InitSDL() -> Rc<RefCell<SDL>> {
    let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");

    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core)
        .unwrap();
    return Rc::new(RefCell::new(sdl));
}

pub fn InitWindow(
    title: &str,
    width: u32,
    height: u32,
    sdl: Rc<RefCell<SDL>>,
) -> Rc<RefCell<GlWindow>> {
    let win = sdl
        .borrow_mut()
        .create_gl_window(
            title,
            WindowPosition::Centered,
            width,
            height,
            WindowFlags::Shown,
        )
        .expect("couldn't make a window and context");

    return Rc::new(RefCell::new(win));
}

impl Context {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let sdl = InitSDL();
        let win = InitWindow(title, width, height, sdl.clone());
        Context {
            _m_Title: title.into(),
            _m_Width: width,
            _m_Height: height,
            m_SDL: sdl,
            _m_Window: win,
        }
    }

    pub fn HandleInput(&mut self, inputController: Option<Rc<RefCell<InputController>>>) -> bool {
        let mut running = true;
        let ic = Rc::new(RefCell::new(inputController));

        
        while let Some(event) = self.m_SDL.borrow_mut().poll_events().and_then(Result::ok) {
            match event {
                Event::Quit(_) => running = false,

                Event::Keyboard(KeyboardEvent {
                    is_pressed,
                    key: KeyInfo { keycode, .. },
                    // repeat : 1,
                    ..
                }) => {
                    if is_pressed {
                        
                        // println!("a {:?}", keycode);
                        match ic.borrow_mut().as_ref()
                        {
                            Some(icr) =>{

                                icr.borrow_mut().HandleEvent(keycode, is_pressed);
                            } ,
                            None => ()
                        }
                        

                    } else {
                       // possibly imlement key up event here
                    }
                }
                _ => (),
            }
        }
        return running;
    }
}

// pub fn Run() -> () {
//     let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");
//     sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
//     sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
//     sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core)
//         .unwrap();
//     #[cfg(target_os = "macos")]
//     {
//         sdl.gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
//             .unwrap();
//     }

//     let _win = sdl
//         .create_gl_window(
//             "Hello Window",
//             WindowPosition::Centered,
//             800,
//             600,
//             WindowFlags::Shown,
//         )
//         .expect("couldn't make a window and context");

//     'main_loop: loop {
//         // handle events this frame
//         while let Some(event) = sdl.poll_events().and_then(Result::ok) {
//             match event {
//                 Event::Quit(_) => break 'main_loop,
//                 _ => (),
//             }
//         }
//     }
// }
