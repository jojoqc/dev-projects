//I can provide you with some general tips and resources for learning Rust and building your own BLAS library:
//Start with the Rust documentation and the Rust book to get a good understanding of the syntax and fundamental concepts of the language.
//Research other existing BLAS libraries written in Rust to understand the best practices for implementing a BLAS library.
//Consider breaking down the project into smaller, manageable parts. For example, you could start with implementing a small number of the most common BLAS functions and then gradually add more.
//Make use of Rust's powerful type system and strict borrowing rules to ensure that your code is efficient, safe, and correct.
//Get involved with the Rust community by participating in forums, joining chat rooms, and reading Rust blogs. You can find these resources on the Rust website and through social media.
//Good luck with your project! Building a BLAS library is a challenging but rewarding task that will help you to master Rust and deepen your understanding of linear algebra and numerical computing.


//The second bullet point I mentioned was the importance of a solid understanding
//of linear algebra in order to effectively implement a BLAS library.
//Linear algebra is a mathematical framework for working with vector and matrix operations,
//and it is the foundation of many numerical algorithms.

//To implement a BLAS library, you'll need to be familiar with concepts like matrix multiplication,
//vector dot product, matrix-vector multiplication, and matrix inversion.
//You'll also need to understand how to perform these operations efficiently,
//which may involve the use of specialized algorithms like the Strassen algorithm or the BLAS standard.
//Additionally, you'll need to have a strong understanding of computer architecture and optimization techniques
// in order to ensure that your BLAS library is as fast and efficient as possible.

//what are the Strassen algorithm and the BLAS standard?

//The Strassen algorithm and the Basic Linear Algebra Subprograms (BLAS) standard are two different topics in computer science and numerical linear algebra.

//The Strassen algorithm, named after the German mathematician Volker Strassen,
// is a fast matrix multiplication algorithm. It was first published in 1969 and is a divide-and-conquer algorithm
// that allows matrix multiplication to be performed in fewer operations than the conventional matrix multiplication algorithm.
// The algorithm is based on the observation that the conventional algorithm uses 8 multiplications and 18 additions to multiply two matrices,
// while the Strassen algorithm uses only 7 multiplications and 18 additions. This results in a faster matrix multiplication process,
// but at the cost of more complicated arithmetic and more memory usage.


//The Basic Linear Algebra Subprograms (BLAS) standard, on the other hand,
// is a library interface specification that provides a standard set of functions
// for linear algebra operations such as vector-vector operations, matrix-vector operations,
// and matrix-matrix operations. The BLAS library is used in high-performance numerical computing,
// and its functions are optimized for specific hardware platforms. The BLAS standard was created
// to provide a unified interface for linear algebra libraries and allow for easy integration and
// interoperability between different software libraries and hardware platforms.

//In summary, the Strassen algorithm is a fast matrix multiplication algorithm that can perform
// the operation in fewer steps, while the BLAS standard provides a set of standard functions for
// linear algebra operations that allow for easy integration and interoperability between different
// software libraries and hardware platforms.



//here are some examples of optimization techniques used in linear algebra libraries:

//    Memory alignment: Making sure that data is stored in memory in a way that is optimized for processing.
// This can reduce the number of memory accesses required, resulting in faster computation times.

//    Loop unrolling: A technique that involves breaking up a loop into smaller segments so that each iteration
// of the loop performs multiple computations. This can reduce the number of branches in the code and increase parallelism,
// leading to improved performance.

//    Matrix transposition: Reordering the elements of a matrix so that they are stored in memory in a way
// that is optimized for computation. This can result in faster matrix multiplication operations and improved performance.

//    Parallelism: Making use of multiple processors or cores in a computer to perform computations in parallel.
// This can reduce the overall time taken to complete a computation and improve performance.

//    Vectorization: A technique that involves performing multiple computations in a single instruction,
// making use of vector processing units in modern CPUs. This can significantly improve performance
// compared to traditional scalar processing.

//    Blocking: A technique that involves dividing large matrices into smaller blocks that can be processed
// more efficiently. This can reduce the number of memory accesses required and increase the use of cache memory,
// leading to improved performance.

//These are just a few examples of optimization techniques that can be used in linear algebra libraries.
// The specific optimizations used will depend on the architecture of the system and the specific operations being performed.



//BLAS routine names have the following structure: <character> <name> <mod>[_64] The <character>
//field indicates the data type:

//s  means: real, single precision 
//d means: real, double precision 

//c means: complex, single precision 
//z means: complex, double precision 


//Some routines and functions can have combined character codes, such as sc or dz.
//For example, the function scasum uses a complex input array and returns a real value.
//The <name> field, in BLAS level 1, indicates the operation type.
//For example, the BLAS level 1 routines ?dot, ?rot, ?swap compute a vector dot product, vector rotation, and vector swap, respectively.
//In BLAS level 2 and 3, <name>
//reflects the matrix argument type: 

//ge means: general matrix 
//gb means: general band matrix 
//sy means: symmetric matrix 
//sp means: symmetric matrix (packed storage) 
//sb means: symmetric band matrix 
//he means: Hermitian matrix 
//hp means: Hermitian matrix (packed storage) 
//hb means: Hermitian band matrix 
//tr means: triangular matrix 
//tp means: triangular matrix (packed storage) 
//tb means: triangular band matrix. 

//The <mod>field, if present, provides additional details of the operation.
//BLAS LEVEL 1 names can have the following characters in the <mod> field:
//c  means: conjugated vector 
//u  means: unconjugated vector 
//g  means: Givens rotation construction 
//m  means: modified Givens rotation 
//mg means: modified Givens rotation construction 

//BLAS level 2 names can have the following characters in the <mod> field:
//mv means: matrix-vector product 
//sv means: solving a system of linear equations with a single unknown vector 
//r  means: rank-1 update of a matrix 
//r2 means: rank-2 update of a matrix. 

// BLAS level 3 names can have the following characters in the <mod> field:
// mm  means: matrix-matrix product 
// sm  means: solving a system of linear equations with multiple unknown vectors 
// rk  means: rank-k update of a matrix 
// r2k means: rank-2k update of a matrix. 

// The examples below illustrate how to interpret BLAS routine names:
// ddot  <d> <dot>: real and double precision, vector-vector dot product 
// cdotc <c> <dot> <c>: complex and single precision, vector-vector dot product, conjugated 
// cdotu <c> <dot> <u>: complex and single precision, vector-vector dot product, unconjugated 
// scasum <sc> <asum>: real and single-precision output, complex and single-precision input, sum of magnitudes of vector elements 
// sgemv <s> <ge> <mv>: real and single precision, general matrix, matrix-vector product 
// ztrmm <z> <tr> <mm> _64: complex and double precision, triangular matrix, matrix-matrix product, 64-bit integer type


// On 64-bit platforms, BLAS routines with the _64
// suffix support large data arrays in the LP64 interface library and enable you to mix integer types in one application.

// For example, when an application is linked with the LP64 interface library, SGEMM
// indexes arrays with the 32-bit integer type, while SGEMM_64
// indexes arrays with the 64-bit integer type. For more interface library details,
// see "Using the ILP64 Interface vs. LP64 Interface" in the developer guide.





//The Basic Linear Algebra Subprograms (BLAS) is a library that provides a standardized interface for linear algebra operations.
// The most common BLAS functions include:

//1-.dgemm (double-precision real matrix-matrix multiplication)
use ndarray::Array2;
use ndarray_blas::Dgemm;

fn main() {
    let a = Array2::<f64>::zeros((2, 3));
    let b = Array2::<f64>::zeros((3, 4));
    let c = Array2::<f64>::zeros((2, 4));
    let mut d = Array2::<f64>::zeros((2, 4));

    dgemm(1.0, &a, &b, 0.0, &mut d);
    assert_eq!(d, c);
}

//2-. daxpy (double-precision real constant times a vector plus a vector)
use ndarray::{Array1, Axis};
use ndarray_blas::Daxpy;

fn main() {
    let a = Array1::<f64>::zeros(3);
    let mut b = Array1::<f64>::zeros(3);

    daxpy(2.0, &a, &mut b);
    assert_eq!(b, Array1::zeros(3));
}

//3-. ddot (double-precision real dot product)
use ndarray::Array1;
use ndarray_blas::Ddot;

fn main() {
    let a = Array1::<f64>::zeros(3);
    let b = Array1::<f64>::zeros(3);

    let result = ddot(&a, &b);
    assert_eq!(result, 0.0);
}




// Building a BLAS library from scratch can be a complex and challenging task,
// as BLAS (Basic Linear Algebra Subprograms) provides a set of low-level routines
// for linear algebra operations that are commonly used in scientific computing.
// Here are some common BLAS functions and their equivalent implementations in Rust:

// 1-. dot product: This function calculates the dot product of two vectors.
fn dot(x: &[f64], y: &[f64]) -> f64 {
    let mut result = 0.0;
    for i in 0..x.len() {
        result += x[i] * y[i];
    }
    result
}

// 2-. matrix-vector multiplication: This function multiplies a matrix and a vector.
fn mat_vec_mul(a: &[[f64]], x: &[f64]) -> Vec<f64> {
    let m = a.len();
    let n = a[0].len();
    let mut y = vec![0.0; m];
    for i in 0..m {
        for j in 0..n {
            y[i] += a[i][j] * x[j];
        }
    }
    y
}

// 3-. matrix-matrix multiplication: This function multiplies two matrices.
fn mat_mat_mul(a: &[[f64]], b: &[[f64]]) -> Vec<Vec<f64>> {
    let m = a.len();
    let n = b[0].len();
    let k = a[0].len();
    let mut c = vec![vec![0.0; n]; m];
    for i in 0..m {
        for j in 0..n {
            for l in 0..k {
                c[i][j] += a[i][l] * b[l][j];
            }
        }
    }
    c
}

// These are just a few examples of the most common BLAS functions.
// However, building a full BLAS library from scratch would require
// implementing a larger set of functions, optimizing them for performance,
// and making sure they are robust and accurate.



//Here are the most common BLAS level 1 subroutines:
//1-. ddot - dot product of two vectors
//ddots returns the dot product of two vectors x and y
fn ddot(n: i32, x: &[f64], incx: i32, y: &[f64], incy: i32) -> f64 {
    let mut dot = 0.0;
    let mut ix = 0;
    let mut iy = 0;
    for _ in 0..n {
        dot += x[ix] * y[iy];
        ix += incx;
        iy += incy;
    }
    dot
}

// 2-. dasum - sum of absolute values of a vector
// dasum returns the sum of the absolute values of a vector x
fn dasum(n: i32, x: &[f64], incx: i32) -> f64 {
    let mut sum = 0.0;
    let mut ix = 0;
    for _ in 0..n {
        sum += x[ix].abs();
        ix += incx;
    }
    sum
}

// 3-. dnrm2 - Euclidean norm of a vector
// dnrm2 returns the Euclidean norm of a vector x
fn dnrm2(n: i32, x: &[f64], incx: i32) -> f64 {
    let mut norm = 0.0;
    let mut ix = 0;
    for _ in 0..n {
        norm += x[ix].powi(2);
        ix += incx;
    }
    norm.sqrt()
}

// 4-. daxpy - constant times a vector plus a vector
// daxpy adds a constant times a vector x to another vector y
fn daxpy(n: i32, alpha: f64, x: &[f64], incx: i32, y: &mut [f64], incy: i32) {
    let mut ix = 0;
    let mut iy = 0;
    for _ in 0..n {
        y[iy] += alpha * x[ix];
        ix += incx;
        iy += incy;
    }
}


// tell me about Rust simd and how to apply to llvm
// Rust SIMD (Single Instruction Multiple Data) is a feature in the Rust programming language
// that enables efficient use of vector instructions available on modern CPUs.
// The Rust SIMD library provides a high-level, safe, and convenient way to perform SIMD operations,
// which can significantly improve the performance of numerical computations.

// LLVM (Low-Level Virtual Machine) is a collection of modular and reusable
// compiler and toolchain technologies. It is used to compile various programming
// languages into machine code, including Rust.

// Rust's SIMD feature can be used with LLVM to generate highly optimized machine
// code for numerical computations. The Rust compiler uses LLVM to generate optimized
// machine code, and when the Rust SIMD library is used, it generates vector instructions
// that can perform multiple arithmetic operations in parallel, taking advantage of the vector units on the CPU.
// For example, to perform a dot product operation using Rust SIMD, the following code can be written:

use std::arch::x86_64::*; //SIMD PACKAGE FOR X86_64

// Load two vectors into XMM registers
let a = _mm_set_ps(1.0, 2.0, 3.0, 4.0);
let b = _mm_set_ps(4.0, 3.0, 2.0, 1.0);

// Perform the dot product
let res = _mm_dp_ps(a, b, 0xF1);

// Store the result
let mut result = [0.0; 4];
unsafe {
    _mm_store_ps(result.as_mut_ptr(), res);
}
println!("Result: {:?}", result[0]);




// With the simd library, you can define a struct that represents a vector with multiple
// elements of the same type, for example, f32x4 for a vector with four f32 elements.
// You can then use the overloaded operators provided by the simd library to perform
// operations on the vector, such as adding two vectors, multiplying a vector by a scalar, and so on.
// Here is an example of using Rust SIMD to add two vectors of f32 elements:


use simd::*;

fn main() {
    let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
    let b = f32x4::new(5.0, 6.0, 7.0, 8.0);
    let c = a + b;
    println!("{:?}", c);
}

// This code defines two vectors a and b, adds them together to get a third vector c, and prints the result.

// Note that the performance benefits of vectorization with SIMD depend on 
// the underlying hardware and the specifics of your code. It's always best 
// to measure the performance of your code with and without vectorization to 
// see if it provides the desired performance improvement.


// what are the differences between use simd::*; and use std::arch::x86_64::*; ?

// simd and std::arch::x86_64 are both modules in Rust that allow for SIMD (Single Instruction Multiple Data)
// vector operations, which allow for parallel computation on multiple data elements in a single instruction.
// The main difference between the two modules is the level of abstraction:
// simd provides a higher-level, more general API for SIMD vector operations 
// that works across multiple architectures, such as x86, ARM, and others. 
// This makes it easier to write portable code, but can come with some performance overhead due to the abstraction.
// std::arch::x86_64 is more low-level and provides a specialized API for x86-64 architecture,
// which is specific to this architecture. This provides the ability to write highly optimized
// code for x86-64 systems, but requires architecture-specific knowledge and is not portable to other architectures.
// Ultimately, the choice between simd and std::arch::x86_64 will depend on your specific use
// case and the trade-off between portability and performance that you're willing to make.





// Memory, loop unrolling, blocking, and vectorization are related to SIMD in the sense
// that they are all techniques that can be used to improve the performance of numerical computations.

// Memory refers to how data is stored and accessed in a computer's memory. In the context of numerical computations,
// it is important to minimize the amount of memory accesses and minimize the amount of cache misses, as accessing memory
// is one of the main performance bottlenecks.

// Loop unrolling is a technique that involves repeating a loop's body several times in order to reduce the overhead of the loop itself.
// This can lead to improved performance, especially for small loops that are executed many times.

// Blocking is a technique that involves dividing a large matrix into smaller blocks, and processing each block separately.
// This can improve performance by reducing the number of cache misses, as each block fits into the cache better than the entire matrix.

// Vectorization is a technique that involves processing multiple data elements in parallel,
// using vector instructions provided by the CPU. This can lead to substantial performance improvements,
//  as modern CPUs are designed to process vectors efficiently.

// SIMD (Single Instruction Multiple Data) is a hardware feature that allows a CPU to perform the same operation
// on multiple data elements in parallel. The term is often used interchangeably with vectorization, as vectorization
//  is a software technique that leverages the SIMD hardware feature.

// In conclusion, SIMD is a hardware feature, while vectorization is a software technique that leverages the SIMD hardware
// to improve performance. Memory, loop unrolling, and blocking are techniques that can be used in conjunction with vectorization
// to further improve performance.


// Here's a simple example of blocking technique in Rust:

// Matrix size
const N: usize = 512;

// Block size
const B: usize = 16;

// A, B, C matrices
let mut a = vec![0.0; N * N];
let mut b = vec![0.0; N * N];
let mut c = vec![0.0; N * N];

// Initialize matrices
for i in 0..N {
    for j in 0..N {
        a[i * N + j] = i as f64;
        b[i * N + j] = j as f64;
        c[i * N + j] = 0.0;
    }
}

// Matrix multiplication with blocking
for ii in (0..N).step_by(B) {
    for jj in (0..N).step_by(B) {
        for kk in (0..N).step_by(B) {
            for i in ii..ii + B {
                for j in jj..jj + B {
                    for k in kk..kk + B {
                        c[i * N + j] += a[i * N + k] * b[k * N + j];
                    }
                }
            }
        }
    }
}
// In this example, we first define the size of our matrices N and the block size B.
// Then we initialize the matrices a, b, and c. In the main loop of the matrix multiplication,
// we use the blocking technique by dividing the matrix into blocks of size B and computing the multiplication for each block.
// This way, we reduce the number of cache misses and improve performance by minimizing the amount of data that needs to be loaded from memory.


// The code example I provided earlier can be optimized
// using SIMD (Single Instruction, Multiple Data) vectorization.
// SIMD is a technology that allows the processor to perform the
// same operation on multiple data elements in parallel, potentially increasing performance.
// The idea behind SIMD is to perform multiple operations in a single
// instruction by processing multiple data elements in parallel.
// Here is an example of how the code can be optimized using SIMD in Rust:

use std::arch::x86_64::*;

fn simd_blocking(a: &[f32], b: &[f32], c: &mut [f32], block_size: usize) {
    let len = c.len();
    let blocks = len / block_size;
    let tail = len % block_size;

    for i in 0..blocks {
        let block_start = i * block_size;
        let block_end = block_start + block_size;

        unsafe {
            let a_block = _mm_loadu_ps(&a[block_start..block_end]);
            let b_block = _mm_loadu_ps(&b[block_start..block_end]);
            let c_block = _mm_add_ps(a_block, b_block);
            _mm_storeu_ps(&mut c[block_start..block_end], c_block);
        }
    }

    // handle tail elements
    for i in (blocks * block_size)..len {
        c[i] = a[i] + b[i];
    }
}

// In this example, the code uses the std::arch::x86_64 module to access the SSE (Streaming SIMD Extensions)
// instructions provided by the x86_64 architecture. The function takes two arrays a and b,
// and a mutable array c as input, and the block size as a parameter.
// The code performs the addition of elements of arrays a and b in blocks of size block_size and stores the result in c.
// The blocks are processed using the _mm_add_ps instruction provided by the SSE instructions set,
// which performs the addition of multiple f32 elements in parallel.

// Note that this is just one example of how SIMD vectorization can be applied to optimize
// the code, and the exact implementation may vary depending on the specific requirements and hardware constraints.




// loop unrolling can also be applied to optimize the code that uses the blocking technique.
// Loop unrolling involves duplicating the loop body a fixed number of times to reduce the
// number of branches and increase instruction-level parallelism. When used in combination
// with blocking and SIMD vectorization, loop unrolling can help to further increase the
// performance of matrix computation. However, care must be taken when implementing loop
// unrolling as it can also increase the code size and make it more complex.
fn add(a: &[f64], b: &[f64], c: &mut [f64]) {
    let n = a.len();
    let unroll_factor = 4;
    let mut i = 0;

    while i < n {
        let j = i..i + unroll_factor;
        c[j] = a[j] + b[j];
        i += unroll_factor;
    }
}


// This optimization technique is called loop unrolling, and it aims
// to reduce the overhead of loop control instructions. By unrolling
// the loop and adding four additions at a time instead of one, we can
// reduce the number of loop control instructions and make the loop body longer,
// allowing the processor to work on more instructions without waiting for the
// next iteration. This can improve the performance of the code, but the improvement
// depends on the specific processor and the data being processed.


// Here is an example of loop unrolling in Rust using the simd crate:
use simd::{f64x2, f64x4};
#[inline(always)]
fn add_unrolled(a: &[f64], b: &[f64], c: &mut [f64]) {
    let n = a.len();
    let unroll_factor = 4;
    let mut i = 0;

    while i < n {
        let j = i..i + unroll_factor;
        let a_simd = f64x4::from_slice_unaligned(&a[j]);
        let b_simd = f64x4::from_slice_unaligned(&b[j]);
        let c_simd = a_simd + b_simd;
        c_simd.write_to_slice_unaligned(&mut c[j]);
        i += unroll_factor;
    }
}


// In this example, we are using the f64x4 type from the simd crate
// to unroll the loop into 4-element vectors. The from_slice_unaligned
// and write_to_slice_unaligned methods allow us to convert slices of
// f64 values into vector types and back.
// For blocking, consider the following example:
use simd::{f64x2, f64x4};
#[inline(always)]
fn add_blocked(a: &[f64], b: &[f64], c: &mut [f64]) {
    let n = a.len();
    let block_size = 256;
    let unroll_factor = 4;

    for block_start in (0..n).step_by(block_size) {
        let block_end = std::cmp::min(block_start + block_size, n);
        let mut i = block_start;

        while i < block_end {
            let j = i..i + unroll_factor;
            let a_simd = f64x4::from_slice_unaligned(&a[j]);
            let b_simd = f64x4::from_slice_unaligned(&b[j]);
            let c_simd = a_simd + b_simd;
            c_simd.write_to_slice_unaligned(&mut c[j]);
            i += unroll_factor;
        }
    }
}
// In this example, we are using the outer loop to iterate over blocks of size block_size
// and the inner loop to unroll the computation. This allows us to make better use of the
// cache, since each iteration of the inner loop will only access a small, contiguous region of memory.


// Here's an example of using SIMD in RISC-V and ARM architectures:
#[target_feature(enable = "simd128")]
unsafe fn simd_add(a: &[u32], b: &[u32], c: &mut [u32]) {
    let n = a.len();
    let mut i = 0;

    while i < n {
        let a_chunk = simd::u32x4::load(a, i);
        let b_chunk = simd::u32x4::load(b, i);
        let c_chunk = a_chunk + b_chunk;
        c_chunk.store(c, i);
        i += 4;
    }
}
// ARM:
#[target_feature(enable = "neon")]
unsafe fn simd_add(a: &[u32], b: &[u32], c: &mut [u32]) {
    let n = a.len();
    let mut i = 0;

    while i < n {
        let a_chunk = vld1q_u32(a.as_ptr().add(i));
        let b_chunk = vld1q_u32(b.as_ptr().add(i));
        let c_chunk = vaddq_u32(a_chunk, b_chunk);
        vst1q_u32(c.as_mut_ptr().add(i), c_chunk);
        i += 4;
    }
}


// To optimize your code using LLVM, you can use the llvm-rustc
// command line tool, which is part of the LLVM suite of tools.
// The tool takes the output from the Rust compiler and performs various
// optimizations on the generated LLVM IR (Intermediate Representation)
// code, such as vectorization, loop unrolling, and inlining.

// Here's an example of how to use llvm-rustc to optimize your code for the RISC-V architecture:
// $ rustc -O --target riscv64-unknown-none-elf my_rust_program.rs
// $ llvm-objcopy --strip-all my_rust_program my_rust_program_stripped
// $ llc -march=riscv -O3 my_rust_program_stripped.bc
// $ clang -o my_optimized_riscv_program my_rust_program_stripped.s -lm

// And here's an example of how to use llvm-rustc to optimize your code for the ARM architecture:
// $ rustc -O --target arm-unknown-linux-gnueabihf my_rust_program.rs
// $ llvm-objcopy --strip-all my_rust_program my_rust_program_stripped
// $ llc -march=arm -O3 my_rust_program_stripped.bc
// $ clang -o my_optimized_arm_program my_rust_program_stripped.s -lm


































//Here is an example of loop unrolling in Rust using the simd crate:
use simd::{f64x2, f64x4};

#[inline(always)]
fn add_unrolled(a: &[f64], b: &[f64], c: &mut [f64]) {
    let n = a.len();
    let unroll_factor = 4;
    let mut i = 0;

    while i < n {
        let j = i..i + unroll_factor;
        let a_simd = f64x4::from_slice_unaligned(&a[j]);
        let b_simd = f64x4::from_slice_unaligned(&b[j]);
        let c_simd = a_simd + b_simd;
        c_simd.write_to_slice_unaligned(&mut c[j]);
        i += unroll_factor;
    }
}

//In this example, we are using the f64x4 type from the simd crate to unroll
//the loop into 4-element vectors. The from_slice_unaligned and write_to_slice_unaligned
//methods allow us to convert slices of f64 values into vector types and back.

//For blocking, consider the following example:
use simd::{f64x2, f64x4};
#[inline(always)]
fn add_blocked(a: &[f64], b: &[f64], c: &mut [f64]) {
    let n = a.len();
    let block_size = 256;
    let unroll_factor = 4;

    for block_start in (0..n).step_by(block_size) {
        let block_end = std::cmp::min(block_start + block_size, n);
        let mut i = block_start;

        while i < block_end {
            let j = i..i + unroll_factor;
            let a_simd = f64x4::from_slice_unaligned(&a[j]);
            let b_simd = f64x4::from_slice_unaligned(&b[j]);
            let c_simd = a_simd + b_simd;
            c_simd.write_to_slice_unaligned(&mut c[j]);
            i += unroll_factor;
        }
    }
}

//In this example, we are using the outer loop to iterate over blocks of
// size block_size and the inner loop to unroll the computation. This allows us
// to make better use of the cache, since each iteration of the inner loop will
// only access a small, contiguous region of memory.













// INTERESTING MACROS TO FUTHER STUDY
// #![feature(asm)]
// #![feature(llvm_asm)]
// #![feature(optin_builtin_traits)]
// #![feature(prelude_import)]
// #![feature(unboxed_closures)]
// #![feature(const_fn)]
// #![feature(const_raw_ptr_deref)]
// #![feature(const_transmute)]

// #[inline(always)]