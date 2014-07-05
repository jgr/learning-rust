use std::string::String;

fn main() {
  for i in range(0u, 100) {
    let mut buf = String::new();

    if i % 3 == 0 {
      buf = buf.append("Fizz");
    }
    if i % 5 == 0 {
      buf = buf.append("Buzz");
    }
    if buf.is_empty() {
      println!("{}", i);
    } else {
      println!("{}", buf);
    }
  }
}
