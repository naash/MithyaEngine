// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//System
#[path = "systems/render_system.rs"]
pub mod render_system;

pub use render_system::RenderingSystem;

//Managers
#[path = "managers/shader_manager.rs"]
pub mod shader_manager;
#[path = "managers/texture_manager.rs"]
pub mod texture_manager;
#[path = "managers/material_manager.rs"]
pub mod material_manager;

pub use shader_manager::ShaderManager;
pub use texture_manager::TextureManager;
pub use material_manager::MaterialManager;

//Components
#[path = "components/mesh.rs"]
pub mod mesh;
#[path = "components/material.rs"]
pub mod material;
#[path = "components/render.rs"]
pub mod render;

pub use mesh::Mesh;
pub use render::Render;
pub use material::Material;