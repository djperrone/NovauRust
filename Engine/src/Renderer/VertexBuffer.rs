use std::mem::size_of;
// use std::mem::size_of_val;

extern crate gl;
extern crate glfw;

use gl::types::GLfloat;
use gl::types::GLsizeiptr;
use gl::types::GLuint;
use std::os::raw::c_void;

// use super::VertexData::VertexData;

// crate VertexData;

struct VertexBuffer {
    m_VertexBufferID: GLuint,
}

impl VertexBuffer {
    pub fn new() -> Self {
        unsafe {
            let mut bufferID: GLuint = 0;
            gl::GenBuffers(1, &mut bufferID);

            VertexBuffer {
                m_VertexBufferID: bufferID,
            }
        }
    }
    // ~VertexBuffer();

    // void Bind() const;
    pub unsafe fn Bind(&mut self) -> () {
        
        gl::BindBuffer(gl::ARRAY_BUFFER, self.m_VertexBufferID);
        
    }

    pub unsafe fn UnBind(&mut self) -> () {
        
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    pub unsafe fn SetData(
        &mut self,
        position: &mut glm::Vec3,
        color: &mut glm::Vec4,
        quantity: &mut glm::Vec2,
        textureSlot: f32,
    ) -> () {
        
        let mut data: Vec<f32> = vec![];
        data.append(&mut position.as_array_mut().to_vec());
        data.append(&mut color.as_array_mut().to_vec());
        data.append(&mut quantity.as_array_mut().to_vec());
        data.push(textureSlot);
        
        gl::BindBuffer(gl::ARRAY_BUFFER, self.GetId());

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (data.len() * size_of::<GLfloat>()) as GLsizeiptr,
            &data[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        self.UnBind();
    }

    pub fn GetId(&self) -> GLuint
    {
        self.m_VertexBufferID
    }

    
    // inline unsigned int GetID() const { return m_VertexBufferID; }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.m_VertexBufferID);
        }
    }
}
