#version 300 es

precision lowp float;

in vec3 position;
in vec3 color;

out vec3 vert_color;

void main() {
  gl_Position = vec4(position, 1.0);
  vert_color = color;
}

