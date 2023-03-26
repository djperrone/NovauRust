use std::mem;
// use std::mem::size_of_val;

extern crate gl;
extern crate glfw;

// use gl::types::GLfloat;
use gl::types::GLsizeiptr;
use gl::types::GLuint;
use std::os::raw::c_void;

pub struct IndexBuffer 
{
    m_IndexBufferID : GLuint,
    m_Count : GLuint,
}

impl IndexBuffer
{
    pub fn new(indices : Vec<u32>, numIndices : u32) -> Self
    {
        let id = unsafe {
            let mut id : GLuint = 0;

            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (numIndices * mem::size_of::<u32>() as u32) as GLsizeiptr, indices.as_ptr() as *const u32 as *const c_void, gl::STATIC_DRAW);

            id
        };


        IndexBuffer{
            m_IndexBufferID : id,
            m_Count : numIndices,
        }
    }

    pub unsafe fn Bind(&mut self) -> () {
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.m_IndexBufferID);
        
    }

    pub unsafe fn UnBind(&mut self) -> () {
        
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }

    pub fn GetCount(&self) -> u32
    {
        self.m_Count
    }

  
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.m_IndexBufferID);
        }
    }
}