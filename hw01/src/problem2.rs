/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
/// mat1: n x m ; mat2: m x p
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let m1cols = mat1.len();
    let m2rows = mat2[0].len();
    assert_eq!(m1cols, m2rows);

    let n = mat1[0].len();
    let m = m1cols;
    let p = mat2.len();

    let mut result = vec![vec![0.;n]; p];
    for i in 0..n {
        for j in 0..p {
            let mut sum: f32 = 0.0;
            for k in 0..m {
                sum += mat1[i][k] * mat2[k][j];
            }
            result[i][j] = sum;
        }
    }
    result
}