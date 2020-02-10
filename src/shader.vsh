#version 450

layout(location = 0) in vec4 a_Pos;
layout(location = 1) in vec3 a_Nrm;

layout(location = 0) out vec3 o_Clr;

layout(set = 0, binding = 0) uniform Locals {
    mat4 u_Transform;
};

void main() {
    gl_Position = u_Transform * a_Pos;
	o_Clr = vec3((1. - a_Nrm.x) / 2., (1. - a_Nrm.y) / 2., (1. - a_Nrm.z) / 2.);
}
