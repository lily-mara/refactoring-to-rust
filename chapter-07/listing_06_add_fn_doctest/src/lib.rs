/// Add together two i32 numbers and return the result of that addition
/// ```
/// assert_eq!(chapter_07_listing_06::add(2, 2), 4);
/// ```
///
/// ```
/// use chapter_07_listing_06::add;
/// assert_eq!(add(2, 2), 5);
/// ```
///
/// ```
/// use chapter_07_listing_06::add;
/// assert_eq!(add("hello", 2), 4);
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
