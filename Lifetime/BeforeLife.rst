fn longest(a: String,b: String) -> String
{  if a.len() > b.len() 
   { return a; }
   else 
   { return b; }
}
 
fn main()
{
  let longest_str;
  let str1= String::from("small");
  let str2= String::from("longer");
  longest_str= longest(str1, str2);
  println!("{}",longest_str);
  }
