use std::{env::args, ffi::c_float};

use argh::FromArgs;


#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
struct TopLevel {
    #[argh(subcommand)]
    nested: Shapes,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Shapes {
    triangle(Triangle),
    circle(Circle),
    rectangle(Rectangle),
}

#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "triangle")]
struct Triangle {
    #[argh(option,short = 'b')]
    /// base value
    base: c_float,

    #[argh(option,short = 'h')]
    /// height value
    height: c_float,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Second subcommand.
#[argh(subcommand, name = "circle")]
struct Circle {
    #[argh(option,short = 'r')]
    /// radius
    radius: c_float,
}


#[derive(FromArgs, PartialEq, Debug)]
/// Second subcommand.
#[argh(subcommand, name = "rectangle")]
struct Rectangle {
    #[argh(option,short = 'l')]
    /// length
    length: usize,

    #[argh(option,short = 'w')]
    /// width
    width: usize,
    
}


fn main() {
    let args: TopLevel = argh::from_env(); 

    match args.nested {
         Shapes::circle(circle) => println!("Area of circle is {}",2.0 * circle.radius * 3.14),
         Shapes::rectangle(rectangle) => println!("Area of rectangle is {}",2 * rectangle.length * rectangle.width),
         Shapes::triangle(triangle)=> println!("Area of triangle is {}",0.5 * triangle.base * triangle.height),
    }
}
