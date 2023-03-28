use std::{mem, ptr::null};

use crate::{Camera::Camera, Novaura::Identity};

use super::{VertexBuffer::VertexBuffer, IndexBuffer::IndexBuffer, VertexArray::VertexArray, Shader::Shader, VertexData::VertexData};

extern crate gl;
extern crate glfw;

const VERTEX_SHADER : &str= r#"
#version 330 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec4 aColor;


out vec4 v_Color;
out vec3 v_Pos;


uniform mat4 u_ViewMatrix;
uniform mat4 u_ProjectionMatrix;

void main()
{
	gl_Position = u_ProjectionMatrix * u_ViewMatrix * vec4(aPos, 1.0);
	//gl_Position = vec4(aPos, 1.0);
    v_Pos = aPos;
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
    //FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    FragColor = v_Color;
} 

"#;



pub struct IRenderer
{
    vertexArray : VertexArray,
    vertexBuffer : VertexBuffer,
    indexBuffer : IndexBuffer,
    colorShader : Shader,
    defaultRectangleVertices : Vec<glm::Vec4>,
    defaultTextureCoords : Vec<glm::Vec2>,
    
}


impl IRenderer  
{
    pub unsafe fn new() -> Self
    {
        println!("irenderer new here");

        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

		gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		gl::Enable(gl::BLEND);

		//glEnable(GL_DEPTH_TEST);
		//glDepthFunc(GL_LESS);
		// gl::Enable(GL_STENCIL_TEST);
		// gl::StencilMask(0x00); // disable writing to the stencil buffer
		// gl::StencilFunc(GL_NOTEQUAL, 1, 0xFF);
		// gl::StencilOp(GL_KEEP, GL_KEEP, GL_REPLACE);

		gl::ClearColor(0.05f32, 0.05f32, 0.05f32, 1.0f32);
		
		let vertexArray = VertexArray::new();
		let vertexBuffer = VertexBuffer::new();
		// s_RenderData.TextureShader = std::make_unique<Shader>("Assets/Shaders/TextureShader.glsl");
		let colorShader = Shader::new(String::from(VERTEX_SHADER), String::from(FRAGMENT_SHADER));
		println!("irenderer new here2");
		// s_RenderData.TextRenderShader = std::make_unique<Shader>("Assets/Shaders/TextRenderShader.glsl");

		let numIndices = 6u32;
		let indices :Vec<u32> = vec![
			0,1,2,
			2,3,0		
        ];		


		let indexBuffer = IndexBuffer::new(indices, numIndices);
		 // aspect ratio
		let mut defaultRectangleVertices :Vec<glm::Vec4> = Vec::new();
		defaultRectangleVertices.resize(4, glm::Vector4::new(0.0,0.0,0.0,0.0));
		defaultRectangleVertices[0] = glm::Vec4::new(-0.5, -0.5, 0.0, 1.0);
		defaultRectangleVertices[1] = glm::Vec4::new( 0.5, -0.5, 0.0, 1.0);
		defaultRectangleVertices[2] = glm::Vec4::new( 0.5,  0.5, 0.0, 1.0);
		defaultRectangleVertices[3] = glm::Vec4::new(-0.5,  0.5, 0.0, 1.0);

		// let mut defaultTextureCoords :Vec<glm::Vec2> = Vec::with_capacity(4);

		let mut defaultTextureCoords :Vec<glm::Vec2> = Vec::new();
		defaultTextureCoords.resize(4, glm::Vector2::new(0.0,0.0));

		defaultTextureCoords[0] = glm::vec2(0.0, 0.0);
		defaultTextureCoords[1] = glm::vec2(1.0, 0.0);
		defaultTextureCoords[2] = glm::vec2(1.0, 1.0);
		defaultTextureCoords[3] = glm::vec2(0.0, 1.0);

        println!("irenderer new here3");


		IRenderer
		{
			vertexArray : vertexArray,
    		vertexBuffer : vertexBuffer,
    		indexBuffer : indexBuffer,
    		colorShader : colorShader,
    		defaultRectangleVertices : defaultRectangleVertices,
    		defaultTextureCoords : defaultTextureCoords,
		}
    }

    pub unsafe fn SetClearColor(r : f32, g :f32, b : f32, a :f32) -> ()
    {
        gl::ClearColor(r, g, b, a);
    }

    pub unsafe fn Clear()
    {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    // pub unsafe fn BeginScene(shader : &Shader, camera : &Camera)
	// {		
	// 	shader.Bind();		
	// 	shader.SetUniformMat4f("u_ViewProjectionMatrix", camera.GetViewProjectionMatrix());
	// }

	pub unsafe fn BeginScene(&mut self, camera : &Camera)
	{
		self.colorShader.Bind();
		//s_RenderData.ColorShader->SetUniformMat4f("u_ViewProjectionMatrix", camera.GetViewProjectionMatrix());
		self.colorShader.SetUniformMat4f("u_ViewMatrix", camera.GetViewMatrix());
		self.colorShader.SetUniformMat4f("u_ProjectionMatrix", camera.GetProjectionMatrix());

		// s_RenderData.TextureShader->Bind();		
		// s_RenderData.TextureShader->SetUniformMat4f("u_ViewProjectionMatrix", camera.GetViewProjectionMatrix());
		//s_RenderData.TextureShader->SetUniformMat4f("u_ProjectionMatrix", camera.GetProjectionMatrix());

		// s_RenderData.TextRenderShader->Bind();
		//s_RenderData.TextRenderShader->SetUniformMat4f("u_ViewMatrix", camera.GetViewMatrix());
		// s_RenderData.TextRenderShader->SetUniformMat4f("u_ViewProjectionMatrix", camera.GetViewProjectionMatrix());
	}

    pub unsafe fn DrawRectangle_Tutorial(&mut self, position : &glm::Vec3, scale :&glm::Vec3, color : &glm::Vec4 /*, _texture : String, _quantity: &glm::Vec2*/ )
	{

	}
    pub unsafe fn DrawRectangle(&mut self, position : &glm::Vec3, scale :&glm::Vec3, color : &glm::Vec4 /*, _texture : String, _quantity: &glm::Vec2*/ )
	{
		// tex = TextureLoader::LoadTexture(texture);
		// tex.Bind();
		// s_RenderData.TextureShader->SetUniform2f("u_Quantity", quantity.x, quantity.y);

		// let mut vertices : Vec<VertexData> = Vec::with_capacity(4);
        
        
		// let transform : glm::Mat4 = glm::ext::translate(&Identity(), glm::Vec3::new(position.x, position.y, position.z)) * glm::ext::scale(&Identity(), glm::Vec3::new(scale.x, scale.y, scale.z));
		// let mut temp = transform * self.defaultRectangleVertices[0];
		// vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), *color, self.defaultTextureCoords[0]));
		
		// temp = transform * self.defaultRectangleVertices[1];
		
		// vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), *color, self.defaultTextureCoords[1]));
		
		// temp = transform * self.defaultRectangleVertices[2];
		// vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), *color, self.defaultTextureCoords[2]));
		// temp = transform * self.defaultRectangleVertices[3];

		// vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), *color, self.defaultTextureCoords[3]));
		// //vertices.push(, color, self.defaultTextureCoords[2]);
		// //vertices.push(transform * self.defaultRectangleVertices[3], color, self.defaultTextureCoords[3]);

		// // self.vertexBuffer.SetData(vertices); ***********************************
		// self.vertexArray.AddBuffer(&mut self.vertexBuffer, 0, 3, gl::FLOAT, gl::FALSE, mem::size_of::<VertexData>()as u32, 0);
		// self.vertexArray.AddBuffer(&mut self.vertexBuffer, 1, 4, gl::FLOAT, gl::FALSE, mem::size_of::<VertexData>()as u32,VertexData::OffsetOfColor() as u32);
		// self.vertexArray.AddBuffer(&mut self.vertexBuffer, 2, 2, gl::FLOAT, gl::FALSE, mem::size_of::<VertexData>()as u32, VertexData::OffsetOfTexCoord() as u32);
		// self.vertexArray.Bind();
		// self.vertexBuffer.Bind();
		// self.indexBuffer.Bind();

		// //shader.SetUniform4f("u_Color", m_Color);
		// gl::DrawElements(gl::TRIANGLES, self.indexBuffer.GetCount() as i32, gl::UNSIGNED_INT, null());
		// tex.UnBind();
	}
}