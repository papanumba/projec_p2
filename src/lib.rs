/* nov-2022 - jun 2023 */
// all suposed for ℝ³ vector space which gives P² projective plane
// infty line = {x[2] == 0}
// https://en.wikipedia.org/wiki/Stereographic_projection \
// #Visualization_of_lines_and_planes

#![allow(dead_code)]

#[macro_use]
extern crate lalrpop_util;

use pyo3::prelude::*;
use pyo3::exceptions::*;
use regex;

mod linalg;
mod proj;
mod ast;
lalrpop_mod!(pub repenser);

#[pyclass]
pub struct ProjWrap {
    canvas: proj::ProjCanvas,
}

// python-public methods
#[pymethods]
impl ProjWrap
{
    #[new]
    fn new(n: usize) -> PyResult<Self>
    {
        return match n {
            0 => Err(PyValueError::new_err("Error from rust: size is 0")),
            _ => Ok(Self { canvas : proj::ProjCanvas::new(n) }),
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
        // parse
        let newtaco = param_taco(pretaco);
        let tarser = repenser::TacoParser::new();
        let result = tarser.parse(&newtaco);
        let taco: ast::Taco;
        match result {
            Ok(t) => taco = t,
            Err(e) => return Err(PySyntaxError::new_err(e.to_string())),
        }
        // draw
        for f in &taco.0 {
            self.draw_fig(f);
        }
        return Ok(());
    }

    // returns 8-bit grayscale buffer
    pub fn get_pix_buff(&self) -> &[u8]
    {
        return self.canvas.as_bytes();
    }
}

// private rust-only methods
impl ProjWrap
{
    fn draw_fig(&mut self, f: &ast::Fig)
    {
        match f {
            ast::Fig::Pt(v) => self.canvas.draw_point(&v),
            ast::Fig::Ln(v, w) => self.canvas.draw_line_by_pts(&v, &w),
            ast::Fig::Eq(v) => self.canvas.draw_line_by_eq(&v),
            ast::Fig::Cn(m) => self.canvas.draw_conic(&m),
        }
    }
}

// param preprocess
// TODO: clean þis big mess
fn param_taco(t: &str) -> String
{
    let mut res = String::from(t);
    let param_re = regex::Regex::new(
        r"^param [A-Za-z]\w* = [+-]?\d+(\.\d+)?$"
    ).unwrap();
    let ident_re = regex::Regex::new(
        r"\b[A-Za-z]\w*\b"
    ).unwrap();
    let float_re = regex::Regex::new(
        r"[+-]?\b\d+(\.\d+)?\b"
    ).unwrap();
    let mut pos: usize = 0;

    for line in t.lines() {
        if line.len() < 2 {
            pos += line.len() + 1;
            continue;
        }
        match &line[0..2] {
            // found start of main scrīpt
            "pt" | "ln" | "eq" | "cn" => return res[pos..].to_string(),
            _ => {},
        }

        if param_re.is_match(&line) {
            let ident = ident_re.find(&line[6..]).unwrap().as_str();
            let float = float_re.find(&line)     .unwrap().as_str();
            let repla = format!("({})", float);
            let this_ident_re = regex::Regex::new(
                format!("\\b{}\\b", ident).as_str()).unwrap();
            res = this_ident_re.replace_all(&res, &repla).to_string();
            pos += (repla.len() as isize -
                    ident.len() as isize) as usize;
        }
        pos += line.len() + 1;
    }

    return res;
}

/* pyo3 boiler-plate */

#[pymodule]
fn projec_p2(_py: Python, m: &PyModule) -> PyResult<()>
{
    m.add_class::<ProjWrap>()?;
    return Ok(());
}
