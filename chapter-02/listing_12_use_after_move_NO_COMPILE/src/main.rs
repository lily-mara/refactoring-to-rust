struct Artwork {
  name: String,
}

fn admire_art(art: Artwork) {
  println!("Wow, {} really makes you think.", art.name);
}

fn main() {
  let art1 = Artwork {
    name: "Man on Fire".to_string(),
  };

  let borrowed_art = &art1;

  admire_art(art1);

  println!("I really enjoy {}", borrowed_art.name);
}
