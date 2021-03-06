use crate::materials::Material;
use crate::ray::Ray;
use crate::surface::{Intersection, Surface};
use crate::vec3::VecR3;

/// A Sphere
pub struct Sphere<M: Material> {
    center: VecR3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: VecR3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Surface for Sphere<M> {
    fn intersect(&self, ray: &Ray<f64>, t_min: f64, t_max: f64) -> Option<Intersection> {
        let rel_origin = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let half_b = ray.direction.dot(rel_origin);
        let c = rel_origin.norm_squared() - self.radius * self.radius;
        let quarter_discriminant = half_b * half_b - a * c;
        if quarter_discriminant < 0.0 {
            return None;
        }
        let half_sqrt_discriminant = quarter_discriminant.sqrt();

        // Find the nearest root within the allowed range
        let mut t = (-half_b - half_sqrt_discriminant) / a; // Smaller root
        if t > t_max {
            return None;
        } else if t < t_min {
            t = (-half_b + half_sqrt_discriminant) / a; // Larger root
            if t < t_min || t > t_max {
                return None;
            }
        }

        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        return Some(Intersection::new(
            ray,
            t,
            point,
            outward_normal,
            &self.material,
        ));
    }
}
