#version 450

layout(location = 0) in vec4 a_Pos;
layout(location = 1) in vec3 a_Nrm;
layout(location = 2) in vec3 a_Uv;

layout(location = 0) out vec3 o_diffColor;
layout(location = 1) out vec3 o_specColor;

layout(set = 0, binding = 0) uniform Locals {
    mat4 u_Transform;
};

void main() {
    float ka = 0.1;
    float kd = 0.8;
    float ks = 0.75;
    float specExp = 24;

    vec3 lightVec = vec3(0.5 * 0.7071, 0.7071,  0.866 * 0.7071);
    vec3 lightColor = vec3(1);
    vec3 matColor = vec3(0.5, 0.25, 1);
    
    vec3 eyeVec = normalize(inverse(mat3(u_Transform))*vec3(0, 0, 1));

    gl_Position = u_Transform * a_Pos;
    
    o_diffColor = min(ka + kd * max(dot(a_Nrm, lightVec), 0), 1) * matColor;
    
    vec3 halfVec = normalize(lightVec - eyeVec);

    o_specColor = pow(max(dot(halfVec, a_Nrm), 0), specExp) * lightColor;
}
