use crate::ray::RayR3;
use crate::vec3::VecR3;

/// Details about a ray-surface intersection
pub struct Intersection {
    /// The ray position at which the intersection occurs
    pub t: f64,
    /// The intersection point
    pub point: VecR3,
    /// The unit normal surface vector at the intersection point
    /// Points from the side of the surface that the ray enters.
    pub normal: VecR3,
    /// Whether the ray enters the front face or the back face of the surface.
    pub front_face: bool,
}

impl Intersection {
    /// Construct an intersection using a normal pointing from the front face.
    pub fn from_front_normal(ray: &RayR3, t: f64, point: VecR3, front_normal: VecR3) -> Self {
        let front_face = ray.direction.dot(front_normal) < 0.0;
        let normal = if front_face {
            front_normal
        } else {
            -front_normal
        };
        Self {
            t,
            point,
            normal,
            front_face,
        }
    }
}

/// A surface in R^3
///
/// Supports checking for intersection with a ray.
pub trait Surface {
    /// Intersect a ray with the surface.
    ///
    /// Returns the first intersection that occurs between t_min and t_max.
    fn intersect(&self, ray: &RayR3, t_min: f64, t_max: f64) -> Option<Intersection>;
}

/// A list of surfaces is itself a surface
impl Surface for Vec<Box<dyn Surface>> {
    fn intersect(&self, ray: &RayR3, t_min: f64, t_max: f64) -> Option<Intersection> {
        return intersect_surfaces(self.iter(), ray, t_min, t_max);
    }
}

/// Intersect a ray with an iterator of surfaces
pub fn intersect_surfaces<'a, I>(
    iter: I,
    ray: &RayR3,
    t_min: f64,
    t_max: f64,
) -> Option<Intersection>
where
    I: Iterator<Item = &'a Box<dyn Surface>>,
{
    let mut result = None;
    let mut closest_so_far = t_max;
    for surface in iter {
        if let Some(intersection) = surface.intersect(ray, t_min, closest_so_far) {
            closest_so_far = intersection.t;
            result = Some(intersection);
        }
    }
    result
}
