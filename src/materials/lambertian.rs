use crate::colour::Colour;
use crate::materials::Material;
use crate::ray::RayR3;
use crate::vec3::VecR3;
use rand;

/// A Lambertian (matte) material
pub struct Lambertian {
    pub colour: Colour,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &RayR3,
        point: &VecR3,
        normal: &VecR3,
        _front_face: bool,
    ) -> Option<(Colour, RayR3)> {
        let mut scatter_direction = *normal + rand::random();
        if scatter_direction.near_zero() {
            scatter_direction = *normal;
        }

        let scattered = RayR3::new(*point, scatter_direction);
        Some((self.colour, scattered))
    }
}
