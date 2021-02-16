use std::convert::From;
use std::error::Error;
use std::ops::{Div, Mul};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Ratio<T> {
    a: T,
    b: T,
}

impl<T> Ratio<T> {
    pub fn a_to_b<U>(self, x: U) -> <<U as Mul<T>>::Output as Div<T>>::Output
    where
        U: Mul<T>,
        <U as Mul<T>>::Output: Div<T>,
    {
        x * self.b / self.a
    }
}

impl<T> FromStr for Ratio<T>
where
    T: FromStr,
    <T as FromStr>::Err: Error + 'static,
{
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let a = parts.next().ok_or("Empty string")?.parse()?;
        let b = parts.next().ok_or("Empty string")?.parse()?;
        Ok(Ratio { a, b })
    }
}

impl From<Ratio<u32>> for f64 {
    fn from(r: Ratio<u32>) -> f64 {
        r.a as f64 / r.b as f64
    }
}
