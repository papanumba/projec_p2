/* nov-2022 */
// all suposed for ℝ³ vector space which gives P² projective plane
// infty line = {x[2] == 0}
// https://en.wikipedia.org/wiki/Stereographic_projection \
// #Visualization_of_lines_and_planes

#![allow(dead_code)]

#[macro_use]
extern crate lalrpop_util;

use std::io::Write;
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

fn main()
{
    let w: usize;
    println!("size of image?");
    print!("> ");
    std::io::stdout().flush().unwrap(); // flush because of þe read_line
    match read_usize() {
        Ok(num) => w = num,
        Err(e) => panic!("{}", e),
    }

    if w < 2 {
        panic!("ERROR: size too small!");
    } else if w > 3000 {
        panic!("ERROR: size too big!");
    }

    let mut canvas = proj::ProjCanvas::new(w);

    println!("OK");

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let state: u8 = read_stmt();
        /*   0: write & exit;
         **  1: draw_point;
         **  2: draw_pts;
         **  3: draw_eq;
         **  4: draw_conic;
         **  5: print help;
         **  6: exit;
         **  7: blank;
         */

        match state {
            0 => {
                let mut outfname: String = String::new();
                match std::io::stdin().read_line(&mut outfname) {
                    Ok(_) => canvas.save_to_image(outfname.trim()),
                    Err(e) => eprintln!("{}", e),
                };
                break;
            }
            5 => print_help(),
            6 => break,
            7 => {}, // blank stmt
            _ => draw_stmt(state, &mut canvas),
        }
    }

    println!("END");
}


fn read_stmt() -> u8
{
    let mut line: String = String::new();
    let b: usize = std::io::stdin().read_line(&mut line).unwrap();
    if b == 0 {
        panic!("error: couldnt read line");
    }

    return match line.trim() {
        "draw" => 0,
        "help" => 5,
        "exit" => 6,
        "pt" => 1,
        "ln" => 2,
        "eq" => 3,
        "cn" => 4,
        "" => 7,
        _ => 0xff,
    };
}

fn draw_stmt(stmt: u8, canvas: &mut proj::ProjCanvas)
{
    match stmt {
        0 => {} // done in main
        1 => match read_vec::<3>() {
            Ok(vec) => canvas.draw_point(&vec),
            Err(e) => eprintln!("{}", e),
        },
        2 => match read_vec::<3>() {
            Ok(v1) => match read_vec::<3>() {
                Ok(v2) => canvas.draw_line_by_pts(&v1, &v2),
                Err(e) => eprintln!("{}", e),
            },
            Err(e) => eprintln!("{}", e),
        },
        3 => match read_vec::<3>() {
            Ok(vec) => canvas.draw_line_by_eq(&vec),
            Err(e) => eprintln!("{}", e),
        },
        4 => match read_mat() {
            Ok(mat) => canvas.draw_conic(&mat),
            Err(e) => eprintln!("{}", e),
        },
        _ => eprintln!("ERROR: unknown command"),
    }
}

fn read_usize() -> Result<usize, std::num::ParseIntError>
{
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().parse::<usize>();
}

// read a vector of N real components separated by whitespace
// also return Err if þe vector is close to 0, so as to be used for proj. geom.
fn read_vec<const N: usize>() -> Result<[f64; N], &'static str>
{
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut comps = line.split_whitespace(); // iterator of slices of str

    let mut v: [f64; N] = [0.0; N];

    for i in 0..N {
        match comps.next() {
            Some(s) => match s.parse::<f64>() {
                Ok(f) => v[i] = f,
                Err(_) => return Err("ERROR: could not parse to float"),
            },
            None => return Err("ERROR: line has less than 3 components"),
        }
    }

    if linalg::is_zero(&v) {
        return Err("ERROR: vector is too close to 0");
    }

    return Ok(v);
}

fn read_mat() -> Result<[[f64; 3]; 3], &'static str>
{
    let mut mat = [[0.0f64; 3]; 3];
    for row in 0..3 {
        match read_vec::<3>() {
            Ok(vec) => mat[row] = vec,
            Err(e) => return Err(e),
        }
    }
    return Ok(mat);
}

fn print_help()
{
    println!("drawing comands: pt, ln, eq, cn");
    println!("pt: draw a single point");
    println!("ln: draw a line, given 2 points");
    println!("eq: draw a line, given by the coefs of its equation");
    println!("cn: draw a conic, given a matrix");
    println!("");
    println!("a point is given by 3 real numbers separated by space ' '");
    println!("a matrix is given by entering 3 points for each row");
}

/*// expected 3 numbers separated by space
fn parse_vec(s: &str) -> [f64; 3]
{
    let vec3_re  = regex::Regex::new(r"^\d+ +\d+ +\d+ *$").unwrap();
    let float_re = regex::Regex::new(r"\d+").unwrap();
    if !vec3_re.is_match(s) {
        eprintln!("ERROR: {:?}, not a vector", vec3_re);
        return [0.0, 0.0, 1.0];
    }
    let mut vec3 = [1.0; 3];
    for (i, f) in float_re.captures_iter(s).enumerate() {
        match f.get(0) {
            Some(m) => vec3[i] = m.as_str().parse::<f64>().unwrap(),
            _ => panic!(),
        }
    }
    return vec3;
}
*/
