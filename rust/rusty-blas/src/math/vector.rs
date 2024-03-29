use std::ops::{Add, Mul};

use crate::default::Default;
use crate::math::Trans;
use crate::vector::ops::*;
use crate::vector::Vector;
use num::complex::{Complex32, Complex64};

impl<'a, T> Add for &'a dyn Vector<T>
where
    T: Axpy + Copy + Default,
{
    type Output = Vec<T>;

    fn add(self, x: &dyn Vector<T>) -> Vec<T> {
        let mut result: Vec<_> = self.into();
        let scale = Default::one();
        Axpy::axpy(&scale, x, &mut result);
        result
    }
}

impl<'a, T> Mul<&'a dyn Vector<T>> for Trans<&'a dyn Vector<T>>
where
    T: Sized + Copy + Dot + Dotc,
{
    type Output = T;

    fn mul(self, x: &dyn Vector<T>) -> T {
        match self {
            Trans::T(v) => Dot::dot(v, x),
            Trans::H(v) => Dotc::dotc(v, x),
        }
    }
}

impl<'a, T> Mul<T> for &'a dyn Vector<T>
where
    T: Sized + Copy + Scal,
{
    type Output = Vec<T>;

    fn mul(self, alpha: T) -> Vec<T> {
        let mut result: Vec<_> = self.into();
        Scal::scal(&alpha, &mut result);
        result
    }
}

macro_rules! left_scale(($($t: ident), +) => (
    $(
        impl<'a> Mul<&'a dyn Vector<$t>> for $t{
            type Output = Vec<$t>;

            fn mul(self, x: &dyn Vector<$t>) -> Vec<$t>{
                let mut result: Vec<_> = x.into();
                Scal::scal(&self, &mut result);
                result
            }
        }
    )+
));

left_scale!(f32, f64, Complex32, Complex64);

#[cfg(test)]
mod tests {
    use crate::math::Marker::H;
    use crate::Vector;
    use num::Complex;

    #[test]
    fn add() {
        let x = vec![1f32, 2f32];
        let y = vec![3f32, 4f32];

        let z = &x as &(dyn Vector<_> + y);
        assert_eq!(&z, &vec![4f32, 6f32]);
    }

    #[test]
    fn herm_dot() {
        let x = vec![Complex::new(1f32, -1f32), Complex::new(1f32, -3f32)];
        let y = vec![Complex::new(1f32, 2f32), Complex::new(1f32, 3f32)];

        let dot = {
            let z = &x as &dyn Vector<_>;
            (z ^ H) * &y
        };
    }

    #[test]
    fn scale() {
        let x = vec![1f32, 2f32];
        let xr = &x as &dyn Vector<_>;

        let y = xr * 3.0;
        let z = 3.0 * xr;
        assert_eq!(y, vec![3f32, 6f32]);
        assert_eq!(z, y);
    }
}
