use bevy::core::{Byteable, Bytes, AsBytes};
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
    /// Junge's exponent
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
            /// Depolarization factor: The ratio of the internal electric field induced by the
            /// charges on the surface of a dielectric when an external field is applied to the polarization of the dielectric.
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

impl Sky {
    /// A.3 Scattering Coefficients
    fn total_rayleigh_scattering_coefficients(&self) -> Vec3 {
        let n2_1: f32 = self.refractive_index * self.refractive_index - 1.0;
        let n2_1_second: f32 = n2_1 * n2_1;
        let lambda_fourth = self.primaries * self.primaries;
        let lambda_fourth = lambda_fourth * lambda_fourth;
        let pi_third = std::f32::consts::PI;
        let pi_third = pi_third * pi_third * pi_third;
        let a = (8.0 * pi_third * n2_1_second) * (6.0 + 3.0 * self.depolarization_factor);
        let b = (3.0 * self.num_molecules * lambda_fourth) * (6.0 - 7.0 * self.depolarization_factor);
        a / b
    }
    fn total_mie_scattering_coefficients(&self) -> Vec3 {
        // concentration factor that varies with turbidity T
        let c: f32 = 0.2 * self.turbidity * 10e-18;
        0.434 * c * std::f32::consts::PI * (std::f32::consts::TAU / self.primaries).powf(self.mie_v - 2.0) * self.mie_k_coefficient * self.mie_coefficient
    }
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            let ptr = self as *const Sky as *const u8;
            let slice = std::slice::from_raw_parts(ptr, std::mem::size_of::<Sky>());
            slice
        }
    }
}
impl Bytes for Sky {
    fn write_bytes(&self, buffer: &mut [u8]) {
        let sky_size = std::mem::size_of::<Sky>();
        assert_eq!(self.byte_len(), buffer.len());
        buffer[0..sky_size].copy_from_slice(self.as_bytes());
        buffer[sky_size..(sky_size + std::mem::size_of::<[f32; 3]>())].copy_from_slice(self.total_rayleigh_scattering_coefficients().as_bytes());
        buffer[(sky_size + std::mem::size_of::<[f32; 4]>())..(sky_size + std::mem::size_of::<[f32; 7]>())].copy_from_slice(self.total_mie_scattering_coefficients().as_bytes());
    }

    fn byte_len(&self) -> usize {
        std::mem::size_of::<Sky>() + std::mem::size_of::<[f32; 8]>()
    }
}

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Sky>()
            .add_startup_system(sky_node::setup.system());
    }
}
