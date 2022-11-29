extern crate libc;
extern crate num;

pub use vector::Vector;
pub use vector::VectorOperations;
pub use matrix::Matrix;
pub use vector::ops::*;
pub use matrix_vector::ops::*;
pub use matrix::ops::*;


#[macro_use]
mod prefix;
mod pointer;
mod scalar;

pub mod attribute;
pub mod default;
pub mod vector;
pub mod vector;
pub mod matrix_vector;
pub mod matrix;
pub mod math;
