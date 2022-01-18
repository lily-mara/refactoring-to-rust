#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    eprintln!("it_works stderr");
    println!("it_works stdout");
    let result = 2 + 2;
    assert_eq!(result, 4);
  }

  #[test]
  fn it_does_not_work() {
    eprintln!("it_does_not_work stderr");
    println!("it_does_not_work stdout");
    let result = 2 + 2;
    assert_eq!(result, 5);
  }
}
