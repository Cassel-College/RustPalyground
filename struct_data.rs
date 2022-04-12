struct Point {
    x : i32,
    y : i32,
}

fn main() {
    let p = Point { x: 0, y: 9};
    println!("Point is at ({}, {})", p.x, p.y);
}