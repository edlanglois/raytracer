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
    /// * `lookfrom` - Where the camera is located.
    /// * `lookat` - Where the camera is looking.
    /// * `vup` - Vertical up direction for the camera (will be projected onto viewport).
    /// * `vfov` - Vertical field of view in degrees.
    /// * `aspect_ratio` - Ratio of width over height.
    pub fn new(lookfrom: VecR3, lookat: VecR3, vup: VecR3, vfov: f64, aspect_ratio: f64) -> Self {
        let viewport_height = 2.0 * (vfov.to_radians() / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).as_unit();
        let u = vup.cross(w).as_unit();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    /// Create a ray through a coordinate on the viewport.
    ///
    /// s in [0, 1] measures from the left to right side of the viewport.
    /// t in [0, 1] measures from the bottom to top of the viewport.
    pub fn get_ray(&self, s: f64, t: f64) -> RayR3 {
        let direction =
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin;
        RayR3 {
            origin: self.origin,
            direction,
        }
    }
}
