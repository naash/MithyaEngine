struct Transforms {
    model: mat4x4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
}

struct Color {
    value: vec3<f32>,
    _pad: f32,  // uniform buffers must be 16-byte aligned
}

@group(0) @binding(0) var<uniform> transforms: Transforms;
@group(1) @binding(0) var<uniform> color: Color;

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vertex_color: vec3<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = transforms.projection * transforms.view * transforms.model * vec4<f32>(in.position, 1.0);
    out.vertex_color = color.value;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.vertex_color, 1.0);
}