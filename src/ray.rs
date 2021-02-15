use crate::vec3::Vec3;
use std::ops::{Add, Mul};

/// A Ray in 3D space
///
/// A line defined by an origin and a direction.
pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
}

/// A Ray in R^3
pub type RayR3 = Ray<f64>;

impl<T> Ray<T> {
    /// Create a new ray
    pub fn new(origin: Vec3<T>, direction: Vec3<T>) -> Self {
        Self { origin, direction }
    }
}

impl<T: Copy> Ray<T> {
    /// The point at a specific offset along the ray.
    ///
    /// ray.at(t) = ray.origin + ray.direction * t
    pub fn at<U>(&self, t: U) -> <Vec3<T> as Add<<Vec3<T> as Mul<U>>::Output>>::Output
    where
        Vec3<T>: Mul<U>,
        Vec3<T>: Add<<Vec3<T> as Mul<U>>::Output>,
    {
        self.origin + self.direction * t
    }
}
