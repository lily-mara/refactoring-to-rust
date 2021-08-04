fn function() {}

mod nested {
  fn function() {
    crate::function();
  }

  mod very_nested {
    fn function() {
      crate::function();
      crate::nested::function();
    }

    mod very_very_nested {
      fn function() {
        crate::function();
        crate::nested::function();
        crate::nested::very_nested::function();
      }
    }
  }
}
