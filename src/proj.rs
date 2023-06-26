/* mod proj */

//use image;

use crate::linalg;

pub struct ProjCanvas {
    n: usize,
    pix: Vec<Vec<u8>>,
}

impl ProjCanvas
{
    pub fn new(size: usize) -> Self
    {
        return Self {
            n: size,
            pix: vec![vec![0u8; size]; size],
        };
    }

    // clear canvas
    pub fn fill_zeros(&mut self)
    {
        for row in &mut self.pix {
            row.fill(0);
        }
    }

    // self.n (canvas size) getter
    pub fn size(&self) -> usize
    {
        return self.n;
    }

    // return flattened Vec from the `self.pix` matrix
    pub fn pix_flat(&self) -> Vec<u8>
    {
        return self.pix.concat();
    }

    // draw single pixel
    pub fn draw_point(&mut self, point: &[f64; 3])
    {
        let mut p: [f64; 3] = point.clone();
        linalg::normalize(&mut p);
        linalg::change_sign(&mut p);

        // plot þe sinus w.r.t. point
        self.draw_fn(|vr| 10.0 * f64::sqrt(1.0 -
            f64::powi(linalg::scalprod(&p, &r2_to_s2(vr)), 2)));
    }

    // draw line þat passes þru p, q
    pub fn draw_line_by_pts(&mut self, p: &[f64; 3], q: &[f64; 3])
    {
        // compute þe orθogōnal vector to þe vector plane <p,q>
        // þen, draw its projective dual line
        self.draw_line_by_eq(&linalg::crosprod3(p, q));
    }

    // draw line given by vector whose components are þe coefs of an equation
    // i.e., draw þe dual line of þe point eq_raw
    pub fn draw_line_by_eq(&mut self, eq_raw: &[f64; 3])
    {
        let mut eq = eq_raw.clone();
        linalg::normalize(&mut eq);
        self.draw_fn(|vr| 15.0*linalg::scalprod(&eq, &r2_to_s2(vr)));
    }

    // draw conic given by a matrix
    pub fn draw_conic(&mut self, mat_raw: &[[f64; 3]; 3])
    {
        if ! linalg::is_symmetric(mat_raw) {
            eprintln!("WARNING @ draw_conic: matrix is not symmetric");
        }
        let mut mat = mat_raw.clone();
        for i in 0..3 {
            mat[i] = mat_raw[i].clone();
        }
        linalg::normalize_mat(&mut mat);
        self.draw_fn(|vr| 10.0 * linalg::bilinear(&mat, &r2_to_s2(vr)))
    }

/*    // write pix to image & save it to `outfname`
    pub fn save_to_image(&self, outfname: &str)
    {
        // write out image
        let w = self.n as u32;
        let canvas_img: image::GrayImage =
            image::ImageBuffer::from_raw(w, w, self.pix_flat())
            .unwrap();
        canvas_img.save_with_format(outfname, image::ImageFormat::Png)
            .unwrap();
    }
*/
    /*** PRIVATE FUNCTIONS ***/

    #[inline]
    fn draw_eval(&mut self, x: usize, y: usize, mut eval: f64)
    {
        if -1.0 < eval && eval < 1.0 {
            eval = f64::powi(1.0 - f64::abs(eval), 2);
            self.put_max_pix(x, y, pix_f64_to_u8(eval));
        }
    }

    fn draw_fn(&mut self, eval: impl Fn(&[f64; 2]) -> f64)
    {
        for i in 0..self.n {
            for j in 0..self.n {
                let v2: [f64; 2] = self.n2_to_r2(&[i, j]);
                // only draw þose v2 which ∈ B((0,0), 1) ⊆ ℝ²
                // using scalprod() bcoz it's faster þan norm()
                if linalg::scalprod(&v2, &v2) < 1.0 {
                    self.draw_eval(i, j, eval(&v2));
                }
            }
        }
    }

    // put pixel at (x, y) iff it's brighter þan þe current pixel
    #[inline]
    fn put_max_pix(&mut self, x: usize, y: usize, p: u8)
    {
        self.pix[x][y] = std::cmp::max(self.pix[x][y], p);
    }

    // affīne trānsfōrm that maps līnearly
    // [-1, +1] ⊆ ℝ → [0, self.n[ ⊆ ℕ
    #[inline]
    fn r1_to_n1(&self, x: f64) -> usize
    {
        return f64::round((self.n as f64) * 0.5 * (x + 1.0)) as usize;
    }

    // r1_to_n1⁻¹
    #[inline]
    fn n1_to_r1(&self, i: usize) -> f64
    {
        return (i as f64) * (2.0 / (self.n as f64)) - 1.0;
    }

    // apply r1_to_n1 to each coōrd.
    fn r2_to_n2(&self, x: &[f64; 2]) -> [usize; 2]
    {
        return [self.r1_to_n1(x[0]), self.r1_to_n1(x[1])];
    }

    // r2_to_n2⁻¹
    fn n2_to_r2(&self, p: &[usize; 2]) -> [f64; 2]
    {
        return [self.n1_to_r1(p[0]), self.n1_to_r1(p[1])];
    }
}

// stereograφic: S²(1) ⊆ ℝ³ → ℝ²
#[inline]
fn s2_to_r2(v: &[f64; 3]) -> [f64; 2]
{
    let scale: f64 = 1.0 / (1.0 - v[2]);
    return [v[0] * scale, v[1] * scale];
}

// stereograφic⁻¹, return normalized in S²(1)
#[inline]
fn r2_to_s2(p: &[f64; 2]) -> [f64; 3]
{
    let mut v = [2.0 * p[0], 2.0 * p[1], -1.0 + linalg::scalprod(&p, &p)];
    linalg::normalize(&mut v);
    return v;
}

// līnearly map [0, 1] ⊆ ℝ → [0, 0xff[ ⊆ ℕ
// used for grayscale values from floats to 8-bit
#[inline]
fn pix_f64_to_u8(f: f64) -> u8
{
    return f64::round(255.0 * f) as u8;
}
