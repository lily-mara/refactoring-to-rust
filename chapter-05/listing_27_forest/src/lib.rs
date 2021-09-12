pub mod forest {
  pub fn enter_area(area: &str) {
    match area {
      "tree cover" => println!("It's getting darker..."),
      "witches coven" => println!("It's getting spookier..."),
      "walking path" => println!("It's getting easier to walk..."),
      x => panic!("Unexpected area: {}", x),
    }
  }
}

pub mod tree_cover {
  pub fn enter() {
    crate::forest::enter_area("tree cover");
  }
}

pub mod walking_path {
  pub fn enter() {
    crate::forest::enter_area("walking path");
  }
}

pub mod witches_coven {
  pub fn enter() {
    crate::forest::enter_area("witches coven");
  }
}
