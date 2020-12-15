struct Artwork {
  name: String,
}

fn admire_art(art: Artwork) {
  println!("Wow, {} really makes you think.", art.name,);
}

fn main() {
  let art1 = Artwork {
    name: "La Trahison des images".to_string(),
  };
  admire_art(art1);
}
