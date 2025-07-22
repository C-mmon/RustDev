use std::rc::Rc;

fn main()
{
    let a =String::from("aniket");
    let b =a.clone(); //deep copy allocate a new memory
    println!("{}", b);
    
    let a1 = Rc::new(String::from("hello"));
    let b1 = Rc::clone(&a1);
    
    println!("{} {}",a1, b1);
}

//how to share heap data safely 
