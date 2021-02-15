use crate::ray::RayR3;
use crate::vec3::VecR3;

pub struct Camera {
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,

    origin: VecR3,
    lower_left_corner: VecR3,
    horizontal: VecR3,
    vertical: VecR3,
}

impl Camera {
    pub fn new(viewport_width: f64, viewport_height: f64, focal_length: f64) -> Self {
        let origin = VecR3::new(0.0, 0.0, 0.0);
        let horizontal = VecR3::new(viewport_width, 0.0, 0.0);
        let vertical = VecR3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - VecR3::new(0.0, 0.0, focal_length);
        Self {
            viewport_width,
            viewport_height,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    /// Create a ray through a coordinate on the viewport.
    ///
    /// u in [0, 1] measures from the left to right side of the viewport.
    /// v in [0, 1] measures from the bottom to top of the viewport.
    pub fn get_ray(&self, u: f64, v: f64) -> RayR3 {
        let direction =
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin;
        RayR3 {
            origin: self.origin,
            direction,
        }
    }
}
