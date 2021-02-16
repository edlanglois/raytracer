use crate::vec3::Vec3;
use image::Rgb;
use rand;

/// An RGB colour
pub type Colour = Vec3<f64>;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x > max {
        max
    } else if x < min {
        min
    } else {
        x
    }
}
/// Convert a float in [0, 1) to u8 with clamping.
fn float_to_u8(x: f64) -> u8 {
    (clamp(x, 0.0, 0.999) * 256.0) as u8
}

impl From<Colour> for Rgb<u8> {
    fn from(c: Colour) -> Rgb<u8> {
        Rgb([float_to_u8(c.x), float_to_u8(c.y), float_to_u8(c.z)])
    }
}

pub fn random() -> Colour {
    let (r, g, b) = rand::random();
    Colour::new(r, g, b)
}

pub fn random_range(low: f64, high: f64) -> Colour {
    let (r, g, b) = rand::random::<(f64, f64, f64)>();
    let range = high - low;
    Colour::new(r * range + low, g * range + low, b * range + low)
}
