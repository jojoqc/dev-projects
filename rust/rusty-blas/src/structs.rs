//LEVEL-1V opetarions on vectors
struct _addv{}
struct _amaxv{}
//a*x + y
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
struct _subm{}

//LEVEL-1M element-wise operations on matrices
struct _addm{}
struct _axpym{}
struct _copym{}
struct _invscalm{}
struct _scalm{}
struct _scal2m{}
struct _setm{}
struct _subm{}


//Sum of vector magnitudes
struct asum{}
//Scalar-vector product
struct axpy{}
//Copy vector
struct copyv{}
//Dot product
struct dotv{}
//Dot product with double precision
struct sdsdot{}
//Dot product conjugated
struct dotc{}
//Dot product unconjugated
struct dotu{}
//Vector 2-norm (Euclidean norm)
struct nrm2{}
//Plane rotation of points
struct rot{}
//Generate Givens rotation of points
struct rotg{}
//Modified Givens plane rotation of points
struct rotm{}
//Generate modified Givens plane rotation of points
struct rotmg{}
//Vector-scalar products
struct scal{}
//Vector-vector swap
struct swap{}
//Index of the maximum absolute value element of a vector
struct iamax{}
//Index of the minimum absolute value element of a vector
struct iamin{}
