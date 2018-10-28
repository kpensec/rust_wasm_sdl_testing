#version 300 es

precision lowp float;

in vec3 vert_color;

out vec4 out_color;

void main() {
  out_color = vec4(vert_color.rgb, 1.0);
}

