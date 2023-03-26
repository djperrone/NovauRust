
pub fn Identity() -> glm::Mat4
{
     glm::Mat4::new(
    glm::Vec4::new(1.0, 0.0, 0.0, 0.0),
    glm::Vec4::new(0.0, 1.0, 0.0, 0.0),
    glm::Vec4::new(0.0, 0.0, 1.0, 0.0),
    glm::Vec4::new(0.0, 0.0, 0.0, 1.0))

}

pub fn ConvertMatToGLM(mat : nalgebra_glm::Mat4) -> glm::Mat4
{
    glm::Mat4::new(
        glm::Vec4::new(mat[0],mat[1],mat[2],mat[3]),
        glm::Vec4::new(mat[4],mat[5],mat[6],mat[7]),
        glm::Vec4::new(mat[8],mat[9],mat[10],mat[11]),
        glm::Vec4::new(mat[12],mat[13],mat[14],mat[15]),
    )
}