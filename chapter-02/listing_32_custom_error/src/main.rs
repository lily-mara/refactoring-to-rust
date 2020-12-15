enum FizzBuzzResult {
  Fizz,
  Buzz,
  FizzBuzz,
  NotDivisible(i32),
}

enum Error {
  GotNegative,
}

fn main() {
  for i in 1..101 {
    match print_fizzbuzz(i) {
      Ok(()) => {}
      Err(e) => {
        match e {
          Error::GotNegative => {
            eprintln!("Error: Fizz Buzz only supports positive numbers!");
            return;
          }
        }
      }
    }
  }
}

fn print_fizzbuzz(x: i32) -> Result<(), Error> {
  match fizzbuzz(x) {
    Ok(result) => {
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

fn fizzbuzz(x: i32) -> Result<FizzBuzzResult, Error> {
  if x < 0 {
    Err(Error::GotNegative)
  } else if x % 3 == 0 && x % 5 == 0 {
    Ok(FizzBuzzResult::FizzBuzz)
  } else if x % 3 == 0 {
    Ok(FizzBuzzResult::Fizz)
  } else if x % 5 == 0 {
    Ok(FizzBuzzResult::Buzz)
  } else {
    Ok(FizzBuzzResult::NotDivisible(x))
  }
}