#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 0) out vec3 vWorldPosition;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
    mat4 CameraTransform;
};
void main() {
    vWorldPosition = Vertex_Position;
    vec3 pos = Vertex_Position + CameraTransform[3].xyz;
    gl_Position = ViewProj * vec4(pos, 1.0);
}
