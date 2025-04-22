/**
 * Shadowing
 * Basically, you can declare a variable with the same name as a previous variable, and it will shadow the previous variable.
 * This is different from mutability, where you can change the value of a variable without changing its name.
 *
 * Shadowing is useful when you want to reuse a variable name for a different purpose, or when you want to change the type of a variable.
 */

fn main() {
  let x = 5;

  let x = x + 1; // This will shadow the previous value of x

  {
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
  }

  println!("The value of x in the outer scope is: {x}");
}
