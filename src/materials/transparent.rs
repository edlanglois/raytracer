use crate::colour::Colour;
use crate::materials::Material;
use crate::ray::RayR3;
use crate::vec3::VecR3;

/// A fully transparent surface that always refracts when possible
pub struct Transparent {
    pub refractive_index: f64,
}

impl Transparent {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }
}

impl Material for Transparent {
    fn scatter(
        &self,
        ray: &RayR3,
        point: &VecR3,
        normal: &VecR3,
        front_face: bool,
    ) -> Option<(Colour, RayR3)> {
        let refraction_ratio = if front_face {
            self.refractive_index.recip()
        } else {
            self.refractive_index
        };
        let unit_direction = ray.direction.as_unit();
        let cos_theta = (-unit_direction.dot(*normal)).min(1.0);
        let new_direction = match unit_direction.refract(*normal, refraction_ratio) {
            Some(refraction) if reflectance(cos_theta, refraction_ratio) < rand::random() => {
                refraction
            }
            _ => unit_direction.reflect(*normal),
        };

        let scatter = RayR3::new(*point, new_direction);
        Some((Colour::new(1.0, 1.0, 1.0), scatter))
    }
}

fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}
