pub mod treats {
  pub mod shop {
    fn buy() -> super::Treat {
      super::Treat::IceCream
    }
  }

  pub enum Treat {
    Candy,
    IceCream,
  }

  pub struct ConsumedTreat {
    treat: Treat,
  }

  fn eat(treat: Treat) -> ConsumedTreat {
    ConsumedTreat { treat }
  }
}

fn regret(treat: treats::ConsumedTreat) {
  println!("That was a mistake");
}
