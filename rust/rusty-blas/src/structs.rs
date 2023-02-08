//LEVEL-1V opetarions on vectors
struct _addv{}
struct _amaxv{}
struct _axpyv{}
struct _copyv{}
struct _dotv{}
struct _dotxv{}
struct _invertv{}
struct _invscalv{}
struct _scal2v{}
struct _setv{}
struct _subv{}
struct _swapv{}

//LEVEL-1D Element-wise operations on matrix diagonals
struct _addd{}
struct _axpyd{}
struct _copyd{}
struct _invertd{}
struct _invscald{}
struct _scald{}
struct _scal2d{}
struct _setd{}
struct _setid{}
struct _submd{}

//LEVEL-1M element-wise operations on matrices
struct _addm{}
struct _axpym{}
struct _copym{}
struct _invscalm{}
struct _scalm{}
struct _scal2m{}
struct _setm{}
struct _subm{}


//BLAS LEVEL-1 SIMPLEX
struct Asum{}//Sum of vector magnitudes
struct Axpy{}//Scalar-vector product
struct Copyv{}//Copy vector
struct Dotv{}//Dot product
struct Sdsdot{}//Dot product with double precision
struct Dotc{}//Dot product conjugated
struct Dotu{}//Dot product unconjugated
struct Nrm2{}//Vector 2-norm (Euclidean norm)
struct Rot{}//Plane rotation of points
struct Rotg{}//Generate Givens rotation of points
struct Rotm{}//Modified Givens plane rotation of points
struct Rotmg{}//Generate modified Givens plane rotation of points
struct Scal{}//Vector-scalar products
struct Swap{}//Vector-vector swap
struct Iamax{}//Index of the maximum absolute value element of a vector
struct Iamin {}//Index of the minimum absolute value element of a vector
