struct Point
{
    x: f64,
    y: f64,
}

fn main()
{
    let origin = Point {x: 0.0, y: 0.0};
    println!("The origin is at ({} {}) ", origin.x, origin.y);
}
