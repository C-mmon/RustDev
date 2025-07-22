trait Shape
{
    fn calculate_area(&self)->f64;
    fn perimeter(&self)->f64;
}

struct Rectangle
{
    width: f64,
    height: f64,
}

impl Shape for Rectangle
{
    fn calculate_area(&self)-> f64
    {
        self.width*self.height
    }
    
    fn perimeter(&self)->f64
    {
        2.0*(self.width+self.height)
    }
}

fn main()
{
    let rect = Rectangle {width: 10.0, height: 5.0 };
    println!("Rectangle area: {}", rect.calculate_area());
    println!("Rectangle perimeter: {}", rect.perimeter());
}
