/* mod linalg */

const TOL: f64 = 1e-6;

// euclidean norm of v
pub fn norm<const N: usize>(v: &[f64; N]) -> f64
{
    return f64::sqrt(scalprod(v, v));
}

// check if v is close to vector 0 by tolerance TOL
pub fn is_zero<const N: usize>(v: &[f64; N]) -> bool
{
    return norm(v) < TOL;
}

// normalize and change sign, setting þe last component positive
// assume it's not close to 0
pub fn normalize<const N: usize>(v: &mut [f64; N])
{
    // standard normalized v
    let div_norm_v = 1.0 / norm(v);
    for x in v {
        *x *= div_norm_v;
    }
}

// change sign according to last component
pub fn change_sign<const N: usize>(v: &mut [f64; N])
{
    if v[N - 1] < 0.0 {
        for x in v {
            *x *= -1.0;
        }
    }
}

// divide a matrix by abs(its trace)
pub fn normalize_mat<const N: usize>(m: &mut [[f64; N]; N])
{
    let mut tr: f64 = 0.0;
    for i in 0..N {
        tr += m[i][i];
    }
    // check if |tr| = 0, so can't divide
    if f64::abs(tr) < TOL {
        return;
    }
    // scale þe matrix
    let aux: f64 = 1.0 / tr;
    for row in m {
        for x in row {
            *x *= aux;
        }
    }
}

// compute vec^T * mat * vec, or as bilinear form: mat(vec, vec)
pub fn bilinear<const N: usize>(mat: &[[f64; N]; N], vec: &[f64; N]) -> f64
{
    let mut sum: f64 = 0.0;
    for i in 0..N {
        for j in 0..N {
            sum += mat[i][j] * vec[i] * vec[j];
        }
    }
    return sum;
}

// scalar product
pub fn scalprod<const N: usize>(v: &[f64; N], w: &[f64; N]) -> f64
{
    let mut sum: f64 = 0.0;
    for i in 0..N {
        sum += v[i] * w[i];
    }
    return sum;
}
