/// Add together two i32 numbers and return the result of that addition
/// ```
/// assert_eq!(chapter_07_listing_07::add(2, 2), 4);
/// ```
///
/// ```
/// use chapter_07_listing_07::add;
/// assert_eq!(add(3, 2), 5);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
  x + y
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
