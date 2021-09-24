#[derive(Debug, serde::Deserialize)]
struct Data {
  name: String,
  value: i32,
}

fn main() {
  let input = "{ \"name\": \"zl594EB\", \"value\": 63836 }";

  let parsed: Data = serde_json::from_str(input).unwrap();

  println!("{:?}", parsed);
}
