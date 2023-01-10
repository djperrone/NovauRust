// use glm::*;

// pub struct VertexData 
// {
//     // position : glm::Vec3,
//     // color : glm::Vec4,
//     // texCoord : glm::Vec2,
//     // quantity : glm::Vec2,
//     // textureSlot : f32,

//     data : Vec<f32>
// }

// impl VertexData
// {
//     // pub fn new(position : glm::Vec3, color : glm::Vec4, texCoord : glm::Vec2, quantity : glm::Vec2, slot : f32) -> Self
//     // {
//     //     VertexData
//     //     {
//     //         position : position,
//     //         color : color,
//     //         texCoord : texCoord,
//     //         quantity : quantity,
//     //         textureSlot : slot,
//     //     }
//     // }

//     // pub fn new(position : glm::Vec3, color : glm::Vec4, texCoord : glm::Vec2, quantity : glm::Vec2) -> Self
//     // {
//     //     VertexData
//     //     {
//     //         position : position,
//     //         color : color,
//     //         texCoord : texCoord,
//     //         quantity : quantity,
//     //         slot : 0.0f,
//     //     }
//     // }

//     pub fn new(position : &mut Vec<f32>, color : &mut Vec<f32>, texCoord : &mut Vec<f32>) -> Self
//     {

//         let mut d : Vec<f32> = vec![];

//         d.append(position);
//         d.append(color);
//         d.append(texCoord);


        
//         VertexData
//         {
//             data : d,
//         }
//     }

//     // pub fn SetQuantity()
// }
