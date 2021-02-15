use num_traits::real::Real;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// 3-dimensional vector
#[derive(Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Vector in R^3
pub type VecR3 = Vec3<f64>;

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Inner product
    pub fn dot<U>(self, other: Vec3<U>) -> <T as Mul<U>>::Output
    where
        T: Mul<U>,
        <T as Mul<U>>::Output: AddAssign,
    {
        let mut value = self.x * other.x;
        value += self.y * other.y;
        value += self.z * other.z;
        value
    }
}

impl<T: Copy> Vec3<T> {
    /// Cross product
    pub fn cross<U>(self, other: Vec3<U>) -> Vec3<<<T as Mul<U>>::Output as Sub>::Output>
    where
        T: Mul<U>,
        U: Copy,
        <T as Mul<U>>::Output: Sub,
    {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T> Vec3<T>
where
    T: Mul<T> + Copy,
    <T as Mul<T>>::Output: AddAssign,
{
    /// Square of the Euclidean norm
    pub fn norm_squared(self) -> <T as Mul<T>>::Output {
        self.dot(self)
    }
}

impl<T> Vec3<T>
where
    T: Real + AddAssign,
{
    /// Euclidean norm
    pub fn norm(self) -> T {
        self.norm_squared().sqrt()
    }

    pub fn unit_vector(self) -> Self {
        self / self.norm()
    }
}

impl<T: fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.y)
    }
}

impl<T: Copy> Copy for Vec3<T> {}

impl<T: Clone> Clone for Vec3<T> {
    fn clone(&self) -> Self {
        Vec3 {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}

// impl<T, I> Index<I> for Vec3<T>
// where
//     [T]: Index<I>,
// {
//     type Output = <[T] as Index<I>>::Output;

//     fn index(&self, index: I) -> &Self::Output {
//         self.0.index(index)
//     }
// }

// impl<T, I> IndexMut<I> for Vec3<T>
// where
//     [T]: IndexMut<I>,
// {
//     fn index_mut(&mut self, index: I) -> &mut Self::Output {
//         self.0.index_mut(index)
//     }
// }

impl<T: Neg> Neg for Vec3<T> {
    type Output = Vec3<<T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        let Vec3 { x, y, z } = self;
        Vec3 {
            x: -x,
            y: -y,
            z: -z,
        }
    }
}

impl<T: Add<U>, U> Add<Vec3<U>> for Vec3<T> {
    type Output = Vec3<<T as Add<U>>::Output>;

    /// Vector addition
    fn add(self, other: Vec3<U>) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: AddAssign<U>, U> AddAssign<Vec3<U>> for Vec3<T> {
    /// Vector addition in-place
    fn add_assign(&mut self, other: Vec3<U>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Sub<U>, U> Sub<Vec3<U>> for Vec3<T> {
    type Output = Vec3<<T as Sub<U>>::Output>;

    /// Vector subtraction
    fn sub(self, other: Vec3<U>) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: SubAssign<U>, U> SubAssign<Vec3<U>> for Vec3<T> {
    /// Vector subtaction in-place
    fn sub_assign(&mut self, other: Vec3<U>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T, U> Mul<U> for Vec3<T>
where
    T: Mul<U>,
    U: Copy,
{
    type Output = Vec3<<T as Mul<U>>::Output>;

    /// Scalar multiplication
    fn mul(self, t: U) -> Self::Output {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl<T, U> MulAssign<U> for Vec3<T>
where
    T: MulAssign<U>,
    U: Copy,
{
    /// Scalar multiplication in-place
    fn mul_assign(&mut self, t: U) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl<T, U> Div<U> for Vec3<T>
where
    T: Div<U>,
    U: Copy,
{
    type Output = Vec3<<T as Div<U>>::Output>;

    /// Scalar division
    fn div(self, t: U) -> Self::Output {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl<T, U> DivAssign<U> for Vec3<T>
where
    T: DivAssign<U>,
    U: Copy,
{
    /// Scalar division in-place
    fn div_assign(&mut self, t: U) {
        self.x /= t;
        self.y /= t;
        self.z /= t;
    }
}

/// Sample a random unit vector
impl Distribution<VecR3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> VecR3 {
        // Use rejection sampling to get a point within the sphere
        loop {
            let (x, y, z) = rng.gen();
            if x * x + y * y + z * z < 1.0 {
                return Vec3::new(x, y, z).unit_vector();
            }
        }
    }
}
