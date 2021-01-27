#version 450

layout(location = 0) out vec4 o_Target;
layout(location = 0) in vec3 vWorldPosition;
layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
    mat4 CameraTransform;
};
layout(set = 1, binding = 0) uniform Sky {
    vec3 primaries;
    float luminance;

    vec3 mie_k_coefficient;
    float mie_coefficient;

    float mie_directional_g;
    float mie_v;
    float mie_zenith_length;
    float num_molecules;

    float rayleigh;
    float rayleigh_zenith_length;
    float turbidity;
    float refractive_index;

    vec3 sun_position;
    float sun_angular_diameter_degrees;

    float sun_intensity_factor;
    float sun_intensity_falloff_steepness;
    float tonemap_weighting;
    float depolarization_factor;
};
void main() {
    o_Target = vec4(vWorldPosition, 1.0);
}
