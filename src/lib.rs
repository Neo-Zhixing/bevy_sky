use bevy::prelude::*;

use bevy::core::Byteable;

mod sky_node;

#[repr(C)]
#[derive(Debug)]
struct Sky {
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
            depolarization_factor: 0.095,
            primaries: Vec3::new(0.0, 0.0, 0.0),
            luminance: 1.0,
            mie_coefficient: 0.011475,
            mie_directional_g: 0.814,
            mie_k_coefficient: Vec3::new(0.686, 0.678, 0.666),
            mie_v: 3.979,
            mie_zenith_length: 1000.0,
            num_molecules: 2.54e+25,
            rayleigh: 2.295,
            rayleigh_zenith_length: 540.0,
            sun_position: Default::default(),
            sun_angular_diameter_degrees: 0.00639,
            sun_intensity_factor: 1151.0,
            sun_intensity_falloff_steepness: 1.22,
            tonemap_weighting: 10.0,
            turbidity: 2.5,
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
