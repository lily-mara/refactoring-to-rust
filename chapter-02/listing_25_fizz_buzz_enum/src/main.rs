enum FizzBuzzValue {
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
    FizzBuzzValue::FizzBuzz => {
      println!("FizzBuzz");
    }
    FizzBuzzValue::Fizz => {
      println!("Fizz");
    }
    FizzBuzzValue::Buzz => {
      println!("Buzz");
    }
    FizzBuzzValue::NotDivisible(num) => {
      // <2>
      println!("{}", num);
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
    FizzBuzzValue::NotDivisible(x) // <3>
  }
}
