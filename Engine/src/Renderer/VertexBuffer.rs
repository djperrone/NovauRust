use std::mem;
// use std::mem::size_of_val;

extern crate gl;
extern crate glfw;

// use gl::types::GLfloat;
use gl::types::GLsizeiptr;
use gl::types::GLuint;
use std::os::raw::c_void;

// user 

use super::VertexData::CircleVertexData;
use super::VertexData::VertexData;

// crate VertexData;

pub struct VertexBuffer {
    m_VertexBufferID: GLuint,
}

impl VertexBuffer {
    pub fn new() -> Self {

        let bufferId = unsafe {
            let mut id: GLuint = 0;
            gl::GenBuffers(1, &mut id);
            id
        };

        VertexBuffer {
            m_VertexBufferID: bufferId,
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
        data : Vec<VertexData>
    ) -> () {
        self.Bind();

        gl::BufferData(gl::ARRAY_BUFFER,
            (data.len() * mem::size_of::<VertexData>()) as GLsizeiptr,
                       &data[0] as *const VertexData as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        self.UnBind();
    }

    pub unsafe fn SetCircleData(
        &mut self,
        data : Vec<CircleVertexData>
    ) -> () {
        self.Bind();

        gl::BufferData(gl::ARRAY_BUFFER,
            (data.len() * mem::size_of::<CircleVertexData>()) as GLsizeiptr,
                       &data[0] as *const CircleVertexData as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

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
