#version 330 core

// Vertex attributes
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 tex_coord;

// Transform matrices
uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

// Output to fragment shader
out vec2 v_tex_coord;

void main() {
    // Transform vertex position to clip space
    gl_Position = u_projection * u_view * u_model * vec4(position, 1.0);
    
    // Pass texture coordinates to fragment shader
    v_tex_coord = tex_coord;
}