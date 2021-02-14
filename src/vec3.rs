use num_traits::real::Real;
use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

/// 3-dimensional vector
#[derive(Debug)]
pub struct Vec3<T>([T; 3]);

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }

    pub fn x(&self) -> &T {
        &self.0[0]
    }
    pub fn y(&self) -> &T {
        &self.0[1]
    }
    pub fn z(&self) -> &T {
        &self.0[2]
    }

    /// Inner product
    pub fn dot<U>(self, other: Vec3<U>) -> <T as Mul<U>>::Output
    where
        T: Mul<U>,
        <T as Mul<U>>::Output: AddAssign,
    {
        let [x, y, z] = self.0;
        let [ox, oy, oz] = other.0;
        let mut value = x * ox;
        value += y * oy;
        value += z * oz;
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
        let [x, y, z] = self.0;
        let [ox, oy, oz] = other.0;
        Vec3([y * oz - z * oy, z * ox - x * oz, x * oy - y * ox])
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
        write!(f, "({}, {}, {})", self.0[0], self.0[1], self.0[2])
    }
}

impl<T: Copy> Copy for Vec3<T> {}

impl<T: Clone> Clone for Vec3<T> {
    fn clone(&self) -> Self {
        Vec3([self.0[0].clone(), self.0[1].clone(), self.0[2].clone()])
    }
}

impl<T, I> Index<I> for Vec3<T>
where
    [T]: Index<I>,
{
    type Output = <[T] as Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T, I> IndexMut<I> for Vec3<T>
where
    [T]: IndexMut<I>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<T: Neg> Neg for Vec3<T> {
    type Output = Vec3<<T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        let [x, y, z] = self.0;
        Vec3([-x, -y, -z])
    }
}

impl<T: Add<U>, U> Add<Vec3<U>> for Vec3<T> {
    type Output = Vec3<<T as Add<U>>::Output>;

    /// Vector addition
    fn add(self, other: Vec3<U>) -> Self::Output {
        let [x, y, z] = self.0;
        let [ox, oy, oz] = other.0;
        Vec3([x + ox, y + oy, z + oz])
    }
}

impl<T: AddAssign<U>, U> AddAssign<Vec3<U>> for Vec3<T> {
    /// Vector addition in-place
    fn add_assign(&mut self, other: Vec3<U>) {
        let [ox, oy, oz] = other.0;
        self.0[0] += ox;
        self.0[1] += oy;
        self.0[2] += oz;
    }
}

impl<T: Sub<U>, U> Sub<Vec3<U>> for Vec3<T> {
    type Output = Vec3<<T as Sub<U>>::Output>;

    /// Vector subtraction
    fn sub(self, other: Vec3<U>) -> Self::Output {
        let [x, y, z] = self.0;
        let [ox, oy, oz] = other.0;
        Vec3([x - ox, y - oy, z - oz])
    }
}

impl<T: SubAssign<U>, U> SubAssign<Vec3<U>> for Vec3<T> {
    /// Vector subtaction in-place
    fn sub_assign(&mut self, other: Vec3<U>) {
        let [ox, oy, oz] = other.0;
        self.0[0] -= ox;
        self.0[1] -= oy;
        self.0[2] -= oz;
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
        let [x, y, z] = self.0;
        Vec3([x * t, y * t, z * t])
    }
}

impl<T, U> MulAssign<U> for Vec3<T>
where
    T: MulAssign<U>,
    U: Copy,
{
    /// Scalar multiplication in-place
    fn mul_assign(&mut self, t: U) {
        self.0[0] *= t;
        self.0[1] *= t;
        self.0[2] *= t;
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
        let [x, y, z] = self.0;
        Vec3([x / t, y / t, z / t])
    }
}

impl<T, U> DivAssign<U> for Vec3<T>
where
    T: DivAssign<U>,
    U: Copy,
{
    /// Scalar division in-place
    fn div_assign(&mut self, t: U) {
        self.0[0] /= t;
        self.0[1] /= t;
        self.0[2] /= t;
    }
}
