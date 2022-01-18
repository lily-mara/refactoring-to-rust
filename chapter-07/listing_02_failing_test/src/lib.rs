#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }

  #[test]
  fn it_does_not_work() {
    let result = 2 + 2;
    assert_eq!(result, 5);
  }
}
