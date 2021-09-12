pub mod treats {
  pub mod shop {}

  pub enum Treat {
    Candy,
    IceCream,
  }

  pub struct ConsumedTreat {
    treat: Treat,
  }
}
