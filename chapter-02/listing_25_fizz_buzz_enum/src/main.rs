enum FizzBuzzResult {
  Fizz,
  Buzz,
  FizzBuzz,
  NotDivisible(i32), // <1>
}

fn main() {
  for i in 1..101 {
    print_fizzbuzz(i);
  }
}

fn print_fizzbuzz(x: i32) {
  match fizzbuzz(x) {
    FizzBuzzResult::FizzBuzz => {
      println!("FizzBuzz");
    }
    FizzBuzzResult::Fizz => {
      println!("Fizz");
    }
    FizzBuzzResult::Buzz => {
      println!("Buzz");
    }
    FizzBuzzResult::NotDivisible(num) => { // <2>
      println!("{}", num);
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
    FizzBuzzResult::NotDivisible(x) // <3>
  }
}