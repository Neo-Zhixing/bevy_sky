use bevy::core::Byteable;
use bevy::prelude::*;

mod sky_node;

#[repr(C)]
#[derive(Debug)]
pub struct Sky {
    primaries: Vec3,
    luminance: f32,

    mie_k_coefficient: Vec3,
    mie_coefficient: f32,

    mie_directional_g: f32,
    mie_v: f32,
    mie_zenith_length: f32,
    num_molecules: f32,

    rayleigh: f32,
    rayleigh_zenith_length: f32,
    turbidity: f32,
    refractive_index: f32,

    sun_position: Vec3,
    sun_angular_diameter_degrees: f32,

    sun_intensity_factor: f32,
    sun_intensity_falloff_steepness: f32,
    tonemap_weighting: f32,
    depolarization_factor: f32,
}

impl Default for Sky {
    fn default() -> Self {
        Sky {
            depolarization_factor: 0.035,
            /// Peak wavelength for red, green and blue light.
            /// 440mn, 550nm and 680nm are considered as the peaks for blue,
            /// green and red light respectively.
            primaries: Vec3::new(6.8e-7, 5.5e-7, 4.5e-7),
            luminance: 1.0,
            mie_coefficient: 0.005,
            mie_directional_g: 0.8,
            mie_k_coefficient: Vec3::new(0.686, 0.678, 0.666),
            mie_v: 4.0,
            mie_zenith_length: 1.25e3,
            /// The molecular density at sea level
            num_molecules: 2.542e25,
            rayleigh: 1.0,
            rayleigh_zenith_length: 8.4e3,
            sun_position: Vec3::new(700000.0, 50000.0, 0.0),
            sun_angular_diameter_degrees: 0.0093333,
            sun_intensity_factor: 1000.0,
            sun_intensity_falloff_steepness: 1.5,
            tonemap_weighting: 9.50,
            turbidity: 2.0,
            /// The index of refraction of air
            refractive_index: 1.000262,
        }
    }
}
unsafe impl Byteable for Sky {}

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Sky>()
            .add_startup_system(sky_node::setup.system());
    }
}
