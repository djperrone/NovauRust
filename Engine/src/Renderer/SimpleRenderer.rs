use std::{mem, ptr::null, ffi::c_void};

use gl::types::{GLsizei, GLsizeiptr};

use crate::{Camera::Camera, Novaura::Identity};

// use crate::{VertexBuffer::VertexBuffer, IndexBuffer::IndexBuffer, VertexArray::VertexArray, Shader::Shader, VertexData::VertexData};
use crate::Renderer::VertexArray::VertexArray;
use crate::Renderer::VertexData::VertexData;
use crate::Renderer::VertexBuffer::VertexBuffer;
use crate::Renderer::IndexBuffer::IndexBuffer;

use crate::Renderer::Shader::Shader;

use super::VertexData::CircleVertexData;

extern crate gl;
extern crate glfw;

const CIRCLE_VERTEX_SHADER : &str =r#"

#version 330 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec4 aColor;
layout(location = 2) in vec2 aTexCoords;
layout(location = 3) in vec2 aQuantity;
layout(location = 4) in float aTexIndex;


out vec4 v_Color;
out vec3 v_Pos;
out vec2 v_TexCoords;
out vec2 v_Quantity;
out float v_TexIndex;

uniform mat4 u_ViewProjectionMatrix;

void main()
{	
	gl_Position = u_ViewProjectionMatrix * vec4(aPos, 1.0);
    v_Color = aColor;
	
    v_Pos = aPos;
    //v_Pos = vec3(gl_Position.x,gl_Position.y,gl_Position.z);
    v_TexCoords = aTexCoords;
    v_TexIndex = aTexIndex;
    v_Quantity = aQuantity;
}

"#;

const CIRCLE_FRAGMENT_SHADER :&str =r#"

#version 330 core

out vec4 Color;

in vec3 v_Pos;
in vec4 v_Color;
in vec2 v_TexCoords;
in vec2 v_Quantity;
in float v_TexIndex;

//uniform sampler2D u_Textures[32];

void main()
{       
    float distance = 1.0 - length(vec2(v_Pos.x - v_TexCoords.x, v_Pos.y - v_TexCoords.y));   
    //float distance = 1.0 - length(vec2(v_Pos.x, v_Pos.y));   
    float fade = 0.005;   
    float cutoff = 1.0 - (v_Quantity.x / 2.0);
    distance = smoothstep(cutoff, cutoff + fade, distance);
   
    Color = vec4(distance) * v_Color;
    // Color = v_Color;

} 

"#;

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

pub struct SimpleRenderer
{
    vertexArray : VertexArray,
    vertexBuffer : VertexBuffer,
    indexBuffer : IndexBuffer,
    colorShader : Shader,
    defaultRectangleVertices : Vec<glm::Vec4>,
    // defaultTextureCoords : Vec<glm::Vec2>,
}

impl SimpleRenderer  
{
    pub unsafe fn new() -> Self
    {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

		gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		gl::Enable(gl::BLEND);

		gl::ClearColor(0.05f32, 0.05f32, 0.05f32, 1.0f32);
		
		let mut vertexArray = VertexArray::new();
		let mut vertexBuffer = VertexBuffer::new();
		let colorShader = Shader::new(String::from(CIRCLE_VERTEX_SHADER), String::from(CIRCLE_FRAGMENT_SHADER));

		let numIndices = 6u32;
		let indices :Vec<u32> = vec![
			0,1,2,
			2,3,0		
        ];		

        let mut indexBuffer = IndexBuffer::new(indices, numIndices);

		 // aspect ratio
		let mut defaultRectangleVertices :Vec<glm::Vec4> = Vec::new();
		defaultRectangleVertices.resize(4, glm::Vector4::new(0.0,0.0,0.0,0.0));
		defaultRectangleVertices[0] = glm::Vec4::new(-0.5, -0.5, 0.0, 1.0);
		defaultRectangleVertices[1] = glm::Vec4::new( 0.5, -0.5, 0.0, 1.0);
		defaultRectangleVertices[2] = glm::Vec4::new( 0.5,  0.5, 0.0, 1.0);
		defaultRectangleVertices[3] = glm::Vec4::new(-0.5,  0.5, 0.0, 1.0);

        vertexArray.Bind();
        vertexBuffer.Bind();
        indexBuffer.Bind();

        // vertexArray.AddBuffer(&mut vertexBuffer, 0, 3, gl::FLOAT, gl::FALSE, mem::size_of::<VertexData>() as GLsizei, VertexData::OffsetOfPosition());
        // vertexArray.AddBuffer(&mut vertexBuffer, 1, 4, gl::FLOAT, gl::FALSE, mem::size_of::<VertexData>() as GLsizei, VertexData::OffsetOfColor());



        vertexArray.AddBuffer(&mut vertexBuffer, 0, 3, gl::FLOAT, gl::FALSE, mem::size_of::<CircleVertexData>() as GLsizei, CircleVertexData::OffsetOfPosition());
        vertexArray.AddBuffer(&mut vertexBuffer, 1, 4, gl::FLOAT, gl::FALSE, mem::size_of::<CircleVertexData>() as GLsizei, CircleVertexData::OffsetOfColor());

        vertexArray.AddBuffer(&mut vertexBuffer, 2, 2, gl::FLOAT, gl::FALSE, mem::size_of::<CircleVertexData>() as GLsizei, CircleVertexData::OffsetOfTexCoord());
        vertexArray.AddBuffer(&mut vertexBuffer, 3, 2, gl::FLOAT, gl::FALSE, mem::size_of::<CircleVertexData>() as GLsizei, CircleVertexData::OffsetOfQuantity());
        vertexArray.AddBuffer(&mut vertexBuffer, 4, 1, gl::FLOAT, gl::FALSE, mem::size_of::<CircleVertexData>() as GLsizei, CircleVertexData::OffsetOfTexSlot());

        //s_RenderData.BatchVertexArray->AddBuffer(*s_RenderData.BatchVertexBuffer, 0, 3, GL_FLOAT, GL_FALSE, sizeof(VertexData), 0);
		//s_RenderData.BatchVertexArray->AddBuffer(*s_RenderData.BatchVertexBuffer, 1, 4, GL_FLOAT, GL_FALSE, sizeof(VertexData), offsetof(VertexData, Color));
		//s_RenderData.BatchVertexArray->AddBuffer(*s_RenderData.BatchVertexBuffer, 2, 2, GL_FLOAT, GL_FALSE, sizeof(VertexData), offsetof(VertexData, TexCoord));
		//s_RenderData.BatchVertexArray->AddBuffer(*s_RenderData.BatchVertexBuffer, 3, 2, GL_FLOAT, GL_FALSE, sizeof(VertexData), offsetof(VertexData, Quantity));
		//s_RenderData.BatchVertexArray->AddBuffer(*s_RenderData.BatchVertexBuffer, 4, 1, GL_FLOAT, GL_FALSE, sizeof(VertexData), offsetof(VertexData, TextureSlot));

        vertexArray.UnBind();
        vertexBuffer.UnBind();
        indexBuffer.UnBind();

		SimpleRenderer
		{
			vertexArray : vertexArray,
    		vertexBuffer : vertexBuffer,
    		indexBuffer : indexBuffer,
    		colorShader : colorShader,
    		defaultRectangleVertices : defaultRectangleVertices,
    		// defaultTextureCoords : defaultTextureCoords,
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

	pub unsafe fn BeginScene(&mut self, camera : &Camera)
	{
		self.colorShader.Bind();		
		self.colorShader.SetUniformMat4f("u_ViewProjectionMatrix", camera.GetViewProjectionMatrix());
	}

  
    pub unsafe fn DrawRectangle(&mut self, position : &glm::Vec3, scale :&glm::Vec3, color : &glm::Vec4 /*, _texture : String, _quantity: &glm::Vec2*/ )
	{
		let mut vertices : Vec<VertexData> = Vec::new();
        
		let transform : glm::Mat4 = glm::ext::translate(&Identity(), glm::Vec3::new(position.x, position.y, position.z)) * glm::ext::scale(&Identity(), glm::Vec3::new(scale.x, scale.y, scale.z));

		let mut temp = transform * self.defaultRectangleVertices[0];
        vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), glm::Vec4::new(color.x, color.y, color.z, 1.0)));

        temp = transform * self.defaultRectangleVertices[1];
        vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), glm::Vec4::new(color.x, color.y, color.z, 1.0)));

        temp = transform * self.defaultRectangleVertices[2];
        vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), glm::Vec4::new(color.x, color.y, color.z, 1.0)));

        temp = transform * self.defaultRectangleVertices[3];
        vertices.push(VertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), glm::Vec4::new(color.x, color.y, color.z, 1.0)));

        self.vertexBuffer.SetData(vertices);
		
        self.vertexArray.Bind();
        gl::DrawElements(gl::TRIANGLES, self.indexBuffer.GetCount() as i32, gl::UNSIGNED_INT, null());

	}

    pub unsafe fn DrawCircle(&mut self, position : &glm::Vec3, scale :&glm::Vec3, color : &glm::Vec4)
	{
		// s_RenderData.CircleShader->Bind();
		// constexpr float texIndex = 0.0f;
        let texIndex : f32 = 0.0;
		// glm::vec2 scale1 = scale;
		// glm::mat4 transform = glm::translate(glm::mat4(1.0f), position) * glm::scale(glm::mat4(1.0f), scale);

		//s_RenderData.BatchRectBuffer.emplace_back(transform * s_RenderData.DefaultRectangleVertices[0], color, glm::vec2(position.x, position.y), glm::vec2(scale1.x, scale1.y), texIndex);
		//s_RenderData.BatchRectBuffer.emplace_back(transform * s_RenderData.DefaultRectangleVertices[1], color, glm::vec2(position.x, position.y), glm::vec2(scale1.x, scale1.y), texIndex);
		//s_RenderData.BatchRectBuffer.emplace_back(transform * s_RenderData.DefaultRectangleVertices[2], color, glm::vec2(position.x, position.y), glm::vec2(scale1.x, scale1.y), texIndex);
		//s_RenderData.BatchRectBuffer.emplace_back(transform * s_RenderData.DefaultRectangleVertices[3], color, glm::vec2(position.x, position.y), glm::vec2(scale1.x, scale1.y), texIndex);
		//s_RenderData.TextureShader->Bind();

		let mut vertices : Vec<CircleVertexData> = Vec::new();


        let transform : glm::Mat4 = glm::ext::translate(&Identity(), glm::Vec3::new(position.x, position.y, position.z)) * glm::ext::scale(&Identity(), glm::Vec3::new(scale.x, scale.y, scale.z));

		let mut temp = transform * self.defaultRectangleVertices[0];
        vertices.push(CircleVertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), glm::Vec4::new(color.x, color.y, color.z, 1.0), glm::Vec2::new(position.x, position.y), glm::Vec2::new(scale.x, scale.y), texIndex));

        temp = transform * self.defaultRectangleVertices[1];
        vertices.push(CircleVertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), glm::Vec4::new(color.x, color.y, color.z, 1.0),glm::Vec2::new(position.x, position.y), glm::Vec2::new(scale.x, scale.y), texIndex));

        temp = transform * self.defaultRectangleVertices[2];
        vertices.push(CircleVertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), glm::Vec4::new(color.x, color.y, color.z, 1.0),glm::Vec2::new(position.x, position.y), glm::Vec2::new(scale.x, scale.y), texIndex));

        temp = transform * self.defaultRectangleVertices[3];
        vertices.push(CircleVertexData::new(glm::Vec3::new(temp.x, temp.y, temp.z), glm::Vec4::new(color.x, color.y, color.z, 1.0),glm::Vec2::new(position.x, position.y), glm::Vec2::new(scale.x, scale.y), texIndex));

        self.vertexBuffer.SetCircleData(vertices);
		
        self.vertexArray.Bind();
        gl::DrawElements(gl::TRIANGLES, self.indexBuffer.GetCount() as i32, gl::UNSIGNED_INT, null());

	}
}