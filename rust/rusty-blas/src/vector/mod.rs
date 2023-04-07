//! Vector operations

use libc::c_int;
use num::complex::{Complex32, Complex64};
use num::traits::NumCast;
use crate::vector::ops::{Asum, Axpy, Copy, Dot, Iamax, Nrm2, Scal};

pub mod ll;
pub mod ops;

//methods that allow a type to be used in blas functions as a vector/
pub trait Vector<T> {
    fn inc(&self) -> c_int {
        1
    }
    fn len(&self) -> c_int;
    fn as_ptr(&self) -> *const T;
    fn as_mut_ptr(&mut self) -> *mut T;
}

impl<'a, T> Into<Vec<T>> for &'a dyn Vector<T>
where
    T: Copy,
{
    fn into(self) -> Vec<T> {
        let n = self.len() as usize;
        let mut x = Vec::with_capacity(n);
        unsafe {
            x.set_len(n);
        }
        Copy::copy(self, &mut x);
        x
    }
}

pub trait VectorOperations<T>: Sized + Vector<T>
where
    T: Copy + Axpy + Scal + Dot + Nrm2 + Asum + Iamax,
{
    #[inline]
    fn update(&mut self, alpha: &T, x: &dyn Vector<T>) -> &mut Self {
        Axpy::axpy(alpha, x, self);
        self
    }

    #[inline]
    fn scale(&mut self, alpha: &T) -> &mut Self {
        Scal::scal(alpha, self);
        self
    }

    #[inline]
    fn dot(&self, x: &dyn Vector<T>) -> T {
        Dot::dot(self, x)
    }

    #[inline]
    fn abs_sum(&self) -> T {
        Asum::asum(self)
    }

    #[inline]
    fn norm(&self) -> T {
        Nrm2::nrm2(self)
    }

    #[inline]
    fn max_index(&self) -> usize {
        Iamax::iamax(self)
    }
}

impl<T> Vector<T> for Vec<T> {
    #[inline]
    fn len(&self) -> c_int {
        let l: Option<c_int> = NumCast::from(Vec::len(self));
        match l {
            Some(l) => l,
            //Find alternatives solutions for this case.
            None => panic!(),
        }
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        self[..].as_ptr()
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T {
        (&mut self[..]).as_mut_ptr()
    }
}

impl<T> Vector<T> for [T] {
    #[inline]
    fn len(&self) -> c_int {
        let l: Option<c_int> = NumCast::from(<[T]>::len(self));
        match l {
            Some(l) => l,
            //Find alternatives solutions for this case.
            None => panic!(),
        }
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        <[T]>::as_ptr(self)
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T {
        <[T]>::as_mut_ptr(self)
    }
}

macro_rules! operations_impl(
    ($v: ident, $($t: ty), +)=>(
        $( impl VectorOperations<$t> for $v<$t> {})+
    )
);

operations_impl!(Vec, f32, f64, Complex32, Complex64);
//impl<'a> VectorOperations<f32> for &'a [f32] {}
//impl<'a> VectorOperations<f64> for &'a [f64] {}
//impl<'a> VectorOperations<Complex32> for &'a [Complex32] {}
//impl<'a> VectorOperations<Complex64> for &'a [Complex64] {}
