use crate::Renderer::Renderer::Renderer;
use crate::Renderer::IRenderer::IRenderer;
use crate::State::State;
use crate::StateMachine::StateMachine;
use crate::CameraController::CameraController;

use std::cell::RefCell;
use std::rc::Rc;
type StateRef = Rc<RefCell<dyn State>>;

use crate::{OpenGLContext::*};
// use spdlog::prelude::*;

pub struct App {
    m_DeltaTime: f64,
    m_LastTime : f64,
    m_IsRunning: bool,
    m_StateMachine: Rc<RefCell<StateMachine>>,
    m_Context: Rc<RefCell<NovaContext>>,
    m_Renderer: Rc<RefCell<IRenderer>>,
    m_CameraController : Rc<RefCell<CameraController>>,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        println!("app new here");

        unsafe {

            App {
                m_DeltaTime: 0.0,
                m_LastTime : 0.0,
                m_IsRunning: true,
                m_StateMachine: Rc::new(RefCell::new(StateMachine::new())),
                m_Context: Rc::new(RefCell::new(NovaContext::new(title, width, height))),
                m_Renderer: Rc::new(RefCell::new(IRenderer::new())),
                m_CameraController : Rc::new(RefCell::new(CameraController::new(width as f32, height as f32))),
            }
        }

        
    }

    pub fn Update(&mut self) {
        // info!("App updating...");
        // self.m_StateMachine.borrow_mut().Update(self.m_Context.borrow_mut().GetWindow());
    }

    pub fn IsRunning(&self) -> bool {
        self.m_IsRunning
    }

    pub fn GetStateMachine(&self) -> Rc<RefCell<StateMachine>> {
        self.m_StateMachine.clone()
    }

    pub fn Run(&mut self, initialState: StateRef) -> () {
        self.m_Context
            .borrow_mut()
            .SetClearColor(0.2, 0.3, 0.3, 1.0);

        self.m_StateMachine.borrow_mut().PushState(initialState);
        

        loop {
            let currentState = self.GetStateMachine().borrow_mut().GetCurrentState();

            self.m_Context
                .borrow_mut()
                .HandleInput(currentState.clone());

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

            //=-----------------------------------------------------------------

            if currentState.borrow_mut().ShouldTransition() {
                // push next state
                let nextState = currentState.borrow_mut().GetNextState();
                currentState.borrow_mut().OnExit();
                let mut sm = self.m_StateMachine.borrow_mut();
                sm.PopState();
                sm.PushState(nextState);
            } else {

                println!("app update here");
                let currentTime = self.m_Context.borrow().GetTime();
                self.m_DeltaTime = currentTime - self.m_LastTime;
                self.m_LastTime = currentTime;
                //----------------------------update is here
                self.m_Context.borrow_mut().ClearWindow();

                // self.Update();

                self.m_StateMachine.borrow_mut().Update(self.m_Context.borrow_mut().GetWindow(), self.m_Renderer.clone(), self.m_CameraController.borrow_mut().GetCamera(), self.m_DeltaTime);


                // self.m_Renderer.borrow_mut().DrawTriangle();

                self.m_Context.borrow_mut().SwapBuffers();

                println!("app update here2");


                // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
                // -------------------------------------------------------------------------------
            }
        }
    }
}
