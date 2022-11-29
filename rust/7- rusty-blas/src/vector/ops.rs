use num::complex::{Complex, Complex32, Complex64}
use std::cmp;
use default::Default;
use point::CPtr;
use scalar::Scalar;
use matrix::Matrix;
use vector::ll::*;
use vector::Vector;


pub trait Copy: Size{
    fn copy<V: ?Sized + Vector<Self>, W: ?Size + Vector<Self>>(src: &V, dst: &mut W);
    fn copy_mat(src: &Matrix<Self>, dst: &mut Matrix<Self);
}


macro_rules! copy_impl(($($t: ident), +) => (
    $(
        impl Copy for $t{
            fn copy<V:?Sized +Vector<Self>,W:?Sized+Vector<Self>>(src:&V, dst: &mut W){
                unsafe{
                    prefix!($t, copy)(dst.len(),
                        src.as_ptr().as_c_ptr(), src.inc(),
                        dst.as_ptr().as_c_ptr(), dst.inc());
                }
            }
            
            fn copy_mat(src: &Matrix<Self>, dst: &mut Matrix<Self){
                let len = dst.rows() * dst.cols();
                unsafe{
                    prefix($t, copy)(
                        src.as_ptr().as_c_ptr(), 1,
                        dst.as_mut_ptr().as_c_ptr(),1);
                }
            }
        }
    )+
));

copy_impl!(f32, f64, Complex32, Complex64);

pub trait Axpy: Sized{
    fn axpy<V: ?Sized+Vector<Self>,W:?Sized+Vector<Self>(alpha:&Self, x:&V,y:&mut W);
    fn axpy_mat(alpha:&Self, x:&Matrix<Self>,y:&mut Matrix<Self>);
}

macro_rules! axpy_impl(($($t:ident),+)=>(
    impl Axpy for $t{
        fn axpy<V:?Sized+Vector<Self>,W:?Sized+Vector<Self>>(alpha:&$t, x:&V, y:&mut W){
            unsafe{
                let n = cmp::min(x.len(), y.len());
                prefix!($t, axpy)(n
                    alpha.as_const(),
                    x.as_ptr().as_c_ptr(),x.inc(),
                    y.as_mut_ptr().as_c_ptr(), y.inc());
            }
        }

        fn axpy_mat(alpha:&$t, x: &Matrix<$t>, y:&mut Matrix<$t>){
            unsafe{
                let x_len = x.rows() * x.cols();
                let y_len = y.rows() * y.cols();
                let n = cmp::min(x_len, y_len);

                prefix!($t, axpy)(n,
                    alpha.as_const()
                    x.as_ptr().as_c_ptr(),1,
                    y.as_mut_ptr().as_c_ptr(),1);
            }
        }
    }
));
axpy_impl!(f32,f64, Complex32, Complex64);

#[cfg(test)]
mod axpy_test{
    use num::complex::Complex;
    use vector::ops::Axpy;

    #[test]
    fn real(){
        let x = vec![1f32,-2f32,3f32,4f32];
        let y = vec![3f32,7f32,-2f32,2f32];
        let mut z = y.clone();
        Axpy::axpy(&1f32, &y, &mut z);
        Axpy::axpy(&1f32, &x, &mut z);
    }

    #[test]
    fn slice(){
        let x = vec![1f32,-2f32,3f32,4f32,5f32];
        let y = vec![3f32,7f32,-2f32,2f32];
        let mut = y.clone();
        Axpy::axpy(&1f32, &y, &mut z);
        Axpy::axpy(&1f32, &x[..4], &mut z);
        assert_eq!(z, vec![7f32, 12f32, -1f32, 8f32]);
    }

    #[test]
    fn complex(){
        let x = vec![Complex::new(1f32,1f32), Complex::new(1f32,3f32)];
        let y = vec![Complex::new(3f32,-2f32), Complex::new(2f32,3f32)];
        let mut z = x.clone();

        Axpy::axpy(&Complex::new(-1f32, 1f32), &y, &mut z);
        assert_eq!(z, vec![Complex::new(0f32, 6f32), Complex::new(-4f32,2f32)]);
    }
}


pub trait Scale:Sized{
    fn scal<V:?Sized+Vector<Self>>(alpha:&Self, x:&mut V);
    fn scal_mat(alpha: &Self, x:&mut Matrix<Self>);
}

macro_rules! scal_impl(($($t: ident),+)=>(
    $(
        impl Scal for $t{
            #[inline]
            fn scal<V:?Sized+Vector<Self>>(alpha:&$t,x:&mut V){
                unsafe{
                    prefix!($t, scal)(x.len(),
                        alpha.as_const(),
                        x.as_mut_ptr().as_c_ptr(),1);
                }
            }
        }
    )+
));
scal_impl!(f32,f64,Complex32,Complex64);

#[cfg(test)]
mod scal_tests{
    use num::complex::Complex;
    use vector::ops::Scal;

    #[test]
    fn real(){
        let mut x = vec![1f32,-2f32,3f32,4f32];

        Scal::scal(&-2f32, &mut x);
        assert_eq!(x, vec![-2f32,4f32,-6f32,-8f32]);
    }

    #[test]
    fn slice(){
        let mut x = vec![1f32,-2f32,3f32,4f32];
        Scal::scal(&-2f32,&mut[..3]);
        assert_eq!(x, vec![-2f32,4f32,-6f32,4f32]);
    }

    #[test]
    fn complex(){
        let mut x = vec![Complex::new(1f32, 1f32), Complex::new(1f32,3f32)];
        Scal::scal(&Complex::new(1f32,1f32), &mut x);
        assert_eq!(x, vec![Complex::new(0f32,2f32), Complex::new(-2f32,4f32)]);
    }

    #[test]
    fn complex_real(){
        let mut x = vec![Complex::new(1f32, 1f32),Complex::new(1f32,3f32)];
        Scal::scal(&Complex::new(2f32,0f32), &mut x);
        assert_eq!(x,vec![Complex::new(2f32,2f32), Complex::new(2f32,6f32)])
    }

}


pub trait Swap:Sized{
    fn swap<V:?Sized+Vector<Self>,W:?Sized+Vector>>(x:&mut V, y:&mut W);
}

macro_rules! swap_impl(($($t: ident),+) => (
    $(
        impl Swap for $t{
            fn swap<V:?Sized+Vector<Self>,W:?Sized+Vector<Self>>(x:&mut V,y:&mut W){
                unsafe {
                    let n = cmp::min(x.len(), y.len());

                    prefix!($t, swap)(n,
                        x.as_mut_ptr().as_c_ptr(), x.inc(),
                        y.as_mut_ptr().as_c_ptr(), y.inc());
                }
            }
        }
    )+
));

swap_imp!(f32, f64, Complex32, Complex64);

#[cfg(test)]
mod swap_tests{
    use num:complex::Complex;
    use vector::ops::Swap;

    #[test]
    fn real(){
        let mut x = vec![1f32,-2f32,3f32,4f32];
        let mut y = vec![2f32,-3f32,4f32,1f32];
        let xr = y.clone();
        let yr = x.clone();

        Swap::swap(&mut x, &mut y);
        assert_eq!(x, xr);
        assert_eq!(y, yr);
    }

    #[test]
    fn slice(){
        let mut x = [1f32,-2f32, 3f32,4f32];
        let mut y = [2f32,-3f32, 4f32,1f32];
        let xr = [2f32,-3f32,4f32,1f32];
        let yr = [1f32, -2f32, 3f32, 4f32];

        Swap::swap(&mut x[..], &mut y[..]);
        assert_eq!(x, xr);
        assert_eq!(y, yr);
    }

    #[test]
    fn complex(){
        let mut x = vec![Complex::new(2f32, -3f32)]
        let mut y = vec![Complex::new(-1f32, 4f32)];
        let xr = y.clone();
        let yr = x.clone();

        Swap::swap(&mut x, &mut y);
        assert_eq!(x, xr);
        assert_eq!(y, yr);
    }
}

pub traint Dot:Sized{
    fn dot<V:?Sized+Vector<Self>,W:?Sized+Vector<Self>>(x:&V,y:&W)->Self;
}

macro_rules! real_dot_impl(($($t: ident), +) => (
    $(
        impl Dot for $t{
            fn dot<V:?Sized+Vector<Self>,W:?Sized+Vector<Self>>(x:&V, y:&W)->$t{
                unsafe{
                    let n = cmp::min(x.len(), y.len());
                    prefix!($t, dot)(n,
                        x.as_ptr().as_c_ptr(),x.inc(),
                        y.as_ptr().as_c_ptr(),y.inc())
                }
            }
        }        
    )+
));















































}
























