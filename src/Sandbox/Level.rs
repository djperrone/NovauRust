

use Engine::Camera::Camera;
use Engine::Novaura::Identity;
use Engine::Renderer::IndexBuffer::IndexBuffer;
use Engine::Renderer::Shader::Shader;
use Engine::Renderer::SimpleRenderer::SimpleRenderer;
use Engine::Renderer::VertexArray::VertexArray;
use Engine::Renderer::VertexBuffer::VertexBuffer;
use Engine::Renderer::VertexData::VertexData;
use Engine::State::State;

use gl::types::{GLfloat, GLsizeiptr, GLsizei};
use spdlog::prelude::*;
use std::cell::RefCell;
use std::ffi::c_void;
// use std::mem:size_of;
use std::{mem, ptr};
use std::ptr::null;
use std::rc::Rc;
extern crate glfw;


const VERTEX_SHADER : &str= r#"
#version 330 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec4 aColor;


out vec4 v_Color;
out vec3 v_Pos;


// uniform mat4 u_ViewMatrix;
// uniform mat4 u_ProjectionMatrix;
uniform mat4 u_ViewProjectionMatrix;

void main()
{
	gl_Position = u_ViewProjectionMatrix * vec4(aPos, 1.0);
    // gl_Position = u_ProjectionMatrix * u_ViewMatrix * vec4(aPos, 1.0);
	//gl_Position = vec4(aPos, 1.0);
    // gl_Position = aPos;
    // v_Pos = aPos;
    // v_Color = gl_Position[0];
    v_Color = aColor;
}

"#;

const FRAGMENT_SHADER : &str= r#"

#version 330 core

out vec4 FragColor;
in vec4 v_Color;
in vec3 v_Pos;
uniform vec4 u_Color;

void main()
{
    // FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    FragColor = v_Color;
} 

"#;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core

    out vec4 v_Color;


    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec4 aColor;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);

       v_Color = aColor;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    in vec4 v_Color;

    void main() {
       FragColor = v_Color;
    //    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    //    FragColor = vec4(0.750f, 0.1f, 0.2f, 0.5f);
    }
"#;


pub struct Level {
    difficulty: i32,
    m_ShouldTransition: bool,
    m_IsRunning: bool,
    m_Count: i32,
}

impl Level {
    pub fn new(dif: i32) -> Self {
        Level {
            difficulty: dif,
            m_ShouldTransition: false,
            m_IsRunning: true,
            m_Count: 1000000,
        }
    }
}
impl State for Level 
{
    fn Update(&mut self, window : Rc<RefCell<glfw::Window>>)
    {
         // axis events
         match window.borrow_mut().get_key(glfw::Key::W) {
            glfw::Action::Press => println!("w was pressed axis"),
            _ => (),
        }

        self.m_Count -= 1;

        if self.m_Count == 0 {
            info!("Level update difficult {}", self.difficulty);
            self.m_IsRunning = false;
        }
    }

    fn Draw(&mut self, window : Rc<RefCell<glfw::Window>>, renderer : Rc<RefCell<SimpleRenderer>>, camera : &Camera, deltaTime : f64)
    {

        unsafe 
        {
            renderer.borrow_mut().BeginScene(camera);

            renderer.borrow_mut().DrawCircle(&glm::Vec3::new(0.0, 0.0, 0.0), &glm::Vec3::new(0.5, 0.5, 1.0), &glm::Vec4::new(0.7, 0.3, 0.5, 1.0));
            renderer.borrow_mut().DrawCircle(&glm::Vec3::new(0.5, 0.50, 0.0), &glm::Vec3::new(0.5, 0.5, 1.0), &glm::Vec4::new(0.3, 0.3, 0.5, 1.0));
            renderer.borrow_mut().DrawCircle(&glm::Vec3::new(-0.5, -0.50, 0.0), &glm::Vec3::new(0.5, 0.5, 1.0), &glm::Vec4::new(0.5, 0.3, 0.9, 1.0));
		    // renderer.borrow_mut().EndScene();
            // self.TestRender(window, renderer, camera, deltaTime,
            //      &glm::Vec3::new(0.0, 0.0, 0.0),
            //       &glm::Vec3::new(1.0, 1.0, 1.0),
            //        &glm::Vec4::new(0.7, 0.3, 0.5, 1.0));
        }
    }

    fn OnEnter(&mut self) -> bool {
       
        true
    }
     fn OnExit(&mut self) -> bool {
       
        true
    }

    fn HandleKeyBoardInput(
        &mut self,
        _window: Rc<RefCell<glfw::Window>>,
        key: glfw::Key,
        action: glfw::Action,
        _modifiers: glfw::Modifiers,
    ) {
       
        // action events
        match key
        {
            glfw::Key::Escape => match action 
            {
                glfw::Action::Press => self.m_IsRunning = false,
                _ => ()
            }
            glfw::Key::A => match action {
                glfw::Action::Press => println!("A was pressed action"),
                glfw::Action::Release => println!("A was released action"),
                _ => (),
            },
            glfw::Key::B => match action {
                glfw::Action::Release => println!("B was released action"),
                _ => (),
            },

            _ => (),
        }
    }

    fn HandleMouseInput(
        &mut self,
        _window: Rc<RefCell<glfw::Window>>,
        _button: glfw::MouseButton,
        _action: glfw::Action,
        _modifiers: glfw::Modifiers,
    ) 
    {
        // no mouse input currently used
    }

    fn ShouldTransition(&self) -> bool {
        self.m_ShouldTransition
    }

    fn IsRunning(&self) -> bool {
        self.m_IsRunning

        
    }

   
}

impl Level
{
    unsafe fn TestRender(&self, window : Rc<RefCell<glfw::Window>>, renderer : Rc<RefCell<Engine::Renderer::IRenderer::IRenderer>>, camera : &Camera, deltaTime : f64, 
        position : &glm::Vec3, scale :&glm::Vec3, color : &glm::Vec4)
    {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

		gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		gl::Enable(gl::BLEND);

		// glEnable(GL_DEPTH_TEST);
		// glDepthFunc(GL_LESS);
		// gl::Enable(GL_STENCIL_TEST);
		// gl::StencilMask(0x00); // disable writing to the stencil buffer
		// gl::StencilFunc(GL_NOTEQUAL, 1, 0xFF);
		// gl::StencilOp(GL_KEEP, GL_KEEP, GL_REPLACE);

		gl::ClearColor(0.0f32, 0.0f32, 0.5f32, 1.0f32);
		
		// let mut vertexArray = VertexArray::new();
		// let mut vertexBuffer = VertexBuffer::new();
		// // s_RenderData.TextureShader = std::make_unique<Shader>("Assets/Shaders/TextureShader.glsl");
		let mut colorShader = Shader::new(String::from(VERTEX_SHADER), String::from(FRAGMENT_SHADER));
        colorShader.Bind();

        // colorShader.SetUniformMat4f("u_ViewMatrix", camera.GetViewMatrix());
		// colorShader.SetUniformMat4f("u_ProjectionMatrix", camera.GetProjectionMatrix());
		colorShader.SetUniformMat4f("u_ViewProjectionMatrix", camera.GetViewProjectionMatrix());


        // let vertices: [f32; 9] = [
        //     -0.5, -0.5, 0.0, // left
        //      0.5, -0.5, 0.0, // right
        //      0.0,  0.5, 0.0,  // top
        // ];

        let mut defaultRectangleVertices :Vec<glm::Vec4> = Vec::new();
		defaultRectangleVertices.resize(4, glm::Vector4::new(0.0,0.0,0.0,0.0));
		defaultRectangleVertices[0] = glm::Vec4::new(-0.5, -0.5, 0.0, 1.0);
		defaultRectangleVertices[1] = glm::Vec4::new( 0.5, -0.5, 0.0, 1.0);
		defaultRectangleVertices[2] = glm::Vec4::new( 0.5,  0.5, 0.0, 1.0);
		defaultRectangleVertices[3] = glm::Vec4::new(-0.5,  0.5, 0.0, 1.0);

        let mut vertices : Vec<VertexData> = Vec::new();
        
		let transform : glm::Mat4 = glm::ext::translate(&Identity(), glm::Vec3::new(position.x, position.y, position.z)) * glm::ext::scale(&Identity(), glm::Vec3::new(scale.x, scale.y, scale.z));

		let mut temp = transform * defaultRectangleVertices[0];
        vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), glm::Vec4::new(0.75, 0.1, 0.5, 1.0)));

        temp = transform * defaultRectangleVertices[1];
        vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), glm::Vec4::new(0.75, 0.1, 0.5, 1.0)));

        temp = transform * defaultRectangleVertices[2];
        vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), glm::Vec4::new(0.75, 0.1, 0.5, 1.0)));

        temp = transform * defaultRectangleVertices[3];
        vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), glm::Vec4::new(0.75, 0.1, 0.5, 1.0)));

        // vertices.push(temp.x);
        // vertices.push(temp.y);
        // vertices.push(temp.z);
        // vertices.push(color.x);
        // vertices.push(color.y);
        // vertices.push(color.z);
        // vertices.push(color.w);

        // temp = transform * defaultRectangleVertices[1];
    //     vertices.push(temp.x);
    //     vertices.push(temp.y);
    //     vertices.push(temp.z);
    //    vertices.push(color.x);
    //    vertices.push(color.y);
    //    vertices.push(color.z);
    //    vertices.push(color.w);

        // temp = transform * defaultRectangleVertices[2];
    //     vertices.push(temp.x);
    //     vertices.push(temp.y);
    //     vertices.push(temp.z);

    //    vertices.push(color.x);
    //    vertices.push(color.y);
    //    vertices.push(color.z);
    //    vertices.push(color.w);

        // temp = transform * defaultRectangleVertices[3];
        // vertices.push(temp.x);
        // vertices.push(temp.y);
        // vertices.push(temp.z);

        // vertices.push(color.x);
        // vertices.push(color.y);
        // vertices.push(color.z);
        // vertices.push(color.w);

        let numIndices = 6u32;
		let indices :Vec<u32> = vec![
			0,1,2,
			2,3,0		
        ];		


		let mut indexBuffer = IndexBuffer::new(indices, numIndices);

        
        
        let (mut VBO, mut VAO) = (0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(VAO);
        indexBuffer.Bind();

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<VertexData>()) as GLsizeiptr,
                       &vertices[0] as *const VertexData as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

		// gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, mem::size_of::<VertexData>() as GLsizei, null());
		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, mem::size_of::<VertexData>() as GLsizei, VertexData::OffsetOfPosition() as *const gl::types::GLvoid);
		gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, mem::size_of::<VertexData>() as GLsizei, VertexData::OffsetOfColor() as *const gl::types::GLvoid);
        
        // gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, mem::size_of::<VertexData>() as GLsizei, ptr::null());
        // let offset = 12;
        // gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, mem::size_of::<VertexData>()  as GLsizei, &offset as *const i32 as *const c_void);
        // gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 4 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);

        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        indexBuffer.UnBind();
        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        
        // gl::UseProgram(shaderProgram);
        gl::BindVertexArray(VAO); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
        // gl::DrawArrays(gl::TRIANGLES, 0, 3);
		gl::DrawElements(gl::TRIANGLES, indexBuffer.GetCount() as i32, gl::UNSIGNED_INT, null());




        
		// println!("irenderer new here2");
		// // s_RenderData.TextRenderShader = std::make_unique<Shader>("Assets/Shaders/TextRenderShader.glsl");

		

		//  // aspect ratio
		

		// // let mut defaultTextureCoords :Vec<glm::Vec2> = Vec::with_capacity(4);

		// let mut defaultTextureCoords :Vec<glm::Vec2> = Vec::new();
		// defaultTextureCoords.resize(4, glm::Vector2::new(0.0,0.0));

		// defaultTextureCoords[0] = glm::vec2(0.0, 0.0);
		// defaultTextureCoords[1] = glm::vec2(1.0, 0.0);
		// defaultTextureCoords[2] = glm::vec2(1.0, 1.0);
		// defaultTextureCoords[3] = glm::vec2(0.0, 1.0);

        // // println!("irenderer new here3");

		// //s_RenderData.ColorShader->SetUniformMat4f("u_ViewProjectionMatrix", camera.GetViewProjectionMatrix());
		// colorShader.SetUniformMat4f("u_ViewMatrix", camera.GetViewMatrix());
		// colorShader.SetUniformMat4f("u_ProjectionMatrix", camera.GetProjectionMatrix());

       
		// vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), *color, defaultTextureCoords[0]));
		
		// temp = transform * defaultRectangleVertices[1];
		
		// vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), *color, defaultTextureCoords[1]));
		
		// temp = transform * defaultRectangleVertices[2];
		// vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), *color, defaultTextureCoords[2]));
		// temp = transform * defaultRectangleVertices[3];

		// vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), *color, defaultTextureCoords[3]));
		// //vertices.push(, color, self.defaultTextureCoords[2]);
		// //vertices.push(transform * self.defaultRectangleVertices[3], color, self.defaultTextureCoords[3]);

		// vertexBuffer.SetData(&vertices);
		// vertexArray.AddBuffer(&mut vertexBuffer, 0, 3, gl::FLOAT, gl::FALSE, mem::size_of::<f32>()as u32, 0);
		// // vertexArray.AddBuffer(&mut vertexBuffer, 0, 3, gl::FLOAT, gl::FALSE, mem::size_of::<VertexData>()as u32, 0);
		// vertexArray.AddBuffer(&mut vertexBuffer, 1, 4, gl::FLOAT, gl::FALSE, mem::size_of::<f32>()as u32, 3 * mem::size_of::<f32>() as u32);
		// vertexArray.AddBuffer(&mut vertexBuffer, 2, 2, gl::FLOAT, gl::FALSE, mem::size_of::<f32>()as u32, 7 * mem::size_of::<f32>() as u32);
		// vertexArray.Bind();
		// vertexBuffer.Bind();
        
		//shader.SetUniform4f("u_Color", m_Color);
		// indexBuffer.Bind();
		// gl::DrawElements(gl::TRIANGLES, indexBuffer.GetCount() as i32, gl::UNSIGNED_INT, null());


		
    }
}
