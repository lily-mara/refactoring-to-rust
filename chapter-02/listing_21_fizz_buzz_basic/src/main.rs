fn main() {
  for i in 1..101 {
    print_fizzbuzz(i);
  }
}

fn print_fizzbuzz(x: i32) { // <1>
  println!("{}", fizzbuzz(x));
}

fn fizzbuzz(x: i32) -> String {
  if x % 3 == 0 && x % 5 == 0 {
    String::from("FizzBuzz")
  } else if x % 3 == 0 {
    String::from("Fizz")
  } else if x % 5 == 0 {
    String::from("Buzz")
  } else {
    format!("{}", x) // <2>
  }
}