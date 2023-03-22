/* nov-2022 */
// all suposed for ℝ³ vector space which gives P² projective plane
// infty line = {x[2] == 0}
// https://en.wikipedia.org/wiki/Stereographic_projection \
// #Visualization_of_lines_and_planes

use image;
use std::io::Write;

mod linalg;
mod proj;


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
    let mut outfname: String = String::new();

    println!("OK");

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let state: u8 = read_stmt();
        /*  0: write & exit;
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
                match std::io::stdin().read_line(&mut outfname) {
                    Ok(_) => save_img(outfname.trim(), &canvas),
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
    let b = std::io::stdin().read_line(&mut line).unwrap();
    if b == 0 {
        panic!("error: couldnt read line");
    }

    line = line.trim().to_string();
    if line == String::from("draw") {
        return 0;
    } else if line == String::from("pt") {
        return 1;
    } else if line == String::from("ln") {
        return 2;
    } else if line == String::from("eq") {
        return 3;
    } else if line == String::from("cn") {
        return 4;
    } else if line == String::from("help") {
        return 5;
    } else if line == String::from("exit") {
        return 6;
    } else if line == String::from("") {
        return 7;
    } else {
        return 0xff;
    }
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

fn save_img(outfname: &str, canvas: &proj::ProjCanvas)
{
    let w: u32 = canvas.pix.len() as u32;

    // write out image
    let mut canvas_img: image::GrayImage = image::ImageBuffer::new(w, w);
    write_img(&mut canvas_img, &canvas.pix);
    canvas_img
        .save_with_format(outfname, image::ImageFormat::Png)
        .unwrap();
}

fn write_img(img: &mut image::GrayImage, pix: &Vec<Vec<u8>>)
{
    let w = pix.len();
    for i in 0..w {
        for j in 0..w {
            img.put_pixel(i as u32, j as u32, image::Luma([pix[i][j]]));
        }
    }
}