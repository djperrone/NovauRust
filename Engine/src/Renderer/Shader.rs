use std::collections::HashMap;
use std::ptr::null;
use std::ptr::null_mut;
// use std::mem::size_of_val;

extern crate gl;
extern crate glfw;

use gl::types::GLsizei;
// use gl::types::GLfloat;
use gl::types::GLchar;
use gl::types::GLuint;
use gl::types::GLint;

pub struct Shader
{
    m_ShaderID : GLuint,
    m_UniformLocationCache : HashMap<String, GLint>,
}

impl Shader
{
    pub unsafe fn new(vertexShader : String, fragmentShader : String) -> Self
    {
        let shaderID = unsafe {
            Self::CreateShader(vertexShader, fragmentShader)
        };

        Shader {
            m_ShaderID : shaderID,
            m_UniformLocationCache : HashMap::new(),
        }
    }

    pub unsafe fn Bind(&self)
    {
        gl::UseProgram(self.m_ShaderID);
    }

    pub unsafe fn Unbind(&self)
    {
        gl::UseProgram(0);
    }

    pub unsafe fn GetUniformLocation(&mut self, name : &str) -> GLint
    {
        if self.m_UniformLocationCache.contains_key(name)
        {
            return self.m_UniformLocationCache[name];
        }
        else
        {
            self.m_UniformLocationCache.insert(name.to_string(), gl::GetUniformLocation(self.m_ShaderID, name.as_ptr() as *const GLchar));
            return self.m_UniformLocationCache[name];
        }
    }

    unsafe fn CompileShader(shaderType : u32, source : String) -> u32
    {
        // let id = glCreateShader(shaderType);
        let id = gl::CreateShader(shaderType);

        // const char* src = source.c_str();

        gl::ShaderSource(id, 1, source.as_ptr() as *const * const i8, null());
        gl::CompileShader(id);

        let mut success = 1;
        // char infoLog[512];
        let mut infoLog = String::with_capacity(512);

        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        if success == 0
        {
            gl::GetShaderInfoLog(id, 512 as GLsizei, null_mut(), infoLog.as_mut_ptr() as *mut i8);
            if shaderType == gl::VERTEX_SHADER
            {
                println!("vertex shader compilation failed");
            }
            else
            {
                println!("fragment shader compilation failed");   
            }
            // println!("ERROR::SHADER::" << ({type} == gl::VERTEX_SHADER ? "vertex" : "fragment") << "VERTEX::COMPILATION_FAILED\n" << infoLog << std::endl;
            gl::DeleteShader(id);
            return 0;
        };

        return id;
    }

    unsafe fn CreateShader(vertexShader : String, fragmentShader : String) -> u32
    {
        let program = gl::CreateProgram();
        let vertex = Self::CompileShader(gl::VERTEX_SHADER, vertexShader);
        let fragment = Self::CompileShader(gl::FRAGMENT_SHADER, fragmentShader);


        gl::AttachShader(program, vertex);
        gl::AttachShader(program, fragment);
        gl::LinkProgram(program);

        let mut success = 1;
        let mut infoLog = String::with_capacity(512);
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success == 0
        {
            gl::GetProgramInfoLog(program, 512 as GLsizei, null_mut(), infoLog.as_mut_ptr() as *mut i8);
            // std::cout << "ERROR::SHADER::PROGRAM::LINKING_FAILED\n" << infoLog << std::endl;
            println!("ERROR::SHADER::PROGRAM::LINKING_FAILED");
            return 0;
        }

        // delete the shaders as they're linked into our program now and no longer necessary
        gl::DeleteShader(vertex);
        gl::DeleteShader(fragment);

        return program;
    }

    pub unsafe fn SetUniform1i(&mut self, name : &str, value : i32)
    {
        gl::Uniform1i(self.GetUniformLocation(name), value);
    }

    pub unsafe fn SetUniform1f(&mut self, name : &str, value : f32)
    {
        gl::Uniform1f(self.GetUniformLocation(name), value);
    }

    pub unsafe fn SetUniform2f(&mut self, name : &str, value1 :f32, value2 : f32)
    {
        gl::Uniform2f(self.GetUniformLocation(name), value1, value2);
    }

    pub unsafe fn SetUniform3f(&mut self, name : &str, value : &glm::Vec3)
    {
        gl::Uniform3f(self.GetUniformLocation(name), value.x, value.y, value.z);
    }

    pub unsafe fn SetUniform4f(&mut self, name : &str, value : &glm::Vec4)
    {
        gl::Uniform4f(self.GetUniformLocation(name), value.x, value.y, value.z, value.w);
    }

    pub unsafe fn SetUniformMat3f(&mut self, name : &str, matrix : &glm::Mat3)
    {
        gl::UniformMatrix3fv(self.GetUniformLocation(name),1, gl::FALSE, matrix.as_array().as_ptr() as *const f32);
    }

    pub unsafe fn SetUniformMat4f(&mut self, name : &str, matrix : &glm::Mat4)
    {
        gl::UniformMatrix4fv(self.GetUniformLocation(name),1, gl::FALSE, matrix.as_array().as_ptr() as *const f32);
    }

    pub unsafe fn SetBool(&mut self, name : &str, value : bool) 
    {
        gl::Uniform1i(self.GetUniformLocation(name), value as i32);
    }

    pub unsafe fn SetInt(&mut self, name : &str, value : i32) 
    {
        gl::Uniform1i(self.GetUniformLocation(name), value);
    }

    pub unsafe fn SetFloat(&mut self, name : &str, value : f32) 
    {
        gl::Uniform1f(self.GetUniformLocation(name), value);
    }
 
    pub unsafe fn SetIntArray(&mut self, name : &str,  values : Vec<i32>, count : i32)
    {      
        gl::Uniform1iv(self.GetUniformLocation(name), count, values.as_ptr());
    }

}