pub mod treats {
  pub mod shop {}

  pub enum Treat {
    Candy,
    IceCream,
  }
}

pub fn eat(treat: treats::Treat) {}
