struct Artwork {
  name: String,
}

fn admire_art(art: Artwork) {
  println!("Wow, {} really makes you think.", art.name);
}

fn main() {
  let art1 = Artwork {
    name: "Las dos Fridas".to_string(),
  };
  let art2 = Artwork {
    name: "The Persistence of Memory".to_string(),
  };

  admire_art(art1);
  admire_art(art2);
}
