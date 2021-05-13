enum FizzBuzzValue {
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
    FizzBuzzValue::FizzBuzz => {
      // <1>
      println!("FizzBuzz");
    }
    FizzBuzzValue::Fizz => {
      println!("Fizz");
    }
    FizzBuzzValue::Buzz => {
      println!("Buzz");
    }
    FizzBuzzValue::NotDivisible => {
      println!("{}", x);
    }
  }
}

fn fizzbuzz(x: i32) -> FizzBuzzValue {
  if x % 3 == 0 && x % 5 == 0 {
    FizzBuzzValue::FizzBuzz
  } else if x % 3 == 0 {
    FizzBuzzValue::Fizz
  } else if x % 5 == 0 {
    FizzBuzzValue::Buzz
  } else {
    FizzBuzzValue::NotDivisible
  }
}
