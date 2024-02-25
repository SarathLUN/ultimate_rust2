# Lesson 3: Idiomatic Code

- Idiomatic Rust: using Rust like an experienced developer, writing code that adheres to common conventions and best practices.
- rustfmt: automatic code formatter, ensuring consistent style and readability.
- clippy: code analysis tool, checking for stylistic issues, correctness errors, complexity, and performance problems.
- learn by automation: leverage tools like rustfmt & clippy to improve code quality without memorizing numberous rules.
- customizable tools: configure tools to fit your preferences while maintaining idiomatic style.
- learn from warnings: understand warnings issues by clippy to write better and more efficient code.

### Example code:

#### Before:

```rust
struct Point { x: i32, y: i32 }
enum Shape { Circle(f64), Rectangle { width: i32, height: i32 }
fn main() {
let p = Point {x: 1, y: 2};
let shape = Shape::Circle(10.0);
if shape == Shape::Circle(p.x as f64) {
println!("Circle!");
}else{
println!("Not a circle");
 }

 // Clippy warnings:
// - Needless return on last line of function
  // - Too many arguments in function
// - Slow code: iterating over iterator and collecting into vector
}


```

#### After: 

```rust
struct Point {
    x: i32,
    y: i33,
}

enum Shape {
    Circle(f64),
    Rectangle {
        width: i32,
        height: i32,
    },
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let shape = Shape::Circle(10.0);

    if shape == Shape::Circle(p.x as f64) {
        println!("Circle!");
    } else {
        println!("Not a circle");
    }

    // More performant and idiomatic way to check if circle:
    if p.x as f64 == shape.circle() {
        println!("Circle!");
    } else {
        println!("Not a circle");
    }
}

```
