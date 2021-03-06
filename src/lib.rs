use genmap::Handle;
mod desc_set_allocator;
mod elem_arithmetic;
mod engine;
mod matrix;
mod matrix_multiply;
mod scalar_ops;
mod sigmoid;
#[cfg(test)]
mod tests;
pub use engine::TensorSludge;

/// A handle referring to a matrix
#[derive(Copy, Clone)]
pub struct Matrix(pub(crate) Handle);

/// A handle referring to a pass
#[derive(Copy, Clone)]
pub struct Pass(pub(crate) Handle);

/// An operation to be executed on the GPU
#[derive(Copy, Clone)]
pub enum Operation {
    /// Perform matrix multiplication, dotting `left` and `right` and storing in `dst`. If either
    /// of the `_transpose` flags are set, operate the matrix multiplication as if
    /// the corresponding matrix was transposed.
    MatrixMultiply {
        left: Matrix,
        left_transpose: bool,
        right: Matrix,
        right_transpose: bool,
        dst: Matrix,
    },
    /// Set all values of this matrix to the sigmoid of their current value
    Sigmoid(Matrix),
    /// Set all values of this matrix to the sigmoid derivative of their current value; Note that
    /// this function expects the matrix in question to already have been passed through
    /// Sigmoid().
    SigmoidDerivative(Matrix),
    /// Performs the elementwise equivalent of `+=`
    InplaceAdd(Matrix, Matrix),
    /// Performs the elementwise equivalent of `-=`
    InplaceSub(Matrix, Matrix),
    /// Performs the elementwise equivalent of `*=`
    InplaceMultiply(Matrix, Matrix),
    /// Performs in-place scalar multiplication
    ScalarMultiply(Matrix, f32),
}
