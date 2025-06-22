#version 330 core
in vec3 vertexColor;   // Must match vertex shader output
out vec4 FragColor;

void main() {
  FragColor = vec4(vertexColor, 1.0);
}