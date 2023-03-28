use std::{cell::RefCell, rc::Rc};

use crate::Camera::Camera;



pub struct CameraBounds
{
    left : f32,
    right : f32, 
    bottom : f32,
    top : f32
}

impl CameraBounds
{
    pub fn new(l :f32, r : f32, b : f32, t : f32) -> Self
    {
        CameraBounds
        {
            left : l,
            right : r,
            bottom : b,
            top : t,
        }
    }
    pub fn GetWidth(&self) -> f32
    {
        return self.right - self.left;
    }

    pub fn GetHeight(&self) -> f32
    {
        return self.top - self.bottom;
    }
}



pub struct CameraController
{
    m_AspectRatio : f32,
    m_ZoomLevel  : f32, //= 1.0f;
    m_Bounds : CameraBounds,
    m_Camera : Camera,

    _m_Rotation : bool,

    m_CameraPosition : glm::Vec3,// = { 0.0f, 0.0f, 0.0f };
    m_CameraRotation : f32, // = 0.0f;
    m_CameraTranslationSpeed : f32, // = 1.0f, m_CameraRotationSpeed = 180.0f;
}

impl CameraController
{
    pub fn new(width : f32, height : f32) -> Self
    {
        println!("camera controller new here");

        let zoomLevel = 1.0f32;
        let aspectRatio : f32 = width / height;
        let bounds = CameraBounds::new(-aspectRatio * zoomLevel, aspectRatio * zoomLevel, -zoomLevel, zoomLevel);

        CameraController
        {
            m_AspectRatio : width / height,
            m_ZoomLevel  : 1.0, //= 1.0f;
            m_Bounds : bounds,
            m_Camera : Camera::new(-aspectRatio * zoomLevel, aspectRatio * zoomLevel, -zoomLevel, zoomLevel),
        
            _m_Rotation : false,
        
            m_CameraPosition : glm::Vec3::new(0.0, 0.0, 0.0),// = { 0.0f, 0.0f, 0.0f };
            m_CameraRotation : 0.0, // = 0.0f;
            m_CameraTranslationSpeed : 1.0, // = 1.0f, 
        }
    }

    pub fn Update(&mut self, window : Rc<RefCell<glfw::Window>>, deltaTime : f32)-> ()
    {
        match window.borrow_mut().get_key(glfw::Key::Left)  {
            glfw::Action::Press => {
                self.m_CameraPosition.x -= glm::cos(glm::radians(self.m_CameraRotation)) * self.m_CameraTranslationSpeed * deltaTime;
			    self.m_CameraPosition.y -= glm::sin(glm::radians(self. m_CameraRotation)) * self.m_CameraTranslationSpeed * deltaTime;
            },
            _ => (),
        }

        match window.borrow_mut().get_key(glfw::Key::Right)  {
            glfw::Action::Press => {
                self.m_CameraPosition.x += glm::cos(glm::radians(self.m_CameraRotation)) * self.m_CameraTranslationSpeed * deltaTime;
			    self.m_CameraPosition.y += glm::sin(glm::radians(self.m_CameraRotation)) * self.m_CameraTranslationSpeed * deltaTime;
            },
            _ => (),
        }

        match window.borrow_mut().get_key(glfw::Key::Up)  {
            glfw::Action::Press => {
                self.m_CameraPosition.x += -glm::sin(glm::radians(self.m_CameraRotation)) * self.m_CameraTranslationSpeed * deltaTime;
			    self.m_CameraPosition.y += glm::cos(glm::radians(self.m_CameraRotation)) *self. m_CameraTranslationSpeed * deltaTime;
            },
            _ => (),
        }

        match window.borrow_mut().get_key(glfw::Key::Down)  {
            glfw::Action::Press => {
                self.m_CameraPosition.x -= -glm::sin(glm::radians(self.m_CameraRotation)) *self. m_CameraTranslationSpeed * deltaTime;
			    self.m_CameraPosition.y -= glm::cos(glm::radians(self.m_CameraRotation)) * self.m_CameraTranslationSpeed * deltaTime;
            },
            _ => (),
        }
		
        self.m_Camera.SetPosition(&self.m_CameraPosition);

		self.m_CameraTranslationSpeed = self.m_ZoomLevel;

        self.m_Camera.RecalculateViewMatrix();

    }
		//void OnEvent(Event& e);

		pub fn OnResize(&mut self, width : f32, height : f32)-> ()
        {
            self.m_AspectRatio = width / height;
		    self.m_Bounds = CameraBounds::new(-self.m_AspectRatio * self.m_ZoomLevel, self.m_AspectRatio * self.m_ZoomLevel, -self.m_ZoomLevel, self.m_ZoomLevel);
		    self.m_Camera.SetProjection(self.m_Bounds.left, self.m_Bounds.right, self.m_Bounds.bottom, self.m_Bounds.top);
        }

		pub fn GetCamera(&self) -> &Camera { return &self.m_Camera; }
		// const Camera& GetCamera() const { return m_Camera; }

		pub fn GetBounds(&self ) -> &CameraBounds  { return &self.m_Bounds; }

		pub fn GetZoomLevel(&self)-> f32 { return self.m_ZoomLevel; }
		pub fn SetZoomLevel(&mut self,  level : f32) -> () {self. m_ZoomLevel = level; }
}
