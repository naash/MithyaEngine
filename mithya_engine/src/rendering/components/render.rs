use super::Mesh;

// Renderable component - marks an entity as something that should be rendered
#[derive(Clone, Debug)]
pub struct Render {
    pub mesh: Mesh,
    pub material_id: Option<u32>,  // Reference by ID instead of name
}
