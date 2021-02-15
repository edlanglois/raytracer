use crate::vec3::Vec3;
use image::Rgb;

pub type Colour = Vec3<f64>;

impl From<Colour> for Rgb<u8> {
    fn from(mut colour: Colour) -> Rgb<u8> {
        colour *= 255.999;
        Rgb([colour.x as u8, colour.y as u8, colour.z as u8])
    }
}
