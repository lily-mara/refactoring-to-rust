enum FizzBuzzResult {
  Fizz,
  Buzz,
  FizzBuzz,
  NotDivisible,
}

fn main() {
  for i in 1..101 {
    print_fizzbuzz(i);
  }
}

fn print_fizzbuzz(x: i32) {
  match fizzbuzz(x) {
    FizzBuzzResult::FizzBuzz => { // <1>
      println!("FizzBuzz");
    }
    FizzBuzzResult::Fizz => {
      println!("Fizz");
    }
    FizzBuzzResult::Buzz => {
      println!("Buzz");
    }
    FizzBuzzResult::NotDivisible => {
      println!("{}", x);
    }
  }
}

fn fizzbuzz(x: i32) -> FizzBuzzResult {
  if x % 3 == 0 && x % 5 == 0 {
    FizzBuzzResult::FizzBuzz
  } else if x % 3 == 0 {
    FizzBuzzResult::Fizz
  } else if x % 5 == 0 {
    FizzBuzzResult::Buzz
  } else {
    FizzBuzzResult::NotDivisible
  }
}