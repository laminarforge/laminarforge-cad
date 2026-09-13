#version 330 core
in vec3 surface_normal;
in vec3 edge_coord;
uniform vec3 material;
uniform int mode;
out vec4 color;
void main() {
    vec3 n = normalize(surface_normal);
    float light = 0.28 + 0.55 * abs(dot(n, normalize(vec3(0.4, -0.6, 1.0))))
                       + 0.17 * abs(dot(n, normalize(vec3(-0.8, 0.3, 0.4))));
    vec3 width = fwidth(edge_coord) * 1.2;
    vec3 smooth_edge = smoothstep(vec3(0.0), width, edge_coord);
    float interior = min(min(smooth_edge.x, smooth_edge.y), smooth_edge.z);
    vec3 shaded = material * light;
    if (mode == 1) shaded = mix(vec3(0.055, 0.12, 0.15), shaded, interior);
    if (mode == 2) shaded = mix(material, vec3(0.055, 0.075, 0.095), interior);
    color = vec4(shaded, 1.0);
}
