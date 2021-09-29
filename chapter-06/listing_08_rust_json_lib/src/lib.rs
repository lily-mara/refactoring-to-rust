#[derive(Debug, serde::Deserialize)]
struct Data {
  name: String,
  value: i32,
}

fn sum(input: &str) -> i32 {
  let parsed: Data = serde_json::from_str(input).unwrap();

  parsed.name.len() as i32 + parsed.value
}
