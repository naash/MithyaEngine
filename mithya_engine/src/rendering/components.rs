use crate::components::Component;
use super::Mesh;

// Renderable component - marks an entity as something that should be rendered
#[derive(Clone, Debug)]
pub struct Renderable {
    pub mesh: Mesh,
    pub shader_program_id: Option<u32>, // Reference to shader program
}

impl Component for Renderable {}