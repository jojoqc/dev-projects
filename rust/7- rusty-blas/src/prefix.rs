macro_rules! prefix(
    (f32, $f: ident) => (cblas_s::$f);
    (f64, $f: ident) => (cblas_d::$f);
    (Complex<f32>, $f: ident)=>(cblas_c::$f);
    (Complex<f64>, $f: ident)=>(cblas_z::$f);
    (Complex32, $f: ident)=>(cblas_c::$f);
    (Complex64, $f: ident)=>(cblas_z::$f);
);
