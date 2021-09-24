struct Data {
  name: String,
  value: i32,
}

fn main() {
  let input = "{ \"name\": \"zl594EB\", \"value\": 63836 }";

  let parsed = serde_json::from_str(input).unwrap();

  println!("{:?}", parsed);
}
