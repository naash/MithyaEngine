#version 330 core

// Input from vertex shader
in vec2 v_tex_coord;

// Texture sampler
uniform sampler2D u_texture;

// Output color
out vec4 frag_color;

void main() {
    // Sample the texture
    vec4 tex_color = texture(u_texture, v_tex_coord);
    
    // Apply tint and output
    frag_color = tex_color;
}