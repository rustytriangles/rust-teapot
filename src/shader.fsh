#version 450

layout(location = 0) in vec3 i_diffColor;
layout(location = 1) in vec3 i_specColor;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(min(i_diffColor + i_specColor,1), 1);
}
