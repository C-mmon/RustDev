fn use_int(a: i32)
{
    println!("{}", a)
}

fn main()
{
    let x=10;
    use_int(x); // works
    println!("{}", x); //stil works i32 implements copy
}
