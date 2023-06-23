/* src/ast.rs */

use std::collections::VecDeque;

pub struct Vec3(pub [f64; 3]);
pub struct Mat3(pub [[f64; 3]; 3]);

pub enum Fig
{
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
