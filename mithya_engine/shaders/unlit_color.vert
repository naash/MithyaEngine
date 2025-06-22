#version 330 core

layout (location = 0) in vec3 position;
out vec3 vertexColor;  // This name must match fragment shader input

uniform vec3 u_color;
uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

void main()
{
    gl_Position = u_projection * u_view * u_model * vec4(position, 1.0);
    vertexColor = u_color;
}