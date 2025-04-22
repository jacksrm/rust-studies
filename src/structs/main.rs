/**
 *
 * Structs
 *
 *
 * Structs are custom data types that let you name and package together multiple related values.
 *
 */
fn main() {
  // Tuple
  let rectangle = (30, 50);
  println!(
    "The area of the rectangle is: {}",
    rectangle.0 * rectangle.1
  );

  // Struct
  struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
  }

  struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
  }

  let mut user1 = User {
    active: true,
    username: String::from("user1"),
    email: String::from("example@user.com"),
    sign_in_count: 0,
  };

  user1.email = String::from("new.mail@exaple.com");

  println!("User email: {}", user1.email);
  println!("User sign in count: {}", user1.sign_in_count);
  println!("User active: {}", user1.active);
  println!("User username: {}", user1.username);

  fn build_user(username: String, email: String) -> User {
    User {
      username,
      email,
      active: true,
      sign_in_count: 0,
    }
  }

  let user2 = build_user(String::from("user2"), String::from("user2@example.com"));

  println!("User2 email: {}", user2.email);
  println!("User2 sign in count: {}", user2.sign_in_count);
  println!("User2 active: {}", user2.active);
  println!("User2 username: {}", user2.username);

  // Create instance from another instance
  let user3 = User {
    email: String::from("user3@example.com"),
    ..user1 // use the values from user1
  };

  println!("User3 email: {}", user3.email);
  println!("User3 sign in count: {}", user3.sign_in_count);
  println!("User3 active: {}", user3.active);
  println!("User3 username: {}", user3.username);

  // Tuple structs
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  let black = Color(0, 0, 0);
  let white = Color(255, 255, 255);
  let origin = Point(0, 0, 0);

  println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
  println!("White color: ({}, {}, {})", white.0, white.1, white.2);
  println!("Origin point: ({}, {}, {})", origin.0, origin.1, origin.2);

  // Unit-like structs
}
