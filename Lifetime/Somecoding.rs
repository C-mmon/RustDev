enum Shape
{
    Circle(f64), 
    Rectangle(f64, f64),
    Triangle
}

fn main()
{
    let my_circle= Shape::Circle(5.0);
    let my_rectangle = Shape::Rectangle(1.0, 1.0);
    let my_triangle = Shape::Triangle;
    
    //now we shall use match pattern 
    //in rust match is exhaustiveho, 
    match my_circle
    {
        Shape::Circle(r) => println!("this is readius of the circle"),
        Shape::Rectangle(w, h) => println!("It's a rectangle with width {} and height {}", w, h),
        Shape::Triangle => println!("It's a triangle"),
    }
    
    //if you want to do only do something for some variant and ignore other, use_
    //match my_circle
    // match my_shape
    //{Shape::Circle(r) => println!("Circle: {}", r),
    // _ => println!("Not a circle!"),
    // }
    
    //it must check if the my_circle is a variant of Shape::Circle(r), if true destructure the value and 
    // pulls  out the radius into the varaible r
    //match is exhaustive, but let is focused on single pattern
    if let Shape::Circle(r) = my_circle {
    println!("It's a circle with radius {}", r);
}

}

//Now obviously you will not end up writing the whole thing again and agian 
impl Shape {
    fn describe(&self) {
        match self {
            Shape::Circle(r) => println!("It's a circle with radius {}", r),
            Shape::Rectangle(w, h) => println!("It's a rectangle with width {} and height {}", w, h),
            Shape::Triangle => println!("It's a triangle"),
        }
    }
} 
