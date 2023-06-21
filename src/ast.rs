/* src/ast.rs */

use std::collections::VecDeque;

pub struct Vec3(pub f64, pub f64, pub f64);

pub enum Fig
{
    Eq(Vec3),
    // Cn(Mat3),
}


pub struct Script
{
    pub size: u32,
    pub figs: VecDeque<Fig>,
}

impl Script
{
    pub fn from(s: u32, l: VecDeque<Fig>) -> Self
    {
        return Self {size: s, figs: l};
    }

    pub fn print(&self)
    {
        println!("size {}", self.size);
        for fig in &self.figs {
            match fig {
                Fig::Eq(v) => println!("eq {} {} {}\n", v.0, v.1, v.2),
                //_ => todo!(),
            };
        }
    }
}
