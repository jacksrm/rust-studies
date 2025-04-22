/**
* Constants
* Constants are values bound to a name that are not allowed to change. They are always immutable.

 fn main() {
   let mut x: i32 = 5;
   const mut y = 10 // this will cause a compile-time error, it needs a type and cannot be mutable
   println!("x: {}", x);
   println!("y: {}", y);
 }

* Constants needs to be capitalized and have a type

  fn main() {
    let x: i32 = 5;
    const Y: i32 = 10;
    println!("x: {}", x);
    println!("Y: {}", Y);
  }


*/

const PI: f64 = 3.14159265358979323846;
fn main() {
  println!("The value of PI is: {}", PI);
}
