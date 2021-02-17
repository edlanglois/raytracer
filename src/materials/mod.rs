pub mod lambertian;
pub mod metal;
pub mod transparent;

pub use lambertian::Lambertian;
pub use metal::Metal;
pub use transparent::Transparent;

use crate::colour::Colour;
use crate::ray::RayR3;
use crate::vec3::VecR3;

/// A surface material
pub trait Material: Sync + Send {
    /// Scatter a ray off of a surface intersection
    ///
    /// # Arguments
    ///
    /// * `ray` - The incident ray
    /// * `point` - The intersection point
    /// * `normal` - The intersection unit normal; `ray.direction.dot(normal) < 0`
    /// * `front_face` - Whether the intersection is on the front side of the surface or back.
    ///
    /// # Returns
    /// If a scatter occurs, returns an attenuation colour and the scattered ray.
    /// Returns None if the ray is absorbed.
    fn scatter(
        &self,
        ray: &RayR3,
        point: &VecR3,
        normal: &VecR3,
        front_face: bool,
    ) -> Option<(Colour, RayR3)>;
}
