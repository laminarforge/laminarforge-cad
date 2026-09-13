#version 330 core
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 barycentric;
uniform mat4 mvp;
out vec3 surface_normal;
out vec3 edge_coord;
void main() {
    gl_Position = mvp * vec4(position, 1.0);
    surface_normal = normal;
    edge_coord = barycentric;
}
