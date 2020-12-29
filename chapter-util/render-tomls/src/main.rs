use std::{
  error::Error,
  fs::File,
  io::{Read, Write},
  path::Path,
};

use glob::glob;

fn main() {
  for path in glob("**/Cargo.toml").unwrap() {
    let path = path.unwrap();

    if let Err(e) = process_one_toml(&path) {
      eprintln!("{}: {}", path.display(), e);
      continue;
    }
  }
}

fn process_one_toml(path: &Path) -> Result<(), Box<dyn Error>> {
  let chapter = find_component_starts_with(&path, "chapter")?;
  let listing = find_component_starts_with(&path, "listing")?;

  let mut listing_parts = listing.split('_');

  let crate_name = format!(
    "{}-{}-{}",
    chapter,
    listing_parts.next().unwrap(),
    listing_parts.next().unwrap()
  );

  let mut in_string = String::new();
  let mut f = File::open(path)?;
  f.read_to_string(&mut in_string)?;

  let mut out_string = String::new();

  for line in in_string.lines() {
    if line.starts_with("name =") {
      out_string.push_str(&format!("name = \"{}\"\n", crate_name));
      continue;
    }

    out_string.push_str(line);
    out_string.push('\n');
  }

  let mut f = File::create(path)?;
  f.write_all(out_string.as_bytes())?;

  Ok(())
}

fn find_component_starts_with<'a>(
  path: &'a Path,
  prefix: &str,
) -> Result<&'a str, String> {
  path
    .components()
    .filter_map(|c| {
      let s = c.as_os_str().to_str()?;

      if s.starts_with(prefix) {
        Some(s)
      } else {
        None
      }
    })
    .nth(0)
    .ok_or_else(|| format!("No component found with prefix {}", prefix))
}
