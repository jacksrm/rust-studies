/**
*
* Variables and Mutability
* all variables are immutable by default

 fn main() {
   let a: u16 = 5;
   println!("a: {}", a);

   a = 10; // This will cause a compile-time error because a is immutable
 }

 * to make a variable mutable, use the mut keyword

  fn main() {
    let mut a: u16 = 5;
    println!("a: {}", a);
    a = 10; // This will cause a compile-time error because a is immutable
    println!("a: {}", a);
  }
*/

fn main() {
  let mut a: u16 = 5;
  println!("a: {}", a);
  a = 10; // This will cause a compile-time error because a is immutable
  println!("a: {}", a);
}
