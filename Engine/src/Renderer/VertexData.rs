use memoffset::offset_of;

#[repr(C, packed)]
pub struct VertexData 
{
    position : glm::Vec3,
    color : glm::Vec4,
    //texCoord : glm::Vec2,
    //quantity : glm::Vec2,
    //textureSlot : f32,
    //isCircle : bool,
}

impl VertexData
{
    pub fn new(position : glm::Vec3, color : glm::Vec4) -> Self
    {
        VertexData
        {
            position : position,
            color : color,
            //texCoord : texCoord,
            //quantity : glm::Vec2::new(1.0f32, 1.0f32),
            //textureSlot : 0.0f32,
            //isCircle : false,
            
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

    // pub fn OffsetOfTexCoord() -> usize
    // {
    //     offset_of!(VertexData, texCoord)
    // }
}



pub struct CircleVertexData 
{
    position : glm::Vec3,
    color : glm::Vec4,
    texCoord : glm::Vec2,
    quantity : glm::Vec2,
    textureSlot : f32,
    // isCircle : bool,
}

impl CircleVertexData
{
    pub fn new(position : glm::Vec3, color : glm::Vec4, texCoord : glm::Vec2, quantity : glm::Vec2, textureSlot : f32) -> Self
    {
        CircleVertexData
        {
            position : position,
            color : color,
            texCoord : texCoord,
            quantity : quantity,
            textureSlot :textureSlot,
            // isCircle : false,
            
        }
    }

    pub fn OffsetOfPosition() -> usize
    {
        offset_of!(CircleVertexData, position)
    }

    pub fn OffsetOfColor() -> usize
    {
        offset_of!(CircleVertexData, color)
    }

    pub fn OffsetOfTexCoord() -> usize
    {
        offset_of!(CircleVertexData, texCoord)
    }

    pub fn OffsetOfQuantity() -> usize
    {
        offset_of!(CircleVertexData, quantity)
    }

    pub fn OffsetOfTexSlot() -> usize
    {
        offset_of!(CircleVertexData, textureSlot)
    }
}



