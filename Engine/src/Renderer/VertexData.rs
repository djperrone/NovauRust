use memoffset::offset_of;

#[repr(C, packed)]
pub struct VertexData 
{
    position : glm::Vec3,
    color : glm::Vec4,
    texCoord : glm::Vec2,
    quantity : glm::Vec2,
    textureSlot : f32,
    isCircle : bool,
}

impl VertexData
{
    pub fn new(position : glm::Vec3, color : glm::Vec4, texCoord : glm::Vec2) -> Self
    {
        VertexData
        {
            position : position,
            color : color,
            texCoord : texCoord,
            quantity : glm::Vec2::new(1.0f32, 1.0f32),
            textureSlot : 0.0f32,
            isCircle : false,
            
        }
    }

    pub fn OffsetOfPosition() -> usize
    {
        offset_of!(VertexData, position)
    }

    pub fn OffsetOfColor() -> usize
    {
        offset_of!(VertexData, color)
    }

    pub fn OffsetOfTexCoord() -> usize
    {
        offset_of!(VertexData, texCoord)
    }
}



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
