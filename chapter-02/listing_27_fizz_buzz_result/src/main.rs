enum FizzBuzzResult {
  Fizz,
  Buzz,
  FizzBuzz,
  NotDivisible(i32),
}

fn main() {
  for i in 1..101 {
    match print_fizzbuzz(i) {
      Ok(()) => {}
      Err(e) => {
        eprintln!("Error: {}", e); // <1>
        return;
      }
    }
  }
}

fn print_fizzbuzz(x: i32) -> Result<(), &'static str> { // <2>
  match fizzbuzz(x) {
    Ok(result) => { // <3>
      match result {
        FizzBuzzResult::FizzBuzz => {
          println!("FizzBuzz");
        }
        FizzBuzzResult::Fizz => {
          println!("Fizz");
        }
        FizzBuzzResult::Buzz => {
          println!("Buzz");
        }
        FizzBuzzResult::NotDivisible(num) => {
          println!("{}", num);
        }
      }

      Ok(())
    }
    Err(e) => {
      Err(e)
    }
  }
}

fn fizzbuzz(x: i32) -> Result<FizzBuzzResult, &'static str> {
  if x < 0 {
    Err("Provided number must be positive!")
  } else if x % 3 == 0 && x % 5 == 0 {
    Ok(FizzBuzzResult::FizzBuzz) // <4>
  } else if x % 3 == 0 {
    Ok(FizzBuzzResult::Fizz)
  } else if x % 5 == 0 {
    Ok(FizzBuzzResult::Buzz)
  } else {
    Ok(FizzBuzzResult::NotDivisible(x))
  }
}