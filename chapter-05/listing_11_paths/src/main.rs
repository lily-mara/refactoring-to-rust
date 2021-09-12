fn main() {
  let value = true;

  // All of the lines below this are paths
  value; // <1>

  hello; // <2>

  std::io::stdin; // <3>

  std::collections::hash_map::ValuesMut::<i32, String>::len; // <4>
}

fn hello() {}
