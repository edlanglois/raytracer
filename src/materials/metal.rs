use crate::colour::Colour;
use crate::materials::Material;
use crate::ray::RayR3;
use crate::vec3::VecR3;

/// A reflective metal surface
pub struct Metal {
    pub colour: Colour,
}

impl Material for Metal {
    fn scatter(&self, ray: &RayR3, point: &VecR3, normal: &VecR3) -> Option<(Colour, RayR3)> {
        let reflection = ray.direction.as_unit().reflect(*normal);
        let scattered = RayR3::new(*point, reflection);
        Some((self.colour, scattered))
    }
}
