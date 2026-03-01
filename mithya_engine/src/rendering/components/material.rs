// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::asset::asset_data::{MaterialData, UniformValue};

#[derive(Clone, Debug)]
pub struct Material {
    pub data: MaterialData,
    pub render_state: RenderState,
}

#[derive(Clone, Debug)]
pub struct RenderState {
    pub blend_enabled: bool,
    pub depth_test: bool,
    pub depth_write: bool,
    pub cull_face: CullFace,
    pub blend_mode: BlendMode,
}

#[derive(Clone, Debug)]
pub enum CullFace {
    None,
    Front,
    Back,
    FrontAndBack,
}

#[derive(Clone, Debug)]
pub enum BlendMode {
    None,
    Alpha,
    Additive,
    Multiply,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            blend_enabled: false,
            depth_test: true,
            depth_write: true,
            cull_face: CullFace::Back,
            blend_mode: BlendMode::None,
        }
    }
}

impl RenderState {
    /// Convert blend mode to wgpu blend state for pipeline creation
    pub fn to_wgpu_blend(&self) -> Option<wgpu::BlendState> {
        if !self.blend_enabled {
            return None;
        }
        match self.blend_mode {
            BlendMode::Alpha => Some(wgpu::BlendState::ALPHA_BLENDING),
            BlendMode::Additive => Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent::OVER,
            }),
            BlendMode::Multiply => Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::Dst,
                    dst_factor: wgpu::BlendFactor::Zero,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent::OVER,
            }),
            BlendMode::None => None,
        }
    }

    /// Convert cull face to wgpu face for pipeline creation
    pub fn to_wgpu_cull(&self) -> Option<wgpu::Face> {
        match self.cull_face {
            CullFace::None => None,
            CullFace::Front => Some(wgpu::Face::Front),
            CullFace::Back => Some(wgpu::Face::Back),
            CullFace::FrontAndBack => Some(wgpu::Face::Back), // wgpu doesn't support both, Back is safest default
        }
    }

    /// Convert depth write to wgpu bool
    pub fn to_wgpu_depth_write(&self) -> bool {
        self.depth_write
    }
}

impl Material {
    pub fn from_data(data: MaterialData) -> Self {
        Self {
            data,
            render_state: RenderState::default(),
        }
    }

    // Uniform setters — unchanged, these just update CPU-side data
    pub fn set_float(&mut self, name: &str, value: f32) {
        self.data.uniforms.insert(name.to_string(), UniformValue::Float(value));
    }

    pub fn set_vec3(&mut self, name: &str, value: [f32; 3]) {
        self.data.uniforms.insert(name.to_string(), UniformValue::Vec3(value));
    }

    pub fn set_vec4(&mut self, name: &str, value: [f32; 4]) {
        self.data.uniforms.insert(name.to_string(), UniformValue::Vec4(value));
    }

    pub fn set_mat4(&mut self, name: &str, value: [f32; 16]) {
        self.data.uniforms.insert(name.to_string(), UniformValue::Mat4(value));
    }

    pub fn set_int(&mut self, name: &str, value: i32) {
        self.data.uniforms.insert(name.to_string(), UniformValue::Int(value));
    }

    pub fn set_bool(&mut self, name: &str, value: bool) {
        self.data.uniforms.insert(name.to_string(), UniformValue::Bool(value));
    }

    pub fn get_uniform(&self, name: &str) -> Option<&UniformValue> {
        self.data.uniforms.get(name)
    }
}