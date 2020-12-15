struct Artwork {
  name: String,
}

fn admire_art(art: Artwork) {
  println!("Wow, {} really makes you think.", art.name);
}

fn main() {
  let art1 = Artwork {
    name: "The Ordeal of Owain".to_string(),
  };

  admire_art(art1);
  admire_art(art1);
}
