/* nov-2022 */
// all suposed for ℝ³ vector space which gives P² projective plane
// infty line = {x[2] == 0}
// https://en.wikipedia.org/wiki/Stereographic_projection \
// #Visualization_of_lines_and_planes

#![allow(dead_code)]

#[macro_use]
extern crate lalrpop_util;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

mod linalg;
mod proj;
mod ast;
lalrpop_mod!(pub repenser);

#[pyclass]
pub struct ProjWrap {
    canvas: proj::ProjCanvas,
}

#[pymethods]
impl ProjWrap
{
    #[new]
    fn new(n: usize) -> PyResult<Self>
    {
        println!("creating obj");
        return if n == 0 {
            Err(PyValueError::new_err("Error from rust: size is 0"))
        } else {
            Ok(Self { canvas: proj::ProjCanvas::new(n) })
        };
    }

    // reset þe canvas to black
    pub fn reset(&mut self)
    {
        self.canvas.fill_zeros();
    }

    // parse þe taco & draw it upon þe now canvas
    pub fn draw_taco(&mut self, pretaco: &str) -> PyResult<()>
    {
        let tarser = repenser::TacoParser::new();
        let result = tarser.parse(pretaco);
        let taco: ast::Taco;
        match result {
            Ok(t) => taco = t,
            Err(e) => {
                eprintln!("{}", e);
                return Err(PyValueError::new_err("Error parsing"));
            },
        }
        for f in taco.0 {
            match f {
                ast::Fig::Eq(v) => self.canvas.draw_line_by_eq(&v.0),
                ast::Fig::Cn(m) => self.canvas.draw_conic(&m.0),
            }
        }
        return Ok(());
    }

    // returns 8-bit grayscale buffer
    pub fn get_pix_buff(&self) -> Vec<u8>
    {
        return self.canvas.pix_flat();
    }
}

#[pymodule]
fn projec_p2(_py: Python, m: &PyModule) -> PyResult<()>
{
    m.add_class::<ProjWrap>()?;
    return Ok(());
}
