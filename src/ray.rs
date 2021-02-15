use crate::vec3::Vec3;
use std::ops::{Add, Mul};

pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
}

impl<T> Ray<T> {
    pub fn new(origin: Vec3<T>, direction: Vec3<T>) -> Self {
        Self { origin, direction }
    }
}

impl<T: Copy> Ray<T> {
    pub fn at<U>(&self, t: U) -> <Vec3<T> as Add<<Vec3<T> as Mul<U>>::Output>>::Output
    where
        Vec3<T>: Mul<U>,
        Vec3<T>: Add<<Vec3<T> as Mul<U>>::Output>,
    {
        self.origin + self.direction * t
    }
}
