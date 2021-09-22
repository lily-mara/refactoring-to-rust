pub fn sum(line: &str) -> i64 {
  let data = parse_as_json(line);

  data.value + data.name.len()
}
