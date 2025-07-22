fn use_string(s: string)
{
    println!("{}", s);
}

fn main()
{
    let my_str =String::from("hello");
    use_string(&s);
    println("{}", my_str); //this will fail because the value is borrowed after move
}
fn longest(a: String, b: String) -> String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}


//Case1 
fn main() {
    let str1 = String::from("small");
    let str2 = String::from("longer");

    // Clone the Strings — make copies
    let longest_str = longest(str1.clone(), str2.clone());

    println!("{}", longest_str);  
    println!("Original 1: {}", str1);
    println!("Original 2: {}", str2); 
}

//Case 2, this would print happily because we are just creating a new string 
fn main()
  {let str1 = String::from("small");
   {
     let str2 = String::from("longer");
     ans = longest(a: str1, b: str2); //after executing you return a new string, after that we dont try to use str1 and str2 after that
   }
   println!("{}", ans);
  }
    



