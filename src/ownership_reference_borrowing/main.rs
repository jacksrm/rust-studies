/*
* Ownership, Borrowing and References

* Ownership

* C, C++ -> Memory Management Control Issue
* Garbage Collector solved this issue, but created a new issue -> Slow Performance
* [stopping/resuming the program to collect garbage]

* Ownership Rules
* 1. Each value in Rust has a variable that’s called its “owner”.

  fn main() {
    let s1 = String::from("RUST"); // s1 is the owner of the string
    let len = calculate_length(&s1); // pass reference to the function
    println!("The length of '{}' is {}.", s1, len);
  }

  fn calculate_length(s: &String) -> usize {
    s.len()
  }

* 2. There can only be one owner at a time.

  fn main() {
    let s1 = String::from("RUST"); // s1 is the owner of the string
    let s2 = s1; // s2 takes ownership of the string, and s1 is no longer valid

    //println!("s1: {}", s1); // This will cause a compile-time error because s1 is no longer valid
    println!("s2: {}", s2); // This will work because s2 is the new owner of the string
  }

* 3. When the owner of a value goes out of scope, Rust will automatically drop the value.

  fn main() {
    let s1 = String::from("RUST"); // s1 is the owner of the string
    let len = calculate_length(&s1); // pass reference to the function
    println!("The length of '{}' is {}.", s1, len);
  }

  // fn print_lost(s: &String) {
  //   println!("{}.", &s1); // This will cause a compile-time error because s1 is no longer valid
  // }

  fn calculate_length(s: &String) -> usize {
    s.len()
  }


* References and Borrowing
* Safety and Performance
* Borrowing and References are powerful concepts

* Understanding References
* References are a way to borrow a value without taking ownership of it.
* Immutable Reference.
* Mutable Reference.
* Create reference by add "&"

* Immutable Reference

  fn main() {
    let x = 5;
    let r = &x; // r is a reference to x

    println!("Value of x: {}", x);
    println!("Value of r: {}", r);
    println!("Value of r + 1: {:?}", *r += 1); // This will cause a compile-time error because r is an immutable reference
  }

* Mutable Reference

  fn main() {
    let mut x = 5; // x is mutable
    let r = &mut x; // r is a mutable reference to x

    *r += 1; // increment the value of x through the reference
    *r -= 3; // decrement the value of x through the reference

    println!("Value of x: {}", x);
    // println!("Value of r: {}", r); // This will not work because you can only have one immutable reference or many mutable references at a time
  }

*/

fn main() {
  let mut _account = BankAccount {
    owner: String::from("John Doe"),
    balance: 150.55,
  };

  // Immutable borrow to  check balance
  _account.check_balance();

  // Mutable borrow to withdraw money
  _account.withdraw(45.50);

  // Immutable borrow to check balance again
  _account.check_balance();
}

struct BankAccount {
  owner: String,
  balance: f64,
}

impl BankAccount {
  fn withdraw(&mut self, amount: f64) {
    println!("Withdrawing {} from {}'s account", amount, self.owner);
    self.balance -= amount;
  }

  fn check_balance(&self) {
    println!("{}'s account balance: {}", self.owner, self.balance);
  }
}
