#version 330 core

// Input from vertex shader
in vec2 v_tex_coord;

// Texture sampler
uniform sampler2D u_texture;

// Output color
out vec4 frag_color;

void main() {
    vec4 texColor = texture2D(u_texture, v_tex_coord);
    
    // If alpha is very low, discard the pixel. Hack!
    if (texColor.a < 0.1) {
        discard;
    }
    
    frag_color = texColor;
}