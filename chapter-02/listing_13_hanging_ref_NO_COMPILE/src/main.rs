struct Artwork {
  name: String,
}

fn build_art() -> &Artwork {
  let art = Artwork {
    name: "La Liberté guidant le peuple".to_string(),
  };

  &art
}

fn main() {
  let art = build_art();
}
