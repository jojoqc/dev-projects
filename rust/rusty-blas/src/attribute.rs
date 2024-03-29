#[repr(C)]
#[derive(Copy,Clone)]
pub enum Order{
    RowMajor=101,
    ColMajor=102,
}

#[repr(C)]
#[derive(Copy,Clone)]
pub enum Transpose{
    NoTrans=111,
    Trans=112,
    ConjTrans=113,
}
#[repr(C)]
#[derive(Copy,Clone)]
pub enum Symmetry{
    Upper=121,
    Lower=122,
}
#[repr(C)]
#[derive(Copy,Clone)]
pub enum Diagonal{
    NonUnit=131,
    Unit=132,
}

#[repr(C)]
#[derive(Copy,Clone)]
pub enum Side{
    Left=141,
    Right=142,
} 





