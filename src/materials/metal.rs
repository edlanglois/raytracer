use crate::colour::Colour;
use crate::materials::Material;
use crate::ray::RayR3;
use crate::vec3::VecR3;
use rand;

/// A reflective metal surface
pub struct Metal {
    pub colour: Colour,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(colour: Colour, fuzz: f64) -> Self {
        Self {
            colour,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &RayR3, point: &VecR3, normal: &VecR3) -> Option<(Colour, RayR3)> {
        let reflection = ray.direction.as_unit().reflect(*normal);
        let scattered = RayR3::new(*point, reflection + rand::random::<VecR3>() * self.fuzz);
        if scattered.direction.dot(*normal) < 0.0 {
            None
        } else {
            Some((self.colour, scattered))
        }
    }
}
