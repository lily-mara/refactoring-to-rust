enum FizzBuzzValue {
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
      Err(e) => match e {
        Error::GotNegative => {
          eprintln!("Error: Fizz Buzz only supports positive numbers!");
          return;
        }
      },
    }
  }
}

fn print_fizzbuzz(x: i32) -> Result<(), Error> {
  match fizzbuzz(x) {
    Ok(result) => {
      match result {
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
          println!("{}", num);
        }
      }

      Ok(())
    }
    Err(e) => Err(e),
  }
}

fn fizzbuzz(x: i32) -> Result<FizzBuzzValue, Error> {
  if x < 0 {
    Err(Error::GotNegative)
  } else if x % 3 == 0 && x % 5 == 0 {
    Ok(FizzBuzzValue::FizzBuzz)
  } else if x % 3 == 0 {
    Ok(FizzBuzzValue::Fizz)
  } else if x % 5 == 0 {
    Ok(FizzBuzzValue::Buzz)
  } else {
    Ok(FizzBuzzValue::NotDivisible(x))
  }
}
