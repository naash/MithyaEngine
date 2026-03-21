// Copyright (c) 2025 Nishant Sthalekar
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::sync::Arc;
use glam::Mat4;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::{
    World, asset::{MaterialData, managers::AssetManager}, core::Transform, 
    rendering::{Camera, components::{MaterialGpuCache, Render, RenderGpuCache, TransformGpuCache}}
};

// GPU-side uniform buffer layout must match WGSL struct exactly
// 3 mat4x4 = 3 * 64 = 192 bytes
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct TransformUniforms {
    model: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
}

// Color uniform — vec3 + padding to reach 16 bytes
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ColorUniforms {
    color: [f32; 3],
    _pad: f32,
}

//This is a special system 
pub struct RenderingSystem {
    pub aspect_ratio: f32,
    pub ui_draw_fn: Option<Box<dyn Fn(&egui::Context, &World)>>,
    pub debug_draw_fn: Option<Box<dyn Fn(&egui::Context, &World)>>,
    
    // Core wgpu state
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    
    // Pipelines
    unlit_color_pipeline: wgpu::RenderPipeline,
    unlit_texture_pipeline: wgpu::RenderPipeline,
    
    // Bind group layouts
    transform_bind_group_layout: wgpu::BindGroupLayout,
    color_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,

    // Default sampler shared across all textures
    default_sampler: wgpu::Sampler,

    //For UI rendering
    egui_renderer: egui_wgpu::Renderer,
    egui_context: egui::Context,
    egui_state: egui_winit::State,
}

impl RenderingSystem {
    pub async fn new(window: Arc<Window>) -> Self {
        let egui_window = window.clone();  // clone Arc before wgpu moves it
        let size = window.inner_size();
        let aspect_ratio = size.width as f32 / size.height as f32;

        // Instance is the entry point to wgpu
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Surface is the thing we draw onto — tied to the window
        let surface = instance.create_surface(window).expect("Failed to find surface");

        // Adapter is a handle to the physical GPU
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.expect("Failed to get adapter");

        // Device is the logical GPU, queue is where we submit commands
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Mithya Engine Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            None,
        ).await.expect("Failed to get device");

        // Configure the surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo, // vsync
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        // --- Bind group layouts ---

        // Transforms: binding 0 = transform uniform buffer
        let transform_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Transform Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            }
        );

        // Color: binding 0 = color uniform buffer (unlit_color only)
        let color_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Color Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            }
        );

        // Texture: binding 0 = texture, binding 1 = sampler (unlit_texture only)
        let texture_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            }
        );

        // Default sampler — linear filtering, clamp to edge
        let default_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Default Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // --- Pipelines ---
        let unlit_color_pipeline = Self::create_unlit_color_pipeline(
            &device,
            surface_format,
            &transform_bind_group_layout,
            &color_bind_group_layout,
        );

        let unlit_texture_pipeline = Self::create_unlit_texture_pipeline(
            &device,
            surface_format,
            &transform_bind_group_layout,
            &texture_bind_group_layout,
        );

        //Egui stuff
        let egui_context = egui::Context::default();
        
        let egui_state = egui_winit::State::new(
            egui_context.clone(),
            egui::ViewportId::ROOT,
            &egui_window,
            Some(egui_window.scale_factor() as f32),
            None,
            None,
        );

        let egui_renderer = egui_wgpu::Renderer::new(
            &device,
            surface_config.format,  // use same format as main surface
            None,  // no depth buffer for UI
            1,     // sample count
            false,
        );

        Self {
            device,
            queue,
            surface,
            surface_config,
            unlit_color_pipeline,
            unlit_texture_pipeline,
            transform_bind_group_layout,
            color_bind_group_layout,
            texture_bind_group_layout,
            default_sampler,
            aspect_ratio,
            egui_renderer,
            egui_context,
            egui_state,
            ui_draw_fn: None,
            debug_draw_fn: None
        }
    }

    fn create_unlit_color_pipeline(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        transform_layout: &wgpu::BindGroupLayout,
        color_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Unlit Color Shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../../../shaders/unlit_color.wgsl").into()
            ),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Unlit Color Pipeline Layout"),
            bind_group_layouts: &[transform_layout, color_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Unlit Color Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    // Position only: location 0, 3 floats, stride 12 bytes
                    wgpu::VertexBufferLayout {
                        array_stride: 12,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                shader_location: 0,
                                offset: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        })
    }

    fn create_unlit_texture_pipeline(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        transform_layout: &wgpu::BindGroupLayout,
        texture_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Unlit Texture Shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../../../shaders/unlit_texture.wgsl").into()
            ),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Unlit Texture Pipeline Layout"),
            bind_group_layouts: &[transform_layout, texture_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Unlit Texture Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    // Position + UV: stride 20 bytes
                    wgpu::VertexBufferLayout {
                        array_stride: 20,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                shader_location: 0,
                                offset: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                shader_location: 1,
                                offset: 12,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
            self.aspect_ratio = width as f32 / height as f32;
        }
    }

    pub fn render(&mut self, window: &Arc<Window>, world: &mut World) {
        // Acquire the next frame from the surface
        let output = match self.surface.get_current_texture() {
            Ok(t) => t,
            Err(wgpu::SurfaceError::Lost) => {
                self.surface.configure(&self.device, &self.surface_config);
                return;
            }
            Err(e) => {
                eprintln!("Surface error: {:?}", e); //// TODO: replace with tracing
                return;
            }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") }
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05, g: 0.05, b: 0.15, a: 1.0
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            let (view, projection) = self.get_camera_matrices(world);

            let entities = world.entity_manager.get_renderable_entities();

            for entity_id in entities {
                if let (Some(transform), Some(render)) = (
                    world.entity_manager.get_component::<Transform>(entity_id).cloned(),
                    world.entity_manager.get_component_mut::<Render>(entity_id),
                ) {
                    self.render_entity(
                        &transform,
                        render,
                        &mut world.asset_manager,
                        &mut render_pass,
                        &view,
                        &projection
                    );
                }
            }
        }

        // egui pass
        let raw_input = self.egui_state.take_egui_input(window);
        let full_output = self.egui_context.run(raw_input, |ctx| {
            if let Some(draw_fn) = &self.ui_draw_fn {
                draw_fn(ctx, world);
            }
            if let Some(debug_fn) = &self.debug_draw_fn {
                debug_fn(ctx, world);
            }
        });

        self.egui_state.handle_platform_output(window, full_output.platform_output.clone());

        let tris = self.egui_context.tessellate(
            full_output.shapes.clone(), 
            full_output.pixels_per_point
        );

        for (id, delta) in &full_output.textures_delta.set {
            self.egui_renderer.update_texture(&self.device, &self.queue, *id, delta);
        }

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.surface_config.width, self.surface_config.height],
            pixels_per_point: full_output.pixels_per_point,
        };

        let mut egui_encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: Some("egui Encoder") }
        );

        self.egui_renderer.update_buffers(
            &self.device, 
            &self.queue, 
            &mut egui_encoder, 
            &tris, 
            &screen_descriptor
        );
        
        
        {
        // forget_lifetime() needed to decouple the render pass lifetime from the encoder
        // so egui_renderer.render() can accept it without lifetime conflicts
        let mut egui_pass = egui_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            }).forget_lifetime();

            self.egui_renderer.render(&mut egui_pass, &tris, &screen_descriptor);
        }
        

        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }

        self.queue.submit([encoder.finish(), egui_encoder.finish()]);
        output.present();
    }

    fn render_entity(
        &self,
        transform: &Transform,
        render: &mut Render,
        asset_manager: &mut AssetManager,
        render_pass: &mut wgpu::RenderPass,
        view: &Mat4,
        projection: &Mat4,
    ) {
        // Upload mesh to GPU if not done yet
        if !render.mesh.is_uploaded() {
            render.mesh.upload(&self.device);
        }

        let material_id = render.material_id.unwrap_or(0);
        let material = match asset_manager.get_material(material_id) {
            Some(m) => m,
            None => return,
        };

        // Build transform matrices
        let model = (Mat4::from_translation(transform.position)
            * Mat4::from_quat(transform.rotation)
            * Mat4::from_scale(transform.scale))
            .to_cols_array_2d();

        let transform_uniforms = TransformUniforms {
            model,
            view: view.to_cols_array_2d(),
            projection: projection.to_cols_array_2d(),
        };

        // --- RenderCache initialization ---
        if render.gpu_cache.is_none() {
            let transform_gpu_cache = self.build_transform_cache(&transform_uniforms);
            let material_gpu_cache = match self.build_material_cache(material, asset_manager, render.material_id) {
                Some(m) => m,
                None => return,
            };
            
            render.gpu_cache = Some(RenderGpuCache {
            transform: transform_gpu_cache,
            material: material_gpu_cache 
            })
        }
        else if let Some(cache) = &mut render.gpu_cache {
            //If cached material doesn't match, we rebuild the cache
            if cache.material.cached_id != render.material_id {
                cache.material = match self.build_material_cache(material, asset_manager, render.material_id) {
                    Some(bg) => bg,
                    None => return,
                };
            }
        }

        let cache = match render.gpu_cache.as_ref() {
            Some(c) => c,
            None => return,
        };

         // --- Transform update ---
        self.queue.write_buffer(
            &cache.transform.buffer,
            0,
            bytemuck::cast_slice(&[transform_uniforms]),
        );

        // --- Pipeline selection and draw ---
        // Bind groups already exist — just hand them to the render pass
        if cache.material.has_textures {
            render_pass.set_pipeline(&self.unlit_texture_pipeline);
        } else {
            render_pass.set_pipeline(&self.unlit_color_pipeline);
        }

        render_pass.set_bind_group(0, &cache.transform.bind_group, &[]);
        render_pass.set_bind_group(1, &cache.material.bind_group, &[]);

        // Draw — unchanged
        let vertex_buffer = render.mesh.vertex_buffer.as_ref().expect("Failed to get vertex buffer");
        let index_buffer = render.mesh.index_buffer.as_ref().expect("Failed to get index buffer");
        let index_count = render.mesh.indices.len() as u32;

        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..index_count, 0, 0..1);
    }

    fn build_transform_cache(&self, transform_uniforms: &TransformUniforms) -> TransformGpuCache {
        let transform_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Transform Buffer"),
            size: std::mem::size_of::<TransformUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Upload initial data
        self.queue.write_buffer(
            &transform_buffer,
            0,
            bytemuck::cast_slice(&[*transform_uniforms]),
        );

        // Bind group points to the buffer — created once, stays valid
        // because the buffer itself never moves or gets dropped
        let transform_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Transform Bind Group"),
            layout: &self.transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
        });

        TransformGpuCache{ buffer: transform_buffer, bind_group: transform_bind_group }
    }

    fn build_material_cache(
        &self,
        material: &MaterialData,
        asset_manager: &AssetManager,
        material_id: Option<u32>,
    ) -> Option<MaterialGpuCache> {
        let material_bind_group = if material.textures.is_empty() {
            let color = match material.uniforms.get("u_color") {
                Some(crate::asset::UniformValue::Vec3(v)) => *v,
                _ => [1.0, 1.0, 1.0],
            };

            let color_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Color Buffer"),
                contents: bytemuck::cast_slice(&[ColorUniforms { color, _pad: 0.0 }]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Color Bind Group"),
                layout: &self.color_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: color_buffer.as_entire_binding(),
                }],
            })

        } else {
            let texture_entry = material
                .textures
                .values()
                .next()
                .and_then(|b| asset_manager.get_texture(&b.texture_id))?;

            self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Texture Bind Group"),
                layout: &self.texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_entry.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.default_sampler),
                    },
                ],
            })
        };

        Some(MaterialGpuCache {
            bind_group: material_bind_group,
            cached_id: material_id,
            has_textures: !material.textures.is_empty()
        })
    }

    pub fn load_assets<F>(&self, asset_manager: &mut AssetManager, f: F)
    where
        F: FnOnce(&mut AssetManager, &wgpu::Device, &wgpu::Queue),
    {
        f(asset_manager, &self.device, &self.queue);
    }

    fn get_camera_matrices(&self, world: &World) -> (Mat4, Mat4) {
        // Find active camera entity
        let camera_entities = world.entity_manager.query_two_components::<Transform, Camera>();
        
        for entity_id in camera_entities {
            let transform = world.entity_manager.get_component::<Transform>(entity_id);
            let camera = world.entity_manager.get_component::<Camera>(entity_id);

            if let (Some(transform), Some(camera)) = (transform, camera) {
                if !camera.is_active { continue; }

                let view = Mat4::from_translation(transform.position).inverse();
                let projection = Mat4::orthographic_rh(
                    -camera.size * self.aspect_ratio, camera.size * self.aspect_ratio,
                    -camera.size, camera.size,
                    camera.near, camera.far,
                );
                return (view, projection);
            }
        }

        // Fallback if no camera found
        let projection = Mat4::orthographic_rh(
            -20.0 * self.aspect_ratio, 20.0 * self.aspect_ratio,
            -20.0, 20.0,
            -1.0, 1.0,
        );
        (Mat4::IDENTITY, projection)
    }
}