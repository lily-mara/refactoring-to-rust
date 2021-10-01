#[derive(Debug, serde::Deserialize)]
struct Data {
  name: i32,
  value: i32,
}

fn main() {
  let input = "{ \"name\": \"Sharpe Oliver\", \"value\": 134087 }";

  let parsed: Data = serde_json::from_str(input).unwrap();

  println!("{:?}", parsed);
}
