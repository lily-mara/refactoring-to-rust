pub mod treats {
  pub mod shop {
    fn buy() -> crate::treats::Treat {
      crate::treats::Treat::IceCream
    }
  }

  pub enum Treat {
    Candy,
    IceCream,
  }

  pub struct ConsumedTreat {
    treat: Treat,
  }

  fn eat(treat: crate::treats::Treat) -> crate::treats::ConsumedTreat {
    crate::treats::ConsumedTreat { treat }
  }
}

fn regret(treat: crate::treats::ConsumedTreat) {
  println!("That was a mistake");
}
