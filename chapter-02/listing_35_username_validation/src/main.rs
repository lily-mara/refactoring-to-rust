enum UsernameError {
  NotLowercase,
  NotUnique,
}

fn main() {
  match validate_username("user1") {
    Ok(()) => println!("Valid username"),
    Err(UsernameError::NotLowercase) => println!("Username must be lowercase"),
    Err(UsernameError::NotUnique) => println!("Username already exists"),
  }
}

fn validate_username(username: &str) -> Result<(), UsernameError> {
  validate_lowercase(username).map_err(|_| UsernameError::NotLowercase)?;
  validate_unique(username).map_err(|_| UsernameError::NotUnique)?;

  Ok(())
}

fn validate_lowercase(username: &str) -> Result<(), ()> {
  Ok(())
}

fn validate_unique(username: &str) -> Result<(), ()> {
  Ok(())
}
