pub mod treats {
  pub mod shop {
    use crate::treats::Treat;

    fn buy() -> Treat {
      Treat::IceCream
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

use treats::ConsumedTreat;

fn regret(treat: ConsumedTreat) {
  println!("That was a mistake");
}
