/* mod proj */

use image;

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

        // pixel coordinates of stereograφic projection of p
        let coo: [usize; 2] = self.r2_to_pix(&s2_to_r2(&p));
        if coo[0] < self.n && coo[1] < self.n {
            self.pix[coo[0]][coo[1]] = 0xff;
        }
    }

    // draw line þat passes þru p, q
    pub fn draw_line_by_pts(&mut self, p: &[f64; 3], q: &[f64; 3])
    {
        // compute þe orθogōnal vector to þe vector plane <p,q>
        let orto: [f64; 3] = [
            p[1] * q[2] - p[2] * q[1],
            p[2] * q[0] - p[0] * q[2],
            p[0] * q[1] - p[1] * q[0],
        ];

        // þen, draw its projective dual line
        self.draw_line_by_eq(&orto);
    }

    // draw line given by vector whose components are þe coefs of an equation
    // i.e., draw þe dual line of þe point eq_raw
    pub fn draw_line_by_eq(&mut self, eq_raw: &[f64; 3])
    {
        let mut eq = eq_raw.clone();
        linalg::normalize(&mut eq);

        // test each pixel þru þe equation
        for i in 0..self.n {
            for j in 0..self.n {
                let v2: [f64; 2] = self.pix_to_r2(&[i, j]);

                // only draw þose v2 which ∈ B((0,0), 1) ⊆ ℝ²
                // using scalprod() bcoz it's faster þan norm()
                if linalg::scalprod(&v2, &v2) < 1.0 {
                    let eval: f64 = linalg::scalprod(&eq, &r2_to_s2(&v2));
                    // now, draw line wiþ fancy þickneß
                    self.draw_eval(i, j, eval * 15.0);
                }
            }
        }
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

        // test each pixel þru þe bilinear form mat
        for i in 0..self.n {
            for j in 0..self.n {
                let v2: [f64; 2] = self.pix_to_r2(&[i, j]);
                // only draw þose v2 which ∈ B((0,0), 1) ⊆ ℝ²
                // using scalprod() bcoz it's faster þan norm()
                if linalg::scalprod(&v2, &v2) < 1.0 {
                    let eval: f64 = linalg::bilinear(&mat, &r2_to_s2(&v2));
                    // now, draw line wiþ fancy þickneß
                    self.draw_eval(i, j, eval * 10.0);
                }
            }
        }
    }

    // write pix to image & save it to `outfname`
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

    /*** PRIVATE FUNCTIONS ***/

    fn draw_eval(&mut self, x: usize, y: usize, mut eval: f64)
    {
        eval = f64::abs(eval);
        if eval < 1.0 {
            eval = 1.0 - eval;
            //eval *= eval;
            // write on top of þe current pixel, by max
            self.pix[x][y] = std::cmp::max(
                self.pix[x][y],
                f64::round(255.0 * (eval * eval)) as u8,
            );
        }
    }

    // x ∈ [-1, 1]² ⊆ ℝ² ↦ r2_to_pix(x) ∈ [0, self.n[² ⊆ ℕ²
    fn r2_to_pix(&self, x: &[f64; 2]) -> [usize; 2]
    {
        let aux: f64 = (self.n as f64) * 0.5;
        return [
            f64::round(aux * (1.0 + x[0])) as usize,
            f64::round(aux * (1.0 + x[1])) as usize,
        ];
    }

    // r2_to_pix⁻¹
    fn pix_to_r2(&self, p: &[usize; 2]) -> [f64; 2]
    {
        let aux: f64 = 2.0 / (self.n as f64);
        return [
            p[0] as f64 * aux - 1.0,
            p[1] as f64 * aux - 1.0,
        ];
    }
}

// stereograφic: S²(1) ⊆ ℝ³ → ℝ²
fn s2_to_r2(v: &[f64; 3]) -> [f64; 2]
{
    let scale: f64 = 1.0 / (1.0 - v[2]);
    return [v[0] * scale, v[1] * scale];
}

// stereograφic⁻¹, return normalized in S²(1)
fn r2_to_s2(p: &[f64; 2]) -> [f64; 3]
{
    let mut v = [2.0 * p[0], 2.0 * p[1], -1.0 + linalg::scalprod(&p, &p)];
    linalg::normalize(&mut v);
    return v;
}
