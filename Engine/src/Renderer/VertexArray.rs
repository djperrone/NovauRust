extern crate gl;
extern crate glfw;

use gl::types::GLenum;
// use gl::types::GLfloat;
use gl::types::GLuint;
use std::os::raw::c_void;

use super::VertexBuffer::VertexBuffer;

pub struct VertexArray
{
    m_VertexArrayID : GLuint,
}

impl VertexArray
{
    pub unsafe fn new() -> Self
	{
        let id = unsafe {
            let mut arrayID : GLuint = 0;
            gl::GenVertexArrays(1, &mut arrayID);
            arrayID
        };

        VertexArray {
            m_VertexArrayID : id,
        }
	}

	pub unsafe fn AddBuffer(&self, vb : &mut VertexBuffer, location : u32, size : u32, bufferType : GLenum , normalized : u8, stride : i32, offset : usize)
	{
		self.Bind();
		vb.Bind();
		gl::EnableVertexAttribArray(location);
		gl::VertexAttribPointer(location, size as i32, bufferType, normalized, stride as i32, offset as *const gl::types::GLvoid);
	}

	pub unsafe fn Bind(&self) 
	{
		gl::BindVertexArray(self.m_VertexArrayID);
	}

	pub unsafe fn UnBind(&self) 
	{
		gl::BindVertexArray(0);
	}
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
			println!("dropping vertex array");
            gl::DeleteBuffers(1, &mut self.m_VertexArrayID);
        }
    }
}