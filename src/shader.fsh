#version 450

layout(location = 0) in vec3 i_Clr;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(i_Clr.x, i_Clr.y, i_Clr.z, 1);
}
