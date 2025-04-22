fn main() {
  /*

   * Primitive data types
   * int, float, char, bool

   * Integer
   * Rust has signed and unsigned integers
   * i8, i16, i32, i64, i128: signed integers
   * u8, u16, u32, u64, u128: unsigned integers
   * signed integers can be negative, unsigned integers cannot

  let x: i32 = -120; // signed integer
  let y: u32 = 10; // unsigned integer
  println!("Signed integer: {}, Unsigned integer: {}", x, y);

   * Float
   * f32, f64: floating point numbers
  let a: f32 = 3.14; // 32-bit floating point number
  let b: f64 = 3.141592653589793; // 64-bit floating point number
  println!("Float: {}, {}", a, b);

   * Char
   * char: a single character
  let c: char = 'A'; // single character
  let d: char = '1'; // single character
  let e: char = ' '; // single character
  let f: char = '😊'; // single characte
  println!("Char: {}, {}, {}, {}", c, d, e, f);

   * Bool
   * bool: true or false
  let g: bool = true; // boolean
  let h: bool = false; // boolean
  println!("Bool: {}, {}", g, h);

  */

  /*
   * Compound data types
   * array: a fixed-size collection of values of the same type
   * tuple: a fixed-size collection of values of different types
   * slices: a dynamically-sized view into a contiguous sequence of elements
   * string (slice string): a dynamically-sized string
   */

  /*
   * Array
  let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // array of integers
  println!("Number array: {:?}", numbers);

  // let mix = [1, 2.0, 'A', true]; // error
  // println!("Mixed array: {:?}", mix);

  let fruits: [&str; 3] = ["apple", "banana", "cherry"]; // array of strings
  println!("Fruits array: {:?}", fruits);
  println!("First fruit: {}", fruits[0]); // access first element
  println!("Second fruit: {}", fruits[1]); // access second element
  println!("Third fruit: {}", fruits[2]); // access third element

   * Tuple
  let person: (String, i32, bool) = ("Alice".to_string(), 30, false);
  println!("Person tuple: {:?}", person);

   * Slice
  let number_slices = &[1, 2, 3, 4, 5]; // slice of integers
  println!("Number slice: {:?}", number_slices);

   * Strings vs String slices (&str)
   * A- Strings [growable, mutable, owned string type]
  let mut stone_cold: String = String::from("Hell, ");
  stone_cold.push_str("yeah!"); // push string
  println!("Stone cold says: {}", stone_cold);

   * B- String slices (&str) [immutable, borrowed string type]
  let string: String = String::from("Hello, world!");
  let slice: &str = &string; // slice of string
  println!("Slice value: {}", slice);

   */

  /*
  * Functions
  * any functions/variables must be written in snake_case

  * snake case: hello_world
  * kebab case: hello-world
  * camel case: helloWorld
  * pascal case: HelloWorld

  * functions are hoisted
  *


  hello_word();
  tell_height(180);
  human_id("Jack", 30, 1.78);

  * Expressions vs Statements
  * Expressions: return a value
  * Statements: do not return a value

  * Example of expression
  * 5
  * 5 + 5
  * true & false
  * if condition {value1} else {value2}
  * blocks of code {  }

  let _x: i32 = {
    let price = 5;
    let quantity = 10;
    price * quantity
  };

  println!("Result is {}", _x);

  let y = add(5, 10);
  println!("Value of Y is: {}", y);
  println!("Value from function 'add' is: {}", add(25, 10));

  println!("Value from function 'one_or_zero' is: {}", one_or_zero(-5));
  println!("Value from function 'one_or_zero' is: {}", one_or_zero(5));
  println!("Value from function 'one_or_zero' is: {}", one_or_zero(0));

  let weight = 70.0;
  let height = 1.82;

  let bmi = calculate_bmi(height, weight);
  println!("Your BMI is: {:.1}", bmi);
  */
}

// function structure
fn hello_word() {
  println!("Hello, world!");
}

// function with parameter
fn tell_height(height: u32) {
  println!("My height is: {}", height);
}

// function with multiple parameters
fn human_id(name: &str, age: u32, height: f32) {
  println!(
    "My name is: {}, I am {} years old, and my height is: {}",
    name, age, height
  );
}

// function with return value
fn add(a: i32, b: i32) -> i32 {
  a + b
}

fn one_or_zero(random_number: i32) -> i32 {
  if random_number.is_positive() { 1 } else { 0 }
}

fn calculate_bmi(height: f64, weight: f64) -> f64 {
  weight / (height * height)
}
