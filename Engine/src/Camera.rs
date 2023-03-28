// use cgmath::ortho;

use crate::Novaura::{ConvertMatToGLM, Identity};

use glm::*;
pub struct Camera
{
    m_ProjectionMatrix : glm::Mat4 ,
	m_ViewMatrix : glm::Mat4 ,
    m_ViewProjectionMatrix : glm::Mat4,

    m_Position : glm::Vec3 , //{ 0.0f,0.0f,0.0f }
	m_Rotation : f32, // 0.0f;
}

impl Camera
{
    pub fn new(left : f32 , right : f32, bottom : f32 , top : f32 ) -> Self
    {
        let projMat1 = nalgebra_glm::ortho(left, right, bottom, top, -1.0, 1.0);
        // let glm_mat : glm::Mat4 = ConvertMatToGLM(projMat);
        
        let projMat = ConvertMatToGLM(projMat1);

        println!("nalg{:?}", projMat1);
        println!("glm{:?}", projMat);
        // let v : glm::Mat4 = 1.0;

        let viewMat = glm::Mat4::new(
            glm::Vec4::new(1.0, 0.0, 0.0, 0.0),
            glm::Vec4::new(0.0, 1.0, 0.0, 0.0),
            glm::Vec4::new(0.0, 0.0, 1.0, 0.0),
            glm::Vec4::new(0.0, 0.0, 0.0, 1.0),
        );
        let viewProjMat = projMat * viewMat;

        Camera {
            
            m_ProjectionMatrix :projMat,
            m_ViewMatrix : viewMat,
            m_ViewProjectionMatrix : viewProjMat,
            m_Position : glm::Vec3::new(0.0,0.0,0.0),
            m_Rotation : 0.0,
        }
    }

		pub fn SetProjection(&mut self, left : f32 , right :  f32 , bottom :  f32 , top : f32 )
        {
            self.m_ProjectionMatrix = ConvertMatToGLM(nalgebra_glm::ortho(left, right, bottom, top, -1.0, 1.0));
		    self.m_ViewProjectionMatrix = self.m_ProjectionMatrix * self.m_ViewMatrix;
            
        }

		pub fn  GetPosition(&self) -> &glm::Vec3
        {
            return &self.m_Position;
        }

		pub fn SetPosition(&mut self, position : &glm::Vec3 ) 
        { 
            self.m_Position = *position; 
            self.RecalculateViewMatrix(); 
        }

		pub fn GetRotation(&self) -> f32 { return self.m_Rotation; }
		pub fn SetRotation(&mut self, rotation : f32) -> (){ self.m_Rotation = rotation; self.RecalculateViewMatrix(); }


		pub fn GetProjectionMatrix(&self ) -> &glm::Mat4  { return &self.m_ProjectionMatrix; }
		pub fn GetViewMatrix(&self) -> &glm::Mat4 { return &self.m_ViewMatrix; }
		pub fn GetViewProjectionMatrix(&self)-> &glm::Mat4 { return &self.m_ViewProjectionMatrix; }

	
		pub fn RecalculateViewMatrix(&mut self) -> ()
        {
            let transform : glm::Mat4 = glm::ext::translate(&glm::Mat4::new(
                glm::Vec4::new(1.0, 0.0, 0.0, 0.0),
                glm::Vec4::new(0.0, 1.0, 0.0, 0.0),
                glm::Vec4::new(0.0, 0.0, 1.0, 0.0),
                glm::Vec4::new(0.0, 0.0, 0.0, 1.0),
            ), self.m_Position) * glm::ext::rotate(&Identity(), glm::radians(self.m_Rotation), glm::Vec3::new(0., 0., 1.));

		    self.m_ViewMatrix = glm::inverse(&transform);
		    self.m_ViewProjectionMatrix = self.m_ProjectionMatrix * self.m_ViewMatrix;
        }
}