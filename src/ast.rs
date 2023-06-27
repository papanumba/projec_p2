/* src/ast.rs */

use std::collections::VecDeque;

pub type Vec3 = [f64; 3];
pub type Mat3 = [[f64; 3]; 3];

pub enum Fig
{
    Pt(Vec3),
    Ln(Vec3, Vec3),
    Eq(Vec3),
    Cn(Mat3),
}

pub struct Taco(pub VecDeque<Fig>);

impl Taco
{
    pub fn push_back(&mut self, f: Fig)
    {
        self.0.push_back(f);
    }
}
