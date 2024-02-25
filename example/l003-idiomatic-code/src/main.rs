use std::cmp::PartialEq;

#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
#[derive(PartialEq)] 
enum Shape {
    Circle(f64),
    Rectangle { width: i32, height: i32 },
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let shape = Shape::Circle(10.0);
    if shape == Shape::Circle(p.x as f64) {
        println!("Circle!");
    } else {
        println!("Not a circle");
    }
    // Clippy warnings:
    // - Needless return on last line of function
    // - Too many arguments in function
    // - Slow code: iterating over iterator and collecting into vector
}


