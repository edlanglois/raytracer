use crate::ray::RayR3;
use crate::vec3::VecR3;

pub struct Camera {
    origin: VecR3,
    lower_left_corner: VecR3,
    horizontal: VecR3,
    vertical: VecR3,
}

impl Camera {
    /// Create a new camera
    ///
    /// # Arguments
    /// * `vfov` - Vertical field of view in degrees.
    /// * `aspect_ratio` - Ratio of width over height.
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {
        let viewport_height = 2.0 * (vfov.to_radians() / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = VecR3::new(0.0, 0.0, 0.0);
        let horizontal = VecR3::new(viewport_width, 0.0, 0.0);
        let vertical = VecR3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - VecR3::new(0.0, 0.0, focal_length);
        Self {
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
