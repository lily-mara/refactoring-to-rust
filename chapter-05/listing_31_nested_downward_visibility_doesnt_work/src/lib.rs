fn function() {
  nested::function();
}

mod nested {
  fn function() {
    very_nested::function();
  }

  mod very_nested {
    fn function() {
      very_very_nested::function();
    }

    mod very_very_nested {
      fn function() {}
    }
  }
}
