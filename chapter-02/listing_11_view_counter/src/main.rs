struct Artwork {
  view_count: i32,
  name: String,
}

fn admire_art(art: &mut Artwork) {
  println!("{} people have seen {} today!", art.view_count, art.name);
  art.view_count += 1;
}

fn main() {
  let mut art1 = Artwork {
    view_count: 0,
    name: "".to_string(),
  };

  admire_art(&mut art1);
  admire_art(&mut art1);
}
