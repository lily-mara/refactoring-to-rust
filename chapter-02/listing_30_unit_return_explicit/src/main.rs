fn main() {
  foo();
  bar();
  baz();
}

fn foo() { // <1>
  println!("Hello!");
}

fn bar() -> () { // <2>
  println!("Hello!");
}

fn baz() -> () {
  println!("Hello!");
  () // <3>
}
